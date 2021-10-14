use crate::core::prelude::StorageClient;
use azure_core::prelude::{Prefix, NextMarker, MaxResults, ClientRequestId, Timeout};
use crate::file::directory::responses::ListDirectoriesAndFilesResponse;
use crate::file::clients::DirectoryClient;

#[derive(Debug, Clone)]
pub struct ListDirectoriesAndFilesBuilder<'a>{
    directory_client: &'a DirectoryClient,
    prefix: Option<Prefix<'a>>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    include_timestamps:bool,
    include_etag:bool,
    include_attributes:bool,
    include_permission_key:bool,

    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> ListDirectoriesAndFilesBuilder<'a>{
    pub(crate) fn new(directory_client: &'a DirectoryClient) -> Self{
        Self{
            directory_client,
            prefix:None,
            next_marker: None,
            max_results: None,
            include_timestamps:false,
            include_etag:false,
            include_attributes:false,
            include_permission_key:false,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        prefix: Prefix<'a> => Some(prefix),
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        include_timestamps:bool => include_timestamps,
        include_etag: bool => include_etag,
        include_attributes: bool => include_attributes,
        include_permission_key: bool => include_permission_key,
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    // TODO execute
    // pub async fn execute(
    //     &self,
    // ) -> Result<ListDirectoriesAndFilesResponse, Box<dyn std::error::Error + Sync + Send>>
    // {
    //
    // }
}