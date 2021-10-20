use crate::container::public_access_from_header;
use azure_core::headers::{add_mandatory_header, add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::status::StatusCode;
use crate::file::clients::FileShareClient;
use crate::StoredAccessPolicyList;

#[derive(Debug, Clone)]
pub struct SetFileShareACLBuilder<'a>{
    file_share_client: &'a FileShareClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    stored_access_policy_list: Option<&'a StoredAccessPolicyList>,

}
impl<'a> SetFileShareACLBuilder<'a>{
    pub(crate) fn new(file_share_client:&'a FileShareClient)->Self{
        Self{
            file_share_client,
            client_request_id:None,
            timeout:None,
            lease_id:None,
            stored_access_policy_list: None,

        }
    }
    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        stored_access_policy_list: &'a StoredAccessPolicyList => Some(stored_access_policy_list),

    }

    pub async fn execute(&self)->Result<(), Box<dyn std::error::Error + Sync + Send>>{
        let mut url = self.file_share_client.url_with_segments(None)?;
        url.query_pairs_mut().append_pair("restype", "share");
        url.query_pairs_mut().append_pair("comp", "acl");

        self.timeout.append_to_url_query(&mut url);

        let xml = self.stored_access_policy_list.map(|xml| xml.to_xml());

        let request = self.file_share_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            xml.map(Bytes::from),
        )?;

        let response = self
            .file_share_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0,StatusCode::OK)
            .await?;
        Ok(())
    }
}