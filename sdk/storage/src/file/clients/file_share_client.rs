use std::sync::Arc;
use crate::core::clients::{StorageAccountClient, StorageClient};
use azure_core::HttpClient;
use http::method::Method;
use http::request::{Builder, Request};
use bytes::Bytes;
use azure_core::prelude::*;
use crate::file::file_share::requests::{CreateBuilder, GetPropertiesBuilder, SetFileShareACLBuilder};
use crate::file_share::requests::{GetFileShareACLBuilder, DeleteFileShareBuilder};


pub trait AsFileShareClient<CN: Into<String>>{
    fn as_file_share_client(&self,file_share_name: CN) -> Arc<FileShareClient>;
}

impl<CN: Into<String>> AsFileShareClient<CN> for Arc<StorageClient>{
    fn as_file_share_client(&self, file_share_name: CN) -> Arc<FileShareClient> {
        FileShareClient::new(self.clone(), file_share_name.into())
    }
}

#[derive(Debug,Clone)]
pub struct FileShareClient{
    storage_client: Arc<StorageClient>,
    file_share_name: String,
}

impl FileShareClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>, file_share_name: String) -> Arc<Self> {
        Arc::new(Self {
            storage_client,
            file_share_name,
        })
    }

    pub fn file_share_name(&self) -> &str { &self.file_share_name}

    pub(crate) fn storage_client(&self) -> &StorageClient{self.storage_client.as_ref()}

    pub(crate) fn http_client(&self) -> &dyn HttpClient{
        self.storage_client.storage_account_client().http_client()
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient{
        self.storage_client.storage_account_client()
    }

    pub(crate) fn url_with_segments<'a,I>(
        &'a self,
        segments: I,
    ) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.storage_client.file_url_with_segments(
            Some(self.file_share_name.as_str())
                .into_iter()
                .chain(segments),
        )
    }

    pub fn create(&self) -> CreateBuilder {
        CreateBuilder::new(self)
    }

    pub fn get_properties(&self) -> GetPropertiesBuilder{
        GetPropertiesBuilder::new(self)
    }
    pub fn get_acl(&self) -> GetFileShareACLBuilder{
        GetFileShareACLBuilder::new(self)
    }
    pub fn set_acl(&self)-> SetFileShareACLBuilder{
        SetFileShareACLBuilder::new(self)
    }
    pub fn delete(&self) -> DeleteFileShareBuilder{
        DeleteFileShareBuilder::new(self)
    }

    // TODO other builders

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}