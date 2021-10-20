use azure_core::prelude::*;
use chrono::{FixedOffset, Utc};
use azure_storage::{StoredAccessPolicyList, StoredAccessPolicy};
use azure_storage::core::prelude::*;
use std::ops::Add;
use std::error::Error;
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


    // now we get back the acess policy list and compare to the one created
    let result = file_share.get_metadata().execute().await?;

    println!("\nget_metadata() == {:?}", result);

    Ok(())
}