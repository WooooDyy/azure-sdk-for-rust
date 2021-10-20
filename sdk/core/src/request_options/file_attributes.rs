use crate::{headers,AddAsHeader, Request, HTTPHeaderError};
use http::request::Builder;
#[derive(Debug,Clone,Copy)]
pub struct FileAttributes<'a>(&'a str);


impl<'a,S> From<S> for FileAttributes<'a>
    where
        S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}
impl<'a> AddAsHeader for FileAttributes<'a>{
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::FILE_ATTRIBUTES, self.0)
    }
    fn add_as_header2(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        request.headers_mut().append(
            headers::FILE_ATTRIBUTES,
            http::HeaderValue::from_str(self.0)?,
        );
        Ok(())
    }
}