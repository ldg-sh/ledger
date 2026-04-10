use crate::{AppState, authenticate};
use async_zip::base::write::ZipFileWriter;
use async_zip::{Compression, ZipDateTimeBuilder, ZipEntryBuilder};
use common::types::file::explode::{ExplodeResponse, ZipRequest};
use futures_util::StreamExt;
use futures_util::io::{AsyncWrite, AsyncWriteExt};
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use web_sys::{TransformStream, WritableStreamDefaultWriter};
use worker::*;

struct WebStreamWriter {
    writer: WritableStreamDefaultWriter,
    fut: Option<wasm_bindgen_futures::JsFuture>,
}

impl WebStreamWriter {
    fn new(writer: WritableStreamDefaultWriter) -> Self {
        Self { writer, fut: None }
    }
}

impl AsyncWrite for WebStreamWriter {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        if let Some(mut fut) = self.fut.take() {
            match Pin::new(&mut fut).poll(cx) {
                Poll::Ready(Ok(_)) => {}
                Poll::Ready(Err(e)) => {
                    return Poll::Ready(Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{:?}", e),
                    )));
                }
                Poll::Pending => {
                    self.fut = Some(fut);
                    return Poll::Pending;
                }
            }
        }

        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }

        let uint8_array = js_sys::Uint8Array::from(buf);
        let promise = self.writer.write_with_chunk(&uint8_array);
        let mut fut = wasm_bindgen_futures::JsFuture::from(promise);

        match Pin::new(&mut fut).poll(cx) {
            Poll::Ready(Ok(_)) => Poll::Ready(Ok(buf.len())),
            Poll::Ready(Err(e)) => Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            ))),
            Poll::Pending => {
                self.fut = Some(fut);
                Poll::Ready(Ok(buf.len()))
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        if let Some(mut fut) = self.fut.take() {
            match Pin::new(&mut fut).poll(cx) {
                Poll::Ready(_) => {}
                Poll::Pending => {
                    self.fut = Some(fut);
                    return Poll::Pending;
                }
            }
        }

        let promise = self.writer.close();
        let mut fut = wasm_bindgen_futures::JsFuture::from(promise);
        match Pin::new(&mut fut).poll(cx) {
            Poll::Ready(_) => Poll::Ready(Ok(())),
            Poll::Pending => {
                self.fut = Some(fut);
                Poll::Pending
            }
        }
    }
}

pub async fn handle_zip(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = ctx.data;
    let body = req.text().await?;

    let ts = TransformStream::new().map_err(|_| Error::from("TS Fail"))?;
    let writable = ts.writable();
    let readable = ts.readable();

    let req_body: ZipRequest =
        serde_json::from_str(&body).map_err(|_| Error::from("Invalid JSON"))?;

    let items_raw = state
        .config
        .make_internal_request::<_, Value>("/internal/file/explode", &user, Method::Post, &req_body)
        .await?;

    if items_raw.0 != 200 {
        return Ok(Response::from_json(&items_raw.1)?.with_status(items_raw.0));
    }

    let items: ExplodeResponse = serde_json::from_value(items_raw.1)?;

    let items_clone = items.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let writer = writable.get_writer().unwrap();
        let web_writer = WebStreamWriter::new(writer);
        let mut zip = ZipFileWriter::new(web_writer);

        for item in items_clone.items {
            if let Ok(mut s3_resp) = Fetch::Url(item.presign_url.parse().unwrap()).send().await {
                if s3_resp.status_code() == 200 {
                    let entry = ZipEntryBuilder::new(item.virtual_path.into(), Compression::Stored);

                    let unix_time = item.created_at.timestamp() as u64;

                    let dt = ZipDateTimeBuilder::new()
                        .year((unix_time / 31_536_000 + 1970) as i32)
                        .month(((unix_time % 31_536_000) / 2_592_000 + 1) as u32)
                        .day(((unix_time % 2_592_000) / 86_400 + 1) as u32)
                        .hour(((unix_time % 86_400) / 3600) as u32)
                        .minute(((unix_time % 3600) / 60) as u32)
                        .second((unix_time % 60) as u32)
                        .build();

                    if let Ok(mut entry_writer) = zip
                        .write_entry_stream(entry.last_modification_date(dt))
                        .await
                    {
                        if let Ok(mut stream) = s3_resp.stream() {
                            while let Some(Ok(chunk)) = stream.next().await {
                                let _ = entry_writer.write_all(&chunk.to_vec()).await;
                            }
                        }
                        let _ = entry_writer.close().await;
                    }
                }
            }
        }
        let _ = zip.close().await;
    });

    let raw_resp = web_sys::Response::new_with_opt_readable_stream(Some(&readable))
        .map_err(|_| Error::from("Response Construct Fail"))?;

    let total_size: i64 = items.items.iter().map(|i| i.size).sum();

    let headers = Headers::new();
    headers.set("Content-Length", &total_size.to_string())?;
    headers.set("Content-Type", "application/zip")?;
    headers.set(
        "Content-Disposition",
        "attachment; filename=\"archive.zip\"",
    )?;

    headers.set("X-Archive-Size", &total_size.to_string())?;

    headers.set(
        "Access-Control-Expose-Headers",
        "Content-Length, Content-Disposition, X-Archive-Size",
    )?;

    Ok(Response::from(raw_resp).with_headers(headers))
}
