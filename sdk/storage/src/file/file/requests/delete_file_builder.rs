use crate::file::clients::FileClient;
use azure_core::prelude::{ClientRequestId, Timeout, FType, FilePermission, FilePermissionKey, FileAttributes, FileCreationTime, FileLastWriteTime};
use http::{Method, StatusCode};
use azure_core::headers::add_optional_header;

#[derive(Debug,Clone)]
pub struct DeleteFileBuilder<'a>{
    file_client: &'a FileClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    dir_path:&'a str,
}

impl<'a> DeleteFileBuilder<'a>{
    pub(crate) fn new(file_client: &'a FileClient)->Self{
        DeleteFileBuilder{
            file_client,
            client_request_id: None,
            timeout: None,
            dir_path:"",
        }
    }
    setters!{
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        dir_path: &'a str => dir_path,
    }
    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.file_client.url_with_segments(None,self.dir_path)?;

        url.query_pairs_mut().append_pair("restype","file");

        let request = self.file_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let _response =  self
            .file_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::ACCEPTED)
            .await?;
        // TODO: Capture and return the response headers
        Ok(())

    }
}
