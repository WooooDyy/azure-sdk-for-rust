use crate::file::clients::DirectoryClient;
use azure_core::prelude::{ClientRequestId, Timeout};
use crate::directory::responses::get_directory_properties_response::GetDirectoryPropertiesResponse;
use azure_core::AppendToUrlQuery;
use http::{StatusCode, Method};
use azure_core::headers::add_optional_header;
use std::convert::TryInto;

#[derive(Debug,Clone)]
pub struct GetDirectoryPropertiesBuilder<'a>{
    directory_client: & 'a DirectoryClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> GetDirectoryPropertiesBuilder<'a> {
    pub(crate) fn new (directory_client: &'a DirectoryClient) ->Self{
        Self{
            directory_client,
            client_request_id:None,
            timeout:None,
        }
    }
    setters!{
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout=> Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<GetDirectoryPropertiesResponse,Box<dyn std::error::Error+Sync+Send>>{
        let mut url = self.directory_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype","directory");

        self.timeout.append_to_url_query(&mut url);

        let request = self.directory_client.prepare_request(
            url.as_str(),
            &Method::HEAD,
            &|mut request | {
                request = add_optional_header(&self.client_request_id,request);
                request
            },
            None,
        )?;

        let response = self
            .directory_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::OK)
            .await?;
        Ok((self.directory_client.directory_name(),response.headers()).try_into()?)
    }
}