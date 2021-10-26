#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::file::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;
use azure_storage::file::clients::directory_client::AsDirectoryClient;
use azure_storage::file::clients::file_client::AsFileClient;
use azure_storage::Properties;
use bytes::Bytes;


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

    let file_client = file_share_client.as_file_client("sd.txt");

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
        .await?;

    println!("{:#?}", response);
    Ok(())
}