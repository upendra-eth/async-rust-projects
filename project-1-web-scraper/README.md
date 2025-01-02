
# Project 1: Async Web Scraper

This project demonstrates how to use async concurrency to fetch data from multiple websites concurrently.

## 🚀 Features
- Fetches data using `reqwest`.
- Utilizes the `tokio` runtime for asynchronous tasks.
- Simple and scalable design for handling multiple URLs.

## 📦 Requirements
- Add the following dependencies in `Cargo.toml`:
  ```toml
  [dependencies]
  tokio = { version = "1", features = ["full"] }
  reqwest = { version = "0.11", features = ["json"] }
