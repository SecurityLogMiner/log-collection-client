use aws_sdk_iam::{Client, Error};
use aws_sdk_iam::operation::{
    attach_role_policy::*, create_access_key::*, create_role::*, create_service_linked_role::*,
    delete_user::*, delete_user_policy::*, get_account_password_policy::*, get_role::*,
    list_attached_role_policies::*, list_groups::*, list_policies::*, list_role_policies::*,
    list_roles::*, list_saml_providers::*, list_users::*,
};
use aws_sdk_iam::error::SdkError;
use tokio::time::{sleep, Duration};
use aws_sdk_iam::types::{AccessKey, Policy, PolicyScopeType, Role, User};


// #[::tokio::main]


/*
    Goals:
    -[] Assign minimal permission policies based on need
    -[] Assign minimal permission policies based on all
    -[] List policies 
    -[] Detach policies
        Maybe the logistics can be better managed on the AWS rather than CMND line. 
        Should we still offer the option?
    -[] Create a specifc customer role on AWS IAM
        - contains minimal permissions
    -[] 


]*/
pub async fn
start_iam() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iam::Client::new(&config);
    Ok(client)
}

pub async fn create_user(client: &Client, user_name: &str) -> Result<User, Error> {
    let response = client.create_user().user_name(user_name).send().await?;

    Ok(response.user.unwrap())
}


/*
    Requires IAM policy ListUsers
 */
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

pub async fn delete_user(client: &Client, user: &User) -> Result<(), SdkError<DeleteUserError>> {
    let user = user.clone();
    let mut tries: i32 = 0;
    let max_tries: i32 = 10;

    let response: Result<(), SdkError<DeleteUserError>> = loop {
        match client
            .delete_user()
            .user_name(user.user_name())
            .send()
            .await
        {
            Ok(_) => {
                break Ok(());
            }
            Err(e) => {
                tries += 1;
                if tries > max_tries {
                    break Err(e);
                }
                sleep(Duration::from_secs(2)).await;
            }
        }
    };

    response
}