use crate::modules::{
    postgres::postgres_service::PostgresService, redis::redis_service::RedisService,
    s3::s3_service::S3Service,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub s3_service: Arc<S3Service>,
    pub postgres_service: Arc<PostgresService>,
    pub redis_service: Arc<RedisService>,
}

impl AppContext {
    pub fn new(
        s3_service: Arc<S3Service>,
        postgres_service: Arc<PostgresService>,
        redis_service: Arc<RedisService>,
    ) -> Self {
        Self {
            s3_service,
            postgres_service,
            redis_service,
        }
    }
}
