use crate::core::prelude::*;
use azure_core::headers::add_optional_header;
use azure_core::headers::request_id_from_headers;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::method::Method;
use http::status::StatusCode;

use crate::file::file_share::responses::ListFileShareResponse;
use std::convert::TryInto;
use crate::file_share::incomplete_vector_from_fileshare_response;

#[derive(Debug, Clone)]
pub struct ListFileSharesBuilder<'a> {
    storage_client: &'a StorageClient,
    prefix: Option<Prefix<'a>>,
    next_marker: Option<NextMarker>,
    include_metadata: bool,
    include_snapshots:bool,
    include_deleted: bool,
    max_results: Option<MaxResults>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> ListFileSharesBuilder<'a>{
    pub(crate) fn new(storage_client: &'a StorageClient) -> Self{
        Self {
            storage_client,
            prefix: None,
            next_marker: None,
            include_metadata: false,
            include_snapshots:false,
            include_deleted: false,
            max_results: None,
            client_request_id: None,
            timeout: None,
        }
    }
    setters! {
        prefix: Prefix<'a> => Some(prefix),
        next_marker: NextMarker => Some(next_marker),
        include_metadata: bool => include_metadata,
        include_snapshots: bool => include_snapshots,
        include_deleted: bool => include_deleted,
        max_results: MaxResults => Some(max_results),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),

    }

    pub async fn execute(
        &self,
    ) -> Result<ListFileShareResponse, Box<dyn std::error::Error + Sync + Send>>{
        // build query's url, input some
        let mut url = self
            .storage_client
            .storage_account_client()
            .file_storage_url()
            .clone();
        url.query_pairs_mut().append_pair("comp","list");
        // url parameters
        self.prefix.append_to_url_query(&mut url);
        self.next_marker.append_to_url_query(&mut url);
        if let Some(include) = match (self.include_metadata,self.include_snapshots, self.include_deleted) {
            (true, true,true) => Some("metadata,snapshots,deleted"),
            (true, true, false) => Some("metadata,snapshots"),
            (true, false, true) => Some("metadata,deleted"),
            (false, true, true) => Some("snapshots,deleted"),
            (true, false, false) => Some("metadata"),
            (false, true, false) => Some("snapshots"),
            (false, false, true) => Some("deleted"),
            (false, false, false) => None,
        }{
            url.query_pairs_mut().append_pair("include",include);
        }
        self.max_results.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        debug!("generated url = {}",url);
        // build request header and body
        let(request,_url) = self.storage_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request | {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
                None,
        )?;

        let response = self
            .storage_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request,StatusCode::OK)
            .await?;

        debug!("response == {:?}",response);

        let body = std::str::from_utf8(response.body())?;
        debug!("body == {}",body);

        let incomplete_vector = incomplete_vector_from_fileshare_response(&body)?;
        let request_id = request_id_from_headers(response.headers())?;
        Ok(ListFileShareResponse{
            incomplete_vector,
            request_id,
        })

    }

}
