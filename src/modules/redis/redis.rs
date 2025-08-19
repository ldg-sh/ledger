use redis::Client;

pub struct RedisService {
    pub client: Client,
}

impl RedisService {
    pub fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).expect("Invalid Redis URL");
        RedisService { client }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}