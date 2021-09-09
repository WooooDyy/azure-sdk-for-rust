
use azure_core::{
    headers::{add_mandatory_header, add_optional_header},
    prelude::*,
};
use http::{method::Method, status::StatusCode};
use crate::file::clients::FileShareClient;
use crate::AccessTier;
use Quota;
use crate::file::file_share::{EnabledProtocols, RootSquash};

#[derive(Debug,Clone)]
pub struct CreateBuilder<'a>{
    file_share_client: &'a FileShareClient,
    metadata: Option<&'a Metadata>,
    share_quota: Option<Quota>,
    access_tier: Option<AccessTier>,
    enabled_protocols: Option<EnabledProtocols>,
    root_squash: Option<RootSquash>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> CreateBuilder<'a> {
    pub(crate) fn new(file_share_client: &'a FileShareClient) -> Self {
        Self{
            file_share_client,
            metadata:None,
            share_quota:None,
            access_tier:None,
            enabled_protocols: None,
            root_squash: None,
            client_request_id: None,
            timeout: None,
        }
    }
    setters! {
        share_quota: Quota => Some(share_quota),
        metadata: &'a Metadata => Some(metadata),
        access_tier: AccessTier => Some(access_tier),
        enabled_protocols: EnabledProtocols => Some(enabled_protocols),
        root_squash: RootSquash => Some(root_squash),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.file_share_client.url_with_segments(None)?;
        url.query_pairs_mut().append_pair("restype", "share");
        self.timeout.append_to_url_query(&mut url);

        let request = self.file_share_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request |{
                // TODO request_options
                request = add_optional_header(&self.share_quota,request);
                request = add_optional_header(&self.access_tier,request);
                request = add_optional_header(&self.enabled_protocols,request);
                request = add_optional_header(&self.root_squash,request);
                request = add_optional_header(&self.metadata, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;
        let _response = self
            .file_share_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::CREATED)
            .await?;
        debug!("response.headers() == {:#?}", _response.headers());
        // TODO: Capture and return the response headers
        Ok(())
    }
}