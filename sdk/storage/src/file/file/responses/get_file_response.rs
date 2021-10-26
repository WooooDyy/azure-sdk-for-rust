use crate::file::file::File;
use azure_core::RequestId;
use chrono::{FixedOffset, DateTime};
use std::convert::TryFrom;
use http::{HeaderMap, header,Response};
use azure_core::headers::REQUEST_ID;
use uuid::Uuid;
use bytes::Bytes;
use azure_core::headers::{date_from_headers, request_id_from_headers};

#[derive(Debug,Clone)]
pub struct  GetFileResponse{
    pub file: File,
    pub request_id:RequestId,
    pub date: DateTime<FixedOffset>,
    pub data: Bytes,
}

impl TryFrom<(&str, Response<Bytes>)> for GetFileResponse {
    type Error = crate::Error;
    fn try_from((file_name, response): (&str, Response<Bytes>)) -> Result<Self, Self::Error> {
        GetFileResponse::from_response(file_name,response)
    }
}

impl GetFileResponse{
    pub(crate) fn from_response(
        file_name: &str,
        response: Response<Bytes>
    ) -> Result<GetFileResponse,crate::Error>{

        let headers = response.headers();

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
            GetFileResponse{
                file,
                request_id,
                date,
                data:response.into_body()
            }
        )
    }
}