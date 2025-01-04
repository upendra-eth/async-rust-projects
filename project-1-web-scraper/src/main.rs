use reqwest::Client;
use tokio::task;
use std::time::Instant;

/// List of URLs to scrape
const URLS: [&str; 5] = [
    "https://www.rust-lang.org",
    "https://www.github.com",
    "https://www.wikipedia.org",
    "https://www.stackoverflow.com",
    "https://www.reddit.com",
];

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    println!("Starting web scraper...");
    
    // Create an HTTP client
    let client = Client::new();

    // Concurrently fetch URLs
    let mut tasks = Vec::new();
    for &url in &URLS {
        let client = client.clone(); // Clone the client for each task
        let task = task::spawn(async move {
            scrape_url(client, url).await
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        match task.await {
            Ok(result) => match result {
                Ok((url, size)) => println!("URL: {url} | Size: {size} bytes"),
                Err(e) => eprintln!("Failed to fetch: {e}"),
            },
            Err(e) => eprintln!("Task error: {e:?}"),
        }
    }

    let elapsed_time = start_time.elapsed();
    println!("Completed in {:.2?}", elapsed_time);
}

/// Function to scrape a single URL
async fn scrape_url(client: Client, url: &str) -> Result<(String, usize), reqwest::Error> {
    let response = client.get(url).send().await?;
    let size = response.text().await?.len();
    Ok((url.to_string(), size))
}
