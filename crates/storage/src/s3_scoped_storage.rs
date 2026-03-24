use crate::StorageBackend;
use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use futures::future::join_all;

#[derive(Clone)]
pub struct S3ScopedStorage {
    pub user_id: String,
    pub bucket: String,
    pub client: Client,
}

impl S3ScopedStorage {
    fn scoped_path(&self, path: &str) -> String {
        format!("{}/{}", self.user_id, path.trim_start_matches('/'))
    }
}

#[async_trait]
impl StorageBackend for S3ScopedStorage {
    async fn create_upload(&self, path: &str) -> anyhow::Result<String> {
        let res = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket)
            .key(self.scoped_path(path))
            .send()
            .await?;

        Ok(res.upload_id.unwrap())
    }

    async fn complete_upload(
        &self,
        path: &str,
        upload_id: &str,
        parts: Vec<(u32, String)>,
    ) -> anyhow::Result<()> {
        let completed_parts: Vec<CompletedPart> = parts
            .iter()
            .map(|p| {
                CompletedPart::builder()
                    .part_number(p.clone().0 as i32)
                    .e_tag(p.clone().1)
                    .build()
            })
            .collect();

        let completion_data = CompletedMultipartUpload::builder()
            .set_parts(Some(completed_parts))
            .build();

        self
            .client
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .key(self.scoped_path(&path))
            .upload_id(upload_id)
            .multipart_upload(completion_data)
            .send()
            .await?;

        Ok(())
    }

    async fn delete(&self, path: &str) -> anyhow::Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(self.scoped_path(path))
            .send()
            .await?;

        Ok(())
    }

    async fn delete_many(&self, paths: Vec<String>) -> anyhow::Result<()> {
        let objects = paths
            .into_iter()
            .map(|path| {
                aws_sdk_s3::types::ObjectIdentifier::builder()
                    .key(self.scoped_path(&path))
                    .build()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        self.client
            .delete_objects()
            .bucket(&self.bucket)
            .delete(
                aws_sdk_s3::types::Delete::builder()
                    .set_objects(Some(objects))
                    .build()
                    .unwrap(),
            )
            .send()
            .await?;

        Ok(())
    }

    async fn move_object(&self, src: &str, dest: &str) -> anyhow::Result<()> {
        self.copy_object(src, dest).await?;
        self.delete(src).await?;

        Ok(())
    }

    async fn move_many(&self, moves: Vec<(&str, &str)>) -> anyhow::Result<()> {
        let tasks = moves
            .into_iter()
            .map(|(src, dest)| self.move_object(src, dest));

        let results = join_all(tasks).await;

        for result in results {
            result?;
        }

        Ok(())
    }

    async fn copy_object(&self, src: &str, dest: &str) -> anyhow::Result<()> {
        self.client
            .copy_object()
            .bucket(&self.bucket)
            .copy_source(format!("{}/{}", self.bucket, self.scoped_path(src)))
            .key(self.scoped_path(dest))
            .send()
            .await?;

        Ok(())
    }

    async fn list_objects(&self, prefix: &str) -> anyhow::Result<Vec<String>> {
        let full_prefix = self.scoped_path(prefix);
        let res = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(full_prefix)
            .send()
            .await?;

        let keys = res
            .contents()
            .iter()
            .filter_map(|obj| obj.key())
            .map(|k| k.replacen(&format!("{}/", self.user_id), "", 1))
            .collect();

        Ok(keys)
    }
}
