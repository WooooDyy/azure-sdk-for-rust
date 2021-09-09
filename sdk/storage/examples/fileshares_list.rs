use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct SampleEntity {
    pub something: String,
}

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

    let response = storage_client.list_fileshares().execute().await?;
    println!("key response = {:#?}", response);



    Ok(())
}