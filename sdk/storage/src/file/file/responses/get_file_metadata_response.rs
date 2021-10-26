use chrono::{DateTime, FixedOffset};
use azure_core::RequestId;
use azure_core::prelude::Metadata;
use std::convert::TryFrom;
use http::{HeaderMap, header};
use bytes::Bytes;
use azure_core::headers::{REQUEST_ID};
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct GetFileMetadataResponse{
    pub etag: String,
    pub last_modified: DateTime<FixedOffset>,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
    pub metadata:Metadata
}

impl TryFrom<(&Bytes,&HeaderMap)> for GetFileMetadataResponse{
    type Error = crate::Error;
    fn try_from((body,header_map):(&Bytes,&HeaderMap))-> Result<Self,Self::Error>{
        GetFileMetadataResponse::from_response(body,header_map)
    }
}

impl GetFileMetadataResponse{
    pub(crate) fn from_response(
        body:&Bytes,
        headers:&HeaderMap,
    )->Result<GetFileMetadataResponse,crate::Error>{
        let etag = match headers.get(header::ETAG) {
            Some(etag) => etag.to_str()?,
            None => {
                static E: header::HeaderName = header::ETAG;
                return Err(crate::Error::MissingHeaderError(E.as_str().to_owned()));
            }
        };

        let last_modified = match headers.get(header::LAST_MODIFIED) {
            Some(last_modified) => last_modified.to_str()?,
            None => {
                static LM: header::HeaderName = header::LAST_MODIFIED;
                return Err(crate::Error::MissingHeaderError(LM.as_str().to_owned()));
            }
        };
        let last_modified = DateTime::parse_from_rfc2822(last_modified)?;

        let request_id = match headers.get(REQUEST_ID) {
            Some(request_id) => request_id.to_str()?,
            None => return Err(crate::Error::MissingHeaderError(REQUEST_ID.to_owned())),
        };

        let date = match headers.get(header::DATE) {
            Some(date) => date.to_str()?,
            None => {
                static D: header::HeaderName = header::DATE;
                return Err(crate::Error::MissingHeaderError(D.as_str().to_owned()));
            }
        };
        let date = DateTime::parse_from_rfc2822(date)?;

        let tmp:Metadata = headers.into();

        let result = GetFileMetadataResponse{
            etag:etag.to_owned(),
            last_modified,
            request_id:Uuid::parse_str(request_id)?,
            date,
            metadata:headers.into()
        };

        Ok(result)
    }
}
