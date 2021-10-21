use crate::{headers, Request, HTTPHeaderError};
use crate::AddAsHeader;
use http::request::Builder;
#[derive(Debug, Copy, Clone)]
pub struct ContentLength(u64);

impl ContentLength{
    pub fn new(content_length:u64) -> Self{Self(content_length)}
}


impl AddAsHeader for ContentLength{
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::FILE_CONTENT_LENGTH,self.0)
    }
    fn add_as_header2(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        request.headers_mut().append(
            headers::FILE_CONTENT_LENGTH,
            http::HeaderValue::from(self.0),
        );
        Ok(())
    }
}