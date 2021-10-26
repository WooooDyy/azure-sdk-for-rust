use azure_core::prelude::{FilePermission, FilePermissionKey, ClientRequestId, Timeout,FileAttributes,FileCreationTime,FileLastWriteTime};
use crate::file::clients::DirectoryClient;
use azure_core::AppendToUrlQuery;
use http::{method::Method, status::StatusCode};
use azure_core::headers::add_optional_header;


#[derive(Debug,Clone)]
pub struct CreateDirectoryBuilder<'a>{
    directory_client: &'a DirectoryClient,
    file_permission: Option<FilePermission<'a>>,
    file_permission_key: Option<FilePermissionKey<'a>>,
    file_attributes: Option<FileAttributes<'a>>,
    file_creation_time:Option<FileCreationTime<'a>>,
    file_last_write_time:Option<FileLastWriteTime<'a>>,
    // TODO x-ms-file-attributes, x-ms-file-creation-time , x-ms-file-last-write-time
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    dir_path:&'a str
}
impl<'a> CreateDirectoryBuilder<'a> {
    pub(crate) fn new(directory_client: &'a DirectoryClient) -> Self{
        Self{
            directory_client,
            file_permission:None,
            file_permission_key:None,
            file_attributes:None,
            file_creation_time:None,
            file_last_write_time:None,
            client_request_id: None,
            timeout: None,
            dir_path:"",
        }
    }
    setters!{
        file_permission: FilePermission<'a> => Some(file_permission),
        file_permission_key: FilePermissionKey<'a> => Some(file_permission_key),
        file_attributes: FileAttributes<'a> => Some(file_attributes),
        file_creation_time: FileCreationTime<'a> => Some(file_creation_time),
        file_last_write_time: FileLastWriteTime<'a> => Some(file_last_write_time),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        dir_path: &'a str => dir_path,
    }
    pub async fn execute(self)-> Result<(),Box<dyn std::error::Error + Sync + Send>>{
        // let mut url = self.directory_client.url_with_segments(None)?;
        let mut url = self.directory_client.url_with_segments(None,self.dir_path)?;

        url.query_pairs_mut().append_pair("restype", "directory");

        self.timeout.append_to_url_query(&mut url);

        let request = self.directory_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request | {
                request = add_optional_header(&self.file_permission,request);
                request = add_optional_header(&self.file_permission_key,request);
                request = add_optional_header(&self.file_attributes,request);
                request = add_optional_header(&self.file_creation_time,request);
                request = add_optional_header(&self.file_last_write_time,request);
                request = add_optional_header(&self.client_request_id,request);
                request
            },
            None,
        )?;

        let _response = self
            .directory_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::CREATED)
            .await?;

        // TODO: Capture and return the response headers
        Ok(())
    }

}