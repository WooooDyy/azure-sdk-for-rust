use std::sync::Arc;
use crate::file::prelude::FileShareClient;
use crate::core::prelude::{StorageClient, StorageAccountClient};
use azure_core::{HttpClient, Request};
use bytes::Bytes;
use http::request::Builder;
use http::Method;
use crate::{Error, Properties};
use url::Url;
use crate::file::file::requests::create_file_builder::CreateFileBuilder;
use crate::file::file::requests::get_file_builder::GetFileBuilder;
use crate::file::file::requests::get_file_properties_builder::GetFilePropertiesBuilder;
use crate::file::file::requests::get_file_metadata_builder::GetFileMetadataBuilder;
use crate::file::file::requests::set_file_properties_builder::SetFilePropertiesBuilder;
use crate::file::file::requests::delete_file_builder::DeleteFileBuilder;
// use azure_core::prelude::FileProperties;

pub trait AsFileClient<CN: Into<String>>{
    fn as_file_client(&self,file_name: CN)-> Arc<FileClient>;
}

impl<CN: Into<String>> AsFileClient<CN> for Arc<FileShareClient>{
    fn as_file_client(&self, file_name: CN) -> Arc<FileClient> {
        FileClient::new(file_name.into(),self.clone())
    }
}
#[derive(Debug,Clone)]
pub struct FileClient{
    file_name: String,
    file_share_client:Arc<FileShareClient>,
}

impl FileClient{
    pub(crate) fn new(file_name: String,file_share_client:Arc<FileShareClient> ) -> Arc<Self>{
        Arc::new(Self{
            file_name,
            file_share_client,
        })
    }
    pub fn file_name(&self) -> &str{ &self.file_name}

    #[allow(dead_code)]
    pub(crate) fn file_share_client(&self) -> &FileShareClient{self.file_share_client.as_ref()}


    pub(crate) fn storage_client(&self) -> &StorageClient{self.file_share_client.storage_client()}

    pub(crate) fn http_client(&self)-> &dyn HttpClient{
        self.storage_client().storage_account_client().http_client()
    }
    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient{
        self.storage_client().storage_account_client()
    }

    pub(crate) fn url_with_dir_path_segments<'a,I>(
        &'a self,
        segments: I,
        dir_path: &'a str
    )-> Result<url::Url,url::ParseError>
        where
            I: IntoIterator<Item = &'a str>,
    {
        self.file_share_client
            .url_with_segments(
                Some(dir_path)
                    .into_iter()
                    .chain(segments)
            )
    }

    pub(crate) fn url_with_segments<'a,I>(
        &'a self,
        segments: I,
        dir_path: &'a str
    )-> Result<url::Url,url::ParseError>
        where
            I: IntoIterator<Item = &'a str>,
    {
        if dir_path=="" {
            self.file_share_client
                .url_with_segments(
                    Some(self.file_name.as_str())
                               .into_iter()
                               .chain(segments),
                )
        }
        else{
            self.url_with_dir_path_segments(
                Some(self.file_name.as_str())
                    .into_iter()
                    .chain(segments),
                dir_path
            )
        }


    }
    // TODO builders
    pub fn create_file(&self)->CreateFileBuilder{
        CreateFileBuilder::new(self)
    }

    pub fn get_file(&self) -> GetFileBuilder{
        GetFileBuilder::new(self)
    }
    pub fn get_file_properties(&self) -> GetFilePropertiesBuilder{
        GetFilePropertiesBuilder::new(self)
    }
    pub fn get_file_metadata(&self) -> GetFileMetadataBuilder{
        GetFileMetadataBuilder::new(self)
    }
    pub fn set_file_properties<'a>(
        &'a self,
        // file_properties: Option<&'a FileProperties<'a, 'a>>,
    ) -> SetFilePropertiesBuilder{
        SetFilePropertiesBuilder::new(self)
    }

    pub fn delete_fle(&self) -> DeleteFileBuilder{
        DeleteFileBuilder::new(self)
    }
    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(http::Request<Bytes>, Url), Error> {
        self.file_share_client
            .prepare_request(url, method, http_header_adder, request_body)
    }

}
