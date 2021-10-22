use crate::file::clients::FileClient;
use azure_core::prelude::{Timeout, ClientRequestId};
use crate::file::file::responses::get_file_properties_response::GetFilePropertiesResponse;
use azure_core::AppendToUrlQuery;
use http::{Method, StatusCode};
use azure_core::headers::add_optional_header;
use std::convert::TryInto;

#[derive(Debug,Clone)]
pub struct GetFilePropertiesBuilder<'a>{
    file_client: & 'a FileClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    dir_path:&'a str
}

impl<'a> GetFilePropertiesBuilder<'a> {
    pub(crate) fn new (file_client: &'a FileClient) ->Self{
        Self{
            file_client,
            client_request_id:None,
            timeout:None,
            dir_path:""
        }
    }
    setters!{
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout=> Some(timeout),
        dir_path: &'a str => dir_path,
    }

    pub async fn execute(
        &self,
    ) -> Result<GetFilePropertiesResponse,Box<dyn std::error::Error+Sync+Send>>{
        let mut url = self.file_client.url_with_segments(None,self.dir_path)?;

        url.query_pairs_mut().append_pair("restype","file");

        self.timeout.append_to_url_query(&mut url);

        let request = self.file_client.prepare_request(
            url.as_str(),
            &Method::HEAD,
            &|mut request | {
                request = add_optional_header(&self.client_request_id,request);
                request
            },
            None,
        )?;

        let response = self
            .file_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::OK)
            .await?;
        Ok((self.file_client.file_name(),response.headers()).try_into()?)
    }
}