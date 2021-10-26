#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use std::sync::Arc;
use azure_storage::core::prelude::{StorageAccountClient, AsStorageClient};
use azure_storage::file::prelude::AsFileShareClient;
use azure_storage::{StoredAccessPolicy, StoredAccessPolicyList};
use chrono::{FixedOffset, Utc};
use std::ops::Add;
use azure_core::prelude::*;


#[tokio::test]
async fn create_and_delete_file_share(){
    let name: &'static str = "file-share-test1";
    let storage_client = initialize().as_storage_client();
    let file_share = storage_client.as_file_share_client(name);

    file_share
        .create()
        .execute()
        .await
        .unwrap();

    // get acl without stored access policy list
    let _result = file_share.get_acl().execute().await.unwrap();

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(chrono::Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = file_share
        .set_acl()
        .stored_access_policy_list(&sapl)
        .execute()
        .await
        .unwrap();

    // now we get back the acess policy list and compare to the one created
    let result = file_share.get_acl().execute().await.unwrap();
    // we cannot compare the returned result because Azure will
    // trim the milliseconds
    // assert!(sapl == result.stored_access_policy_list);
    assert!(sapl.stored_access.len() == result.stored_access_policy_list.stored_access.len());

    for (i1, i2) in sapl
        .stored_access
        .iter()
        .zip(result.stored_access_policy_list.stored_access.iter())
    {
        assert!(i1.id == i2.id);
        assert!(i1.permission == i2.permission);
    }

    let res = file_share.get_properties().execute().await.unwrap();

    // list file share
    let list = storage_client
        .list_fileshares()
        .execute()
        .await
        .unwrap();

    // delete
    file_share
        .delete()
        .execute()
        .await
        .unwrap();


}



fn initialize() -> Arc<StorageAccountClient>{
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let http_client = new_http_client();

    StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)

}