use std::time::Instant;
use rayon::prelude::*;
use reqwest::blocking::Client;
use std::sync::{ Arc, Mutex };

fn fetch_url(url: &str) -> usize {
    let client = Client::new();
    println!("Running on thread: {:?}", std::thread::current().id());
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
    let thread_counter = Arc::new(Mutex::new(0));

    urls.par_iter().for_each(|&url| {
        let _ = fetch_url(url);
        let mut count = thread_counter.lock().unwrap();
        *count += 1;
    });

    println!("Total time taken: {:?}", start_time.elapsed());
    println!("Threads used: {:?}", *thread_counter.lock().unwrap());
}
