use crate::file::clients::FileShareClient;
use azure_core::prelude::{ClientRequestId, Timeout, LeaseId};

use azure_core::AppendToUrlQuery;
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use crate::file::file_share::responses::GetFileShareMetadataResponse;

#[derive(Debug,Clone)]
pub struct GetFileShareMetadataBuilder<'a>{
    file_share_client: & 'a FileShareClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<& 'a LeaseId>,
}
impl<'a> GetFileShareMetadataBuilder<'a>{
    pub(crate) fn new (file_share_client: &'a FileShareClient) -> Self{
        Self {
            file_share_client,
            client_request_id:None,
            timeout:None,
            lease_id:None,
        }
    }
    setters!{
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout=> Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<GetFileShareMetadataResponse, Box<dyn std::error::Error+Sync+Send>>{
        let mut url = self.file_share_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype","share");
        url.query_pairs_mut().append_pair("comp", "metadata");


        self.timeout.append_to_url_query(&mut url);

        let request = self.file_share_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request | {
                request = add_optional_header(&self.client_request_id,request);
                request = add_optional_header_ref(&self.lease_id,request);
                request
            },
            None,
        )?;

        let response = self
            .file_share_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::OK)
            .await?;
        Ok((response.body(), response.headers()).try_into()?)

    }
}