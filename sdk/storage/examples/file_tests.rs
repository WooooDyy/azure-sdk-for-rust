use azure_storage::core::prelude::{StorageAccountClient, AsStorageClient};
use std::sync::Arc;
use azure_core::prelude::{ContentLength, Range,Write,FilePermission,FileAttributes,FileCreationTime,FileLastWriteTime,FType
};
use std::error::Error;
use azure_storage::file::prelude::AsFileShareClient;
use azure_storage::file::clients::file_client::AsFileClient;
use bytes::Bytes;
use azure_core::prelude::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let storage_client = initialize().as_storage_client();
    let file_share_client = storage_client.as_file_share_client("test-file-share");
    let file_client = file_share_client.as_file_client("test-file-6");

    // create
    let file_permission = FilePermission::from("inherit");
    let file_arrtibutes = FileAttributes::from("None");
    let file_creation_time = FileCreationTime::from("now");
    let file_last_write_time = FileLastWriteTime::from("now");
    let content_length = ContentLength::new(1024);
    let f_type = FType::from("file");
    let response = file_client
        .create_file()
        .content_length(content_length)
        .f_type(f_type)
        .dir_path("test-directory-2/test-directory-3")
        .file_permission(file_permission)
        .file_attributes(file_arrtibutes)
        .file_creation_time(file_creation_time)
        .file_last_write_time(file_last_write_time)
        .execute()
        .await
        .unwrap();

    println!("{:#?}", response);
    // get
    let response = file_client
        .get_file()
        .dir_path("test-directory-2/test-directory-3")
        .execute()
        .await
        .unwrap();
    println!("{:#?}", response);

    //write (put range)
    let write = Write::from("update");
    let range = Range::new(0,10);
    let data = Bytes::from("test range".to_string());
    let len:u32 = data.len() as u32;
    let len:u64 = len as u64;
    let content_length = ContentLength::new(len);
    let response = file_client
        .put_range(data)
        .write(write)
        .range(range)
        .content_length(content_length)
        .dir_path("test-directory-2/test-directory-3")
        .execute()
        .await
        .unwrap();

    println!("{:#?}", response);

    // get
    let response = file_client
        .get_file()
        .dir_path("test-directory-2/test-directory-3")
        .execute()
        .await
        .unwrap();
    println!("{:#?}", response);

    //set length, set properties and get proterties
    let file_permission = FilePermission::from("preserve");
    let file_creation_time = FileCreationTime::from("preserve");
    let file_last_write_time = FileLastWriteTime::from("preserve");
    let content_length = ContentLength::new(1024);

    let response = file_client
        .set_file_properties()
        .content_length(content_length)
        .f_type(f_type)
        .dir_path("test-directory-2/test-directory-3")
        .file_permission(file_permission)
        .file_attributes(file_arrtibutes)
        .file_creation_time(file_creation_time)
        .file_last_write_time(file_last_write_time)
        .execute()
        .await
        .unwrap();

    let response = file_client
        .get_file_properties()
        .dir_path("test-directory-2/test-directory-3")
        .execute()
        .await
        .unwrap();
    assert_eq!(response.file.content_length,1024);
    println!("{:#?}", response);

    //delete
    let response = file_client
        .delete_file()
        .dir_path("test-directory-2/test-directory-3")
        .execute()
        .await
        .unwrap();
    println!("{:#?}", response);
    Ok(())
}
fn initialize() -> Arc<StorageAccountClient>{
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let http_client = new_http_client();

    StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)

}

