use crate::file::file::File;
use azure_core::RequestId;
use chrono::{DateTime, FixedOffset};
use std::convert::TryFrom;
use http::{HeaderMap, header};
use azure_core::headers::REQUEST_ID;
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct GetFilePropertiesResponse{
    pub file: File,
    pub request_id:RequestId,
    pub date: DateTime<FixedOffset>,
}

impl TryFrom<(&str,&HeaderMap)> for GetFilePropertiesResponse{
    type Error = crate::Error;
    fn try_from((body,header_map): (&str, &HeaderMap)) -> Result<Self, Self::Error> {
        GetFilePropertiesResponse::from_response(body,header_map)
    }
}
impl GetFilePropertiesResponse{
    pub(crate) fn from_response(
        file_name: &str,
        headers: &HeaderMap,
    ) -> Result<GetFilePropertiesResponse,crate::Error>{
        let request_id = match headers.get(REQUEST_ID){
            Some(requeset_id) => Uuid::parse_str(requeset_id.to_str()?)?,
            None => return Err(crate::Error::MissingHeaderError(REQUEST_ID.to_owned())) ,
        };

        let date = match headers.get(header::DATE) {
            Some(date) => DateTime::parse_from_rfc2822(date.to_str()?)?,
            None =>{
                static D: header::HeaderName = header::DATE;
                return Err(crate::Error::MissingHeaderError(D.as_str().to_owned())) ;
            }
        };

        let file = File::from_response(file_name,headers)?;
        Ok(
            GetFilePropertiesResponse{
                file,
                request_id,
                date,
            }
        )
    }
}
