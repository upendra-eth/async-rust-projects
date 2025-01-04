
# Project 1: Async Web Scraper

Let’s build **Project 1: Async Web Scraper** step by step with a detailed explanation. The goal is to understand how to use Rust's async ecosystem to fetch data concurrently from multiple URLs efficiently.  

---

### **What Is a Web Scraper?**
A web scraper is a program that sends HTTP requests to a list of web pages, extracts data from the responses, and processes or stores that data.  

### **Why Use Async for Web Scraping?**
When scraping multiple websites, many requests are I/O-bound (waiting for a server's response). Using async programming allows the program to handle many such requests concurrently without wasting resources on thread overhead.

---

### **High-Level Design**
1. **Input**: A list of URLs to scrape.
2. **Output**: Print the HTTP status and size of the response for each URL.
3. **Steps**:
   - Read a list of URLs from a file or define them in the code.
   - Use an async HTTP client (`reqwest`) to send GET requests concurrently.
   - Process and print the response information.


#### **Key Components**
1. **Tokio Runtime**:
   - The `#[tokio::main]` macro initializes the async runtime.
   - It allows the use of async/await in the main function.

2. **Concurrency with `tokio::task::spawn`**:
   - Each URL fetch runs as an independent task.
   - Tasks are lightweight compared to threads, and many can run concurrently.

3. **Async HTTP Client**:
   - `reqwest::Client` is used for sending HTTP GET requests.
   - The client is cloned for each task to handle requests independently.

4. **Error Handling**:
   - The `Result` type is used to propagate and handle errors gracefully.

5. **Performance Measurement**:
   - `std::time::Instant` tracks the total execution time.

---


## 🚀 Features
- Concurrently fetch data from multiple URLs using async programming.
- Efficient and scalable with low overhead using `tokio` and `reqwest`.

## 📦 Requirements
- Add the following dependencies:
  ```toml
  [dependencies]
  tokio = { version = "1.42", features = ["full"] }
  reqwest = { version = "0.12.12", features = ["json"] }
  ```

## 📜 Running the Project
1. Clone the repository and navigate to this folder:
   ```bash
   cd async-rust-projects/project-1-web-scraper
   ```
2. Run the program:
   ```bash
   cargo run
   ```

## 📚 Learnings
- Basics of asynchronous programming in Rust.
- Managing multiple concurrent tasks with `tokio`.
- Using an async HTTP client (`reqwest`) for non-blocking I/O.

## 💡 Future Improvements
- Read URLs from a file.
- Save responses to local files.
- Implement retries on failure.

