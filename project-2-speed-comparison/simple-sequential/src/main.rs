use std::time::Instant;
use reqwest::blocking::Client;
use std::thread;

fn fetch_url(client: &Client, url: &str) -> usize {
    println!("Running on thread: {:?}", thread::current().id());
    let response = client.get(url).send().unwrap();
    response.text().unwrap().len()
}

fn main() {
    let urls = [
        "https://www.rust-lang.org",
        "https://crates.io",
        "https://docs.rs",
        "https://github.com",
        "https://www.mozilla.org",
        "https://www.wikipedia.org",
        "https://www.stackoverflow.com",
        "https://news.ycombinator.com",
        "https://www.reddit.com",
        "https://www.nytimes.com",
        "https://www.bbc.com",
        "https://www.cnn.com",
        "https://www.theverge.com",
        "https://arstechnica.com",
        "https://www.medium.com",
        "https://www.linkedin.com",
        "https://www.apple.com",
        "https://www.microsoft.com",
        "https://www.amazon.com",
        "https://www.google.com",
    ];
    let start_time = Instant::now();
    let client = Client::new();

    for url in &urls {
        fetch_url(&client, url);
    }

    println!("Total time taken: {:?}", start_time.elapsed());
}
