use std::env;

use fastrand::shuffle;
use reqwest::Client;

async fn cells(
    client: &Client,
    apis: &Vec<&str>,
    token: &str,
    count: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for api in apis {
        let resp = client
            .get(*api)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !resp.status().is_success() {
            let url = resp.url().to_string();
            println!("{}: {}", url, resp.text().await?);
            *count += 1;
        };
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut apis = vec![
        "https://graph.microsoft.com/v1.0/groups",
        "https://graph.microsoft.com/v1.0/sites/root",
        "https://graph.microsoft.com/v1.0/sites/root/sites",
        "https://graph.microsoft.com/v1.0/sites/root/drives",
        "https://graph.microsoft.com/v1.0/sites/root/columns",
        "https://graph.microsoft.com/v1.0/me/",
        "https://graph.microsoft.com/v1.0/me/drive",
        "https://graph.microsoft.com/v1.0/me/drive/root",
        "https://graph.microsoft.com/v1.0/me/drive/root/children",
        "https://graph.microsoft.com/v1.0/me/drive/recent",
        "https://graph.microsoft.com/v1.0/me/drive/sharedWithMe",
        "https://graph.microsoft.com/v1.0/me/outlook/masterCategories",
        "https://graph.microsoft.com/v1.0/me/mailFolders",
        "https://graph.microsoft.com/v1.0/me/mailFolders/Inbox/messages/delta",
        "https://graph.microsoft.com/v1.0/me/mailFolders/inbox/messageRules",
        "https://graph.microsoft.com/v1.0/me/messages",
        "https://graph.microsoft.com/v1.0/me/messages?$filter=importance eq 'high'",
        "https://graph.microsoft.com/v1.0/me/messages?$search=\"hello world\"",
        "https://graph.microsoft.com/beta/me/messages?$select=internetMessageHeaders&$top",
    ];

    let token = env::var("ONE_TOKEN").expect("TOKEN_API is not set");
    let client = reqwest::Client::new();

    let mut count = 0;

    let periods = fastrand::usize(50..80);

    for _ in 0..periods {
        shuffle(&mut apis);
        cells(&client, &apis, &token, &mut count).await?;
    }
    println!("{}", count);
    Ok(())
}
