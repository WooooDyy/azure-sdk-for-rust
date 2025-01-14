use crate::core::clients::StorageClient;
use std::sync::Arc;
use azure_core::{HttpClient};
use crate::core::prelude::StorageAccountClient;
use http::method::Method;
use bytes::Bytes;
use azure_core::prelude::*;
use http::request::Builder;
use http::Request;
use crate::file::directory::requests::{CreateDirectoryBuilder};
use crate::file::clients::FileShareClient;
use crate::directory::requests::get_directory_properties_builder::GetDirectoryPropertiesBuilder;
use crate::directory::requests::delete_directory_builder::DeleteDirectoryBuilder;
use crate::directory::requests::get_directory_metadata_builder::GetDirectoryMetadataBuilder;
use crate::directory::responses::ListDirectoriesAndFilesResponse;
use crate::directory::requests::list_directories_and_files_builder::ListDirectoriesAndFilesBuilder;

pub trait AsDirectoryClient<CN: Into<String>>{
    fn as_directory_client(&self,directory_name: CN)-> Arc<DirectoryClient>;
}

impl<CN: Into<String>> AsDirectoryClient<CN> for Arc<FileShareClient>{
    fn as_directory_client(&self, directory_name: CN) -> Arc<DirectoryClient> {
        DirectoryClient::new(directory_name.into(),self.clone())
    }
}

#[derive(Debug,Clone)]
pub struct DirectoryClient{
    directory_name: String,
    file_share_client:Arc<FileShareClient>,
}

impl DirectoryClient{
    pub(crate) fn new(directory_name: String,file_share_client:Arc<FileShareClient> ) -> Arc<Self>{
        Arc::new(Self{
            directory_name,
            file_share_client,
        })
    }
    pub fn directory_name(&self) -> &str{ &self.directory_name}

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
                .url_with_segments(Some(
                    self.directory_name.as_str()
                )
                    .into_iter()
                    .chain(segments),
                )
        }
        else{
            self.url_with_dir_path_segments(
            Some(self.directory_name.as_str())
                .into_iter()
                .chain(segments),
            dir_path
        )
        }


    }

    pub fn create_directory(&self) ->CreateDirectoryBuilder{
        CreateDirectoryBuilder::new(self)
    }

    pub fn get_directory_properties(&self) -> GetDirectoryPropertiesBuilder{
        GetDirectoryPropertiesBuilder::new(self)
    }

    pub fn delete_directory(&self) -> DeleteDirectoryBuilder{
        DeleteDirectoryBuilder::new(self)
    }

    pub fn get_directory_metadata(&self)->GetDirectoryMetadataBuilder{
        GetDirectoryMetadataBuilder::new(self)
    }
    pub fn list_directories_and_files(&self) -> ListDirectoriesAndFilesBuilder{
        ListDirectoriesAndFilesBuilder::new(self)
    }
    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        self.file_share_client
            .prepare_request(url, method, http_header_adder, request_body)
    }

}
