use crate::file::clients::FileClient;
use azure_core::prelude::{ClientRequestId, Timeout, FType, FilePermission, FilePermissionKey, FileAttributes, FileCreationTime, FileLastWriteTime, ContentLength};
use crate::Properties;
use azure_core::AppendToUrlQuery;
use http::{Method, StatusCode};
use azure_core::headers::{add_optional_header_ref, add_optional_header};
use std::convert::TryInto;
use crate::file::file::responses::set_file_properties_response::SetFilePropertiesResponse;


#[derive(Debug, Clone)]
pub struct SetFilePropertiesBuilder<'a> {
    file_client: &'a FileClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    // file_properties: Option<&'a FileProperties<'a, 'a>>,
    content_length:  Option<ContentLength>,
    dir_path:&'a str,
    f_type:Option<FType<'a>>,
    file_permission: Option<FilePermission<'a>>,
    file_permission_key: Option<FilePermissionKey<'a>>,
    file_attributes: Option<FileAttributes<'a>>,
    file_creation_time:Option<FileCreationTime<'a>>,
    file_last_write_time:Option<FileLastWriteTime<'a>>,
    // TODO setting some other properties
}

impl<'a> SetFilePropertiesBuilder<'a> {
    pub(crate) fn new(
        file_client: &'a FileClient,
        // file_properties: Option<&'a FileProperties<'a, 'a>>,
    ) -> Self {
        Self {
            file_client,
            content_length: None ,
            client_request_id: None,
            timeout: None,
            // file_properties,
            dir_path:"",
            f_type: None,
            file_permission: None,
            file_permission_key: None,
            file_attributes: None,
            file_creation_time: None,
            file_last_write_time: None,

        }
    }

    setters! {
        content_length:ContentLength => Some(content_length),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        dir_path: &'a str => dir_path,
        f_type:FType<'a> => Some(f_type),
        file_permission: FilePermission<'a> => Some(file_permission),
        file_permission_key: FilePermissionKey<'a> => Some(file_permission_key),
        file_attributes: FileAttributes<'a> => Some(file_attributes),
        file_creation_time: FileCreationTime<'a> => Some(file_creation_time),
        file_last_write_time: FileLastWriteTime<'a> => Some(file_last_write_time),
    }

    pub async fn execute(
        &self,
    ) -> Result<SetFilePropertiesResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.file_client.url_with_segments(None,self.dir_path)?;

        url.query_pairs_mut().append_pair("comp", "properties");
        self.timeout.append_to_url_query(&mut url);

        debug!("url = {}", url);

        let request = self.file_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.content_length, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header(&self.f_type, request);
                request = add_optional_header(&self.file_permission,request);
                request = add_optional_header(&self.file_permission_key,request);
                request = add_optional_header(&self.file_attributes,request);
                request = add_optional_header(&self.file_creation_time,request);
                request = add_optional_header(&self.file_last_write_time,request);
                request
            },
            None,
        )?;
        println!("request{:#?}", request);
        debug!("request == {:?}", request);

        let response = self
            .file_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok((&response).try_into()?)
    }
}
