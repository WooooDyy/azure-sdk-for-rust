use crate::{headers, AddAsHeader, Request, HTTPHeaderError};
use http::request::Builder;

#[derive(Debug,Clone,Copy)]
pub struct Quota( u64);

impl Quota{
    pub fn new(quota:u64) ->Self{ Self(quota)}
}

impl AddAsHeader for Quota {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::QUOTA,self.0)
    }

    fn add_as_header2(
        &self, request: &mut Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        request
            .headers_mut()
            .append(headers::QUOTA,http::HeaderValue::from_str(&self.0.to_string())?);
        Ok(())
    }
}