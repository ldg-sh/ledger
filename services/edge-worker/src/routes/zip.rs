use crate::{authenticate, AppState};
use async_zip::base::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use common::types::file::explode::{ZipRequest, ExplodeResponse};
use serde_json::Value;
use std::sync::Arc;
use web_sys::TransformStream;
use worker::*;

pub async fn handle_zip(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = ctx.data;
    let body = req.text().await?;

    let ts = TransformStream::new().map_err(|_| Error::from("TS Fail"))?;
    let writable = ts.writable();

    let req_body = match serde_json::from_str::<ZipRequest>(&body) {
        Ok(data) => data,
        Err(_) => {
            return Response::error("Invalid request body", 400);
        }
    };

    let items = state
        .config
        .make_internal_request::<_, Value>("/internal/file/explode", &user, Method::Post, &req_body)
        .await?;

    if items.0 != 200 {
        return Ok(Response::from_json(&items.1)?.with_status(items.0));
    }

    let items: ExplodeResponse = serde_json::from_value(items.1)?;


    wasm_bindgen_futures::spawn_local(async move {
        let mut zip_buffer = Vec::new();
        let mut zip = ZipFileWriter::new(&mut zip_buffer);

        for item in items.items.iter() {
            let mut s3_resp = Fetch::Url(item.presign_url.parse().unwrap()).send().await.unwrap();
            if s3_resp.status_code() == 200 {
                let bytes = s3_resp.bytes().await.unwrap();
                let entry = ZipEntryBuilder::new(item.virtual_path.clone().into(), Compression::Stored);
                zip.write_entry_whole(entry, &bytes).await.unwrap();
            }
        }
        zip.close().await.unwrap();

        let writer = writable.get_writer().unwrap();
        let chunk = js_sys::Uint8Array::from(&zip_buffer[..]);
        let _ = wasm_bindgen_futures::JsFuture::from(writer.write_with_chunk(&chunk)).await;
        let _ = wasm_bindgen_futures::JsFuture::from(writer.close()).await;
    });

    let readable = ts.readable();

    let raw_resp = web_sys::Response::new_with_opt_readable_stream(Some(&readable))
        .map_err(|_| Error::from("Failed to construct Response with stream"))?;

    let response = Response::from(raw_resp);

    let headers = Headers::new();
    headers.set("Content-Type", "application/zip")?;
    headers.set("Content-Disposition", "attachment; filename=\"archive.zip\"")?;

    Ok(response.with_headers(headers))
}