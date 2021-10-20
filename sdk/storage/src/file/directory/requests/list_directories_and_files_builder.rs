use crate::core::prelude::StorageClient;
use azure_core::prelude::{Prefix, NextMarker, MaxResults, ClientRequestId, Timeout};
use crate::file::directory::responses::ListDirectoriesAndFilesResponse;
use crate::file::clients::DirectoryClient;
use azure_core::AppendToUrlQuery;
use azure_core::headers::{add_optional_header, request_id_from_headers};
use http::{Method, StatusCode};
use crate::file::directory::Directory;


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

    pub async fn execute(
        &self,
    ) -> Result<ListDirectoriesAndFilesResponse, Box<dyn std::error::Error + Sync + Send>>
    {
        let mut url = self.directory_client.url_with_segments(None)?;
        url.query_pairs_mut().append_pair("restype","directory");
        url.query_pairs_mut().append_pair("comp","list");

        self.prefix.append_to_url_query(&mut url);
        self.next_marker.append_to_url_query(&mut url);
        self.max_results.append_to_url_query(&mut url);

        let mut include : Vec<&str> = Vec::new();
        if self.include_timestamps {
            let mut include_timstamps_str = "Timestamps";
            include.push(include_timstamps_str);
        }
        if self.include_etag {
            let mut include_etag_str="ETag";
            include.push(include_etag_str);
        }
        if self.include_attributes {
            let mut include_attributes_str="Attributes";
            include.push(include_attributes_str);
        }
        if self.include_permission_key{
            let mut include_permission_key_str="PermissionKey";
            include.push(include_permission_key_str);
        }
        let include: String = include.join(",");
        let include: String = ["{",include.as_str(),"}"].join("");

        // TODO URI parameter "include" for 2020-10-02 and newer version
        // url.query_pairs_mut().append_pair("include",include.as_str());

        self.timeout.append_to_url_query(&mut url);
        debug!("generated url = {}",url);
        // build request header and body
        let(request,_url) = self.directory_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request |{
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;
        let response = self
            .directory_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request,StatusCode::OK)
            .await?;
        debug!("response == {:?}",response);

        let body = std::str::from_utf8(response.body())?;
        debug!("body == {}",body);

        let incomplete_vector =Directory::incomplete_vector_from_directory_response(&body)?;

        let request_id = request_id_from_headers(response.headers())?;
        Ok(
            ListDirectoriesAndFilesResponse{
                incomplete_vector,
                request_id,
            }
        )

    }

}