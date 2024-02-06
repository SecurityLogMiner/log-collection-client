use aws_sdk_iam::{Client, Error};
use aws_sdk_iam::operation::{
    attach_role_policy::*, create_access_key::*, create_role::*, create_service_linked_role::*,
    delete_user::*, delete_user_policy::*, get_account_password_policy::*, get_role::*,
    list_attached_role_policies::*, list_groups::*, list_policies::*, list_role_policies::*,
    list_roles::*, list_saml_providers::*, list_users::*,
};
use aws_sdk_iam::error::SdkError;

use aws_sdk_iam::types::{AccessKey, Policy, PolicyScopeType, Role, User};


// #[::tokio::main]
pub async fn
start_iam() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iam::Client::new(&config);
    Ok(client)
}


pub async fn list_users(
    client: &Client,
    path_prefix: Option<String>,
    marker: Option<String>,
    max_items: Option<i32>,
) -> Result<ListUsersOutput, SdkError<ListUsersError>> {
    let response = client
        .list_users()
        .set_path_prefix(path_prefix)
        .set_marker(marker)
        .set_max_items(max_items)
        .send()
        .await?;
    Ok(response)
}