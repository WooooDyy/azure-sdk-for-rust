use crate::core::clients::StorageClient;
use std::sync::Arc;
use azure_core::{HttpClient, Request};
use crate::core::prelude::StorageAccountClient;
use http::Method;
use bytes::Bytes;
use http::request::Builder;

pub trait AsDirectoryClient<CN: Into<String>>{
    fn as_directory_client(&self,directory_name: CN)-> Arc<DirectoryClient>;
}

impl<CN: Into<String>> AsDirectoryClient<CN> for Arc<StorageClient>{
    fn as_directory_client(&self, directory_name: CN) -> Arc<DirectoryClient> {
        DirectoryClient::new(self.clone(),directory_name.into())
    }
}

#[derive(Debug,Clone)]
pub struct DirectoryClient{
    storage_client: Arc<StorageClient>,
    directory_name: String,
}

impl DirectoryClient{
    pub(crate) fn new(storage_client: Arc<StorageClient>, directory_name: String) -> Arc<Self>{
        Arc::new(Self{
            storage_client,
            directory_name,
        })
    }
    pub fn directory_name(&self) -> &str{ &self.directory_name}

    pub(crate) fn storage_client(&self) -> &StorageClient{self.storage_client.as_ref()}

    pub(crate) fn http_client(&self)-> &dyn HttpClient{
        self.storage_client().storage_account_client().http_client()
    }
    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient{
        self.storage_client().storage_account_client()
    }

    pub(crate) fn url_with_segments<'a,I>(
        &'a self,
        segments: I,
    )-> Result<url::Url,url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.storage_client.file_url_with_segments(
            Some(self.directory_name().as_str())
                .into_iter()
                .chain(segments),
        )
    }
    // TODO builders

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