use crate::rocal_api_client::RocalAPIClient;

use super::{
    unsubscribe::get_subscription_status,
    utils::{
        project::{find_project_root, get_app_name},
        refresh_user_token::refresh_user_token,
    },
};

pub async fn list() {
    refresh_user_token().await;

    let subscription_status = if let Ok(status) = get_subscription_status().await {
        status
    } else {
        eprintln!("Could not find your subscription.");
        return;
    };

    if subscription_status.get_plan() != "developer" && subscription_status.get_plan() != "pro" {
        eprintln!("You must subscribe Developer or Pro plan to use sync servers.");
        return;
    }

    get_sync_server_info().await;
}

async fn get_sync_server_info() {
    let root_path = find_project_root().expect(
        "Failed to find a project root. Please run the command in a project built by Cargo",
    );

    let app_name = get_app_name(&root_path);

    let client = RocalAPIClient::new();

    match client.get_sync_server(&app_name).await {
        Ok(sync_server) => {
            println!("App ID: {}", sync_server.get_app_id());
            println!("Sync server: {}", sync_server.get_endpoint());
            println!("To use the sync server, set the App ID and the endpoint on your config.");
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
