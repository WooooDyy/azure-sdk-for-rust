#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use std::sync::Arc;
use azure_storage::core::prelude::{StorageAccountClient, AsStorageClient};
use azure_storage::{StoredAccessPolicy, StoredAccessPolicyList};
use azure_storage::file::clients::directory_client::AsDirectoryClient;
use chrono::{FixedOffset, Utc};
use std::ops::Add;
use azure_core::prelude::*;
use azure_storage::file::clients::AsFileShareClient;

//  cargo test -p azure_storage --test directory --features test_e2e  -- --nocapture
#[tokio::test]
async fn create_and_delete_directory(){
    let file_share_name: &'static str = "test-file-share";
    let directory_name: &'static str = "directory-test1";
    let storage_client = initialize().as_storage_client();
    let file_share = storage_client.as_file_share_client(file_share_name);
    let directory = file_share.as_directory_client(directory_name);

    let file_permission = FilePermission::from("inherit");
    let file_arrtibutes = FileAttributes::from("None");
    let file_creation_time = FileCreationTime::from("now");
    let file_last_write_time = FileLastWriteTime::from("now");

    let create_response = directory
        .create_directory()
        .file_permission(file_permission)
        .file_attributes(file_arrtibutes)
        .file_creation_time(file_creation_time)
        .file_last_write_time(file_last_write_time)
        .execute()
        .await
        .unwrap();
    println!("{:#?}", create_response);

    let metadata = directory
        .get_directory_metadata()
        .execute()
        .await
        .unwrap();
    println!("\nget metadata =={:?}", metadata);

    let properties = directory
        .get_directory_properties()
        .execute()
        .await
        .unwrap();
    println!("\nget properties =={:?}", properties);


    let delete_response = directory
        .delete_directory()
        .execute()
        .await
        .unwrap();
    println!("\ndelete response =={:?}", delete_response);


}


#[tokio::test]
async fn list_directories_and_files(){
    let storage_client = initialize().as_storage_client();
    let file_share_client = storage_client.as_file_share_client("test-file-share");

    let directory_client = file_share_client.as_directory_client("test-directory-1");

    let res = directory_client
        .list_directories_and_files()
        .execute()
        .await
        .unwrap();
    println!("\nlist_directories_files() =={:?}", res);

}

fn initialize() -> Arc<StorageAccountClient>{
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let http_client = new_http_client();

    StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)

}