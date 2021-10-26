use crate::file::clients::FileClient;
use azure_core::prelude::{FilePermission, FilePermissionKey, FileCreationTime, FileLastWriteTime, ClientRequestId, Timeout, FileAttributes,ContentLength,FType};
use azure_core::AppendToUrlQuery;
use azure_core::headers::{add_mandatory_header, add_optional_header};
use http::{Method, StatusCode};

#[derive(Debug,Clone)]
pub struct CreateFileBuilder<'a>{
    file_client: &'a FileClient,
    content_length:  Option<ContentLength>,
    f_type:Option<FType<'a>>,
    file_permission: Option<FilePermission<'a>>,
    file_permission_key: Option<FilePermissionKey<'a>>,
    file_attributes: Option<FileAttributes<'a>>,
    file_creation_time:Option<FileCreationTime<'a>>,
    file_last_write_time:Option<FileLastWriteTime<'a>>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    dir_path:&'a str
}
impl<'a> CreateFileBuilder<'a>{
    pub(crate) fn new(file_client: &'a FileClient) -> Self {
        Self {
            file_client,
            content_length: None ,
            f_type: None,
            file_permission: None,
            file_permission_key: None,
            file_attributes: None,
            file_creation_time: None,
            file_last_write_time: None,
            client_request_id: None,
            timeout: None,
            dir_path: "",
        }
    }
    setters!{
        content_length:ContentLength => Some(content_length),
        f_type:FType<'a> => Some(f_type),
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
        let mut url = self.file_client.url_with_segments(None,self.dir_path)?;

        url.query_pairs_mut().append_pair("restype", "file");

        self.timeout.append_to_url_query(&mut url);

        let request = self.file_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request | {
                request = add_optional_header(&self.content_length, request);
                request = add_optional_header(&self.f_type, request);
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
            .file_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::CREATED)
            .await?;

        // TODO: Capture and return the response headers
        Ok(())
    }


}