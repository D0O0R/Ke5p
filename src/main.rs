use std::{
    env,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use fastrand::shuffle;
use reqwest::Client;

async fn cells(
    client: &Client,
    apis: &Vec<&str>,
    token: &str,
    count: &Arc<AtomicUsize>,
) -> Result<(), Box<dyn std::error::Error>> {
    for api in apis {
        match client
            .get(*api)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("{}: success", api);
                } else {
                    println!(
                        "{}: {}",
                        api,
                        resp.text().await.unwrap_or("Failed".to_string())
                    );
                    count.fetch_add(1, Ordering::Relaxed);
                };
            }
            Err(e) => {
                println!("{}: {}", api, e);
                count.fetch_add(1, Ordering::Relaxed);
            }
        }
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

    let count = Arc::new(AtomicUsize::new(0));
    let periods = fastrand::usize(50..80);

    let mut tasks = vec![];

    for _ in 0..periods {
        shuffle(&mut apis);
        let client = client.clone();
        let apis = apis.clone();
        let token = token.clone();
        let count = Arc::clone(&count);
        let task = tokio::spawn(async move {
            cells(&client, &apis, &token, &count).await.unwrap();
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    println!("{}", count.load(Ordering::Relaxed));
    Ok(())
}
