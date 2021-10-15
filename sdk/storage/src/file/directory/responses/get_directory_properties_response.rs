use crate::file::directory::Directory;
use azure_core::RequestId;
use chrono::{DateTime, FixedOffset};
use std::convert::TryFrom;
use http::{HeaderMap, header};
use azure_core::headers::REQUEST_ID;
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct GetDirectoryPropertiesResponse{
    pub directory: Directory,
    pub request_id:RequestId,
    pub date: DateTime<FixedOffset>,
}

impl TryFrom<(&str,&HeaderMap)> for GetDirectoryPropertiesResponse{
    type Error = crate::Error;
    fn try_from((body,header_map): (&str, &HeaderMap)) -> Result<Self, Self::Error> {
        GetDirectoryPropertiesResponse::from_response(body,header_map)
    }
}
impl GetDirectoryPropertiesResponse{
    pub(crate) fn from_response(
        directory_name: &str,
        headers: &HeaderMap,
    ) -> Result<GetDirectoryPropertiesResponse,crate::Error>{
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

        let directory = Directory::from_response(directory_name,headers)?;
        Ok(
            GetDirectoryPropertiesResponse{
                directory,
                request_id,
                date,
            }
        )
    }
}
