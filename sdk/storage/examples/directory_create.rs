#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::file::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;
use azure_storage::file::clients::directory_client::AsDirectoryClient;


// #[derive(Debug,Clone,Serialize)]
// #[serde(rename_all = "snaka_case" )]
// struct SampleEntity{
//     pub something: String,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let storage_client = storage_account_client.as_storage_client();

    // input name
    // let container = std::env::args()
    //     .nth(1)
    //     .expect("please specify file share name as command line parameter");

    let file_share_client = storage_client.as_file_share_client("test-file-share");

    let directory_client = file_share_client.as_directory_client("test-directory-4");

    // TODO fill in the x-ms-file-permission header
    let file_permission = FilePermission::from("inherit");
    let file_arrtibutes = FileAttributes::from("None");
    let file_creation_time = FileCreationTime::from("now");
    let file_last_write_time = FileLastWriteTime::from("now");

    let response = directory_client
        .create_directory()
        .dir_path("test-directory-2/test-directory-3")
        .file_permission(file_permission)
        .file_attributes(file_arrtibutes)
        .file_creation_time(file_creation_time)
        .file_last_write_time(file_last_write_time)
        .execute()
        .await?;
    println!("{:#?}", response);
    Ok(())
}