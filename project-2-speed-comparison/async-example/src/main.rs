use tokio::time::Instant;
use reqwest::Client;
use tokio::task;
use std::sync::{Arc, Mutex};

async fn fetch_url(client: Arc<Client>, url: &str, counter: Arc<Mutex<usize>>) -> usize {
    println!("Running on thread: {:?}", std::thread::current().id());
    let response = client.get(url).send().await.unwrap();
    let size = response.text().await.unwrap().len();
    let mut count = counter.lock().unwrap();
    *count += 1;
    size
}

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() {
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
    
    let client = Arc::new(Client::new()); // ✅ Wrapping client in Arc
    let thread_counter = Arc::new(Mutex::new(0));
    let start_time = Instant::now();

    let futures: Vec<_> = urls.iter().map(|&url| {
        let counter = Arc::clone(&thread_counter);
        let client = Arc::clone(&client); // ✅ Cloning Arc correctly
        task::spawn(fetch_url(client, url, counter)) // ✅ Passing Arc<Client>
    }).collect();

    for future in futures {
        future.await.unwrap();
    }

    println!("Total time taken: {:?}", start_time.elapsed());
    println!("Threads used: {:?}", *thread_counter.lock().unwrap());
}
