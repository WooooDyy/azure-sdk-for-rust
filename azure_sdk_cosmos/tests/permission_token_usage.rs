#![cfg(all(test, feature = "test_e2e"))]
use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::PermissionMode;

mod setup;

#[tokio::test]
async fn permissions() {
    const DATABASE_NAME: &str = "cosmos-test-db-permusage";
    const COLLECTION_NAME: &str = "cosmos-test-db-permusage";
    const USER_NAME: &str = "someone@cool.net";
    const PERMISSION: &str = "sdktest";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database()
        .with_database_name(&DATABASE_NAME)
        .execute()
        .await
        .unwrap();

    let database_client = client.with_database(&DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let create_collection_response = database_client
        .create_collection()
        .with_collection_name(&COLLECTION_NAME)
        .with_offer(Offer::Throughput(400))
        .with_partition_key(&("/id".into()))
        .with_indexing_policy(&indexing_policy)
        .execute()
        .await
        .unwrap();

    let collection_client = database_client.with_collection(&COLLECTION_NAME);

    let user_client = database_client.with_user(&USER_NAME);
    let _create_user_response = user_client.create_user().execute().await.unwrap();

    // create the RO permission
    let permission_client = user_client.with_permission(&PERMISSION);
    let permission_mode = PermissionMode::Read(create_collection_response.clone().collection);

    let create_permission_response = permission_client
        .create_permission()
        .with_permission_mode(&permission_mode)
        .with_expiry_seconds(18000) // 5 hours, max!
        .execute()
        .await
        .unwrap();

    // change the AuthorizationToken using the token
    // of the permission.
    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();
    let original_authorization_token = client.replace_auth_token(new_authorization_token);

    // Now we try to insert a document with the "read-only"
    // authorization_token just created. It will fail.
    let data = r#"
        {
            "age": 43,
            "phones": [
                "+39 1234567",
                "+39 2345678"
            ]
        }"#;
    let document = Document::new(
        "Gianluigi Bombatomica".to_owned(),
        serde_json::from_str::<serde_json::Value>(data).unwrap(),
    );
    collection_client
        .create_document()
        .with_document(&document)
        .with_is_upsert(true)
        .with_partition_keys(
            PartitionKeys::new()
                .push(document.document_attributes.id())
                .unwrap(),
        )
        .execute()
        .await
        .unwrap_err();

    // noe let's replace the permission with a
    // read-write one.
    println!(
        "Replacing authorization_token with {:?}.",
        original_authorization_token
    );
    client.replace_auth_token(original_authorization_token);

    permission_client
        .delete_permission()
        .execute()
        .await
        .unwrap();

    // All includes read and write.
    let permission_mode = PermissionMode::All(create_collection_response.collection);
    let create_permission_response = permission_client
        .create_permission()
        .with_permission_mode(&permission_mode)
        .with_expiry_seconds(18000) // 5 hours, max!
        .execute()
        .await
        .unwrap();

    let new_authorization_token: AuthorizationToken = create_permission_response
        .permission
        .permission_token
        .into();
    let original_authorization_token = client.replace_auth_token(new_authorization_token);

    // now we have an "All" authorization_token
    // so the create_document should succeed!
    let create_document_response = collection_client
        .create_document()
        .with_document(&document)
        .with_is_upsert(true)
        .with_partition_keys(
            PartitionKeys::new()
                .push(document.document_attributes.id())
                .unwrap(),
        )
        .execute()
        .await
        .unwrap();
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    // set the original (master) authorization token
    // so we can delete the user and finally finish
    // this long exmaple :).
    client.replace_auth_token(original_authorization_token);

    database_client.delete_database().execute().await.unwrap();
}
