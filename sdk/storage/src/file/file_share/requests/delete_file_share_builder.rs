
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;
use crate::file::prelude::FileShareClient;

#[derive(Debug,Clone)]
pub struct DeleteFileShareBuilder<'a>{
    file_share_client: &'a FileShareClient,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> DeleteFileShareBuilder<'a>{
    pub(crate) fn new(file_share_client: &'a FileShareClient)->Self{
        DeleteFileShareBuilder{
            file_share_client,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }
    setters!{
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }
    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.file_share_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "share");

        let request = self.file_share_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            None,
        )?;

        let _response = self
            .file_share_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::ACCEPTED)
            .await?;
        // TODO: Capture and return the response headers
        Ok(())
    }
}