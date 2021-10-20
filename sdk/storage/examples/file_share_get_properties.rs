use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use chrono::{FixedOffset, Utc};
use std::error::Error;
use std::ops::Add;
use std::time::Duration;
use azure_storage::file::prelude::AsFileShareClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let file_share_name = std::env::args()
        .nth(1)
        .expect("please specify file share name as command line parameter");
    let http_client = new_http_client();
    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();
    let file_share = storage_account.as_file_share_client(file_share_name);

    let res = file_share
        .get_properties()
        .execute()
        .await?;
    println!("\nget_properties() == {:?}", res);
    Ok(())

}