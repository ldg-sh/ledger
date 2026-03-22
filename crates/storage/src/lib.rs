pub mod s3_scoped_storage;
pub mod s3_manager;

use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait StorageBackend {
    async fn delete(&self, path: &str) -> Result<()>;
    async fn delete_many(&self, paths: Vec<&str>) -> Result<()>;
    async fn move_object(&self, src: &str, dest: &str) -> Result<()>;
    async fn move_many(&self, moves: Vec<(&str, &str)>) -> Result<()>;
    async fn copy_object(&self, src: &str, dest: &str) -> Result<()>;
    async fn list_objects(&self, prefix: &str) -> Result<Vec<String>>;
}