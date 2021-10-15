use crate::file::clients::DirectoryClient;
use azure_core::prelude::{ClientRequestId, Timeout};
use http::{StatusCode, Method};
use azure_core::headers::add_optional_header;

#[derive(Debug,Clone)]
pub struct DeleteDirectoryBuilder<'a>{
    directory_client: &'a DirectoryClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> DeleteDirectoryBuilder<'a>{
    pub(crate) fn new(directory_client: &'a DirectoryClient)->Self{
        DeleteDirectoryBuilder{
            directory_client,
            client_request_id: None,
            timeout: None,
        }
    }
    setters!{
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }
    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.directory_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype","directory");

        let request = self.directory_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let _response =  self
            .directory_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::ACCEPTED)
            .await?;
        // TODO: Capture and return the response headers
        Ok(())

    }
}
