use azure_core::headers::{CommonStorageResponseHeaders, etag_from_headers, last_modified_from_headers};
use azure_core::prelude::Etag;
use chrono::{DateTime, Utc};
use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
use http::Response;

#[derive(Debug, Clone)]
pub struct SetFilePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: DateTime<Utc>,
}

impl TryFrom<&Response<Bytes>> for SetFilePropertiesResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        trace!("body == {}", std::str::from_utf8(response.body())?);
        trace!("headers == {:?}", response.headers());

        Ok(SetFilePropertiesResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: Etag::from(etag_from_headers(response.headers())?),
            last_modified: last_modified_from_headers(response.headers())?,
        })
    }
}
