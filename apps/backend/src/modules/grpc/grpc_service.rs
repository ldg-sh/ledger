use crate::ledger::authentication_client::AuthenticationClient;
use tonic::Request;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct GrpcService {
    channel: Channel,
    auth_metadata: MetadataValue<Ascii>,
}

impl GrpcService {
    pub fn new(channel: Channel, auth_key: &str) -> Result<Self, InvalidMetadataValue> {
        let auth_metadata = auth_key.parse::<MetadataValue<Ascii>>()?;

        Ok(Self {
            channel,
            auth_metadata,
        })
    }

    pub(crate) fn authentication_client(&self) -> AuthenticationClient<Channel> {
        AuthenticationClient::new(self.channel.clone())
    }

    pub(crate) fn request_with_auth<T>(&self, message: T) -> Request<T> {
        let mut request = Request::new(message);
        request
            .metadata_mut()
            .insert("authorization", self.auth_metadata.clone());
        request
    }
}
