use crate::file::clients::FileClient;
use azure_core::prelude::{FilePermission, FilePermissionKey, FileCreationTime, FileLastWriteTime, ClientRequestId, Timeout, FileAttributes, ContentLength, FType, Write, Range};
use azure_core::AppendToUrlQuery;
use azure_core::headers::{add_mandatory_header, add_optional_header};
use http::{Method, StatusCode};
use bytes::Bytes;

#[derive(Debug,Clone)]
pub struct PutRangeBuilder<'a>{
    file_client: &'a FileClient,
    content_length:  Option<ContentLength>,
    write: Option<Write<'a>>,
    range:Option<Range>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    dir_path:&'a str,
    data:Bytes ,
}
impl<'a> PutRangeBuilder<'a>{
    pub(crate) fn new(file_client: &'a FileClient,data: impl Into<Bytes>,) -> Self {
        Self {
            file_client,
            content_length: None ,
            write:None,
            range:None,
            client_request_id: None,
            timeout: None,
            dir_path: "",
            data: data.into(),

        }
    }
    setters!{
        content_length:ContentLength => Some(content_length),
        write: Write<'a> => Some(write),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        range: Range => Some(range),
        timeout: Timeout => Some(timeout),
        dir_path: &'a str => dir_path,
        data: Bytes => data,
    }

    pub async fn execute(self)-> Result<(),Box<dyn std::error::Error + Sync + Send>>{
        // let mut url = self.directory_client.url_with_segments(None)?;
        let mut url = self.file_client.url_with_segments(None,self.dir_path)?;

        url.query_pairs_mut().append_pair("comp", "range");

        self.timeout.append_to_url_query(&mut url);


        let request = self.file_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request | {
                request = add_optional_header(&self.content_length, request);
                request = add_optional_header(&self.write,request);
                request = add_optional_header(&self.client_request_id,request);
                request = add_optional_header(&self.range, request);
                request
            },
            Some(self.data.clone()),
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