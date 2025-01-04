# Concurrency Benchmarking in Rust

This project compares four different approaches to fetching multiple URLs in Rust:

1. **Simple Sequential Execution** – Fetches URLs one by one in a blocking manner.
2. **Thread Pool Execution** – Uses a thread pool to fetch multiple URLs concurrently.
3. **Blocking Threads Execution** – Spawns multiple threads, each making a blocking request.
4. **Asynchronous Execution** – Uses asynchronous Rust (`tokio`) to fetch multiple URLs concurrently.

## Performance Results

Each approach fetches the same set of 20 URLs, and the total execution time is measured.

| Approach             | Total Time Taken |
|----------------------|-----------------|
| Simple Sequential   | ~36.7s           |
| Thread Pool         | ~4.4s            |
| Blocking Threads    | ~3.3s            |
| Asynchronous Rust   | ~2.8s            |

## Project Structure

Each approach is implemented in its own folder:

- `simple-sequential/`
- `thread-pool/`
- `blocking-threads/`
- `async-example/`

## Running the Project

To run all four implementations sequentially, use the following commands:

```sh
cd simple-sequential && cargo run
cd ../thread-pool && cargo run
cd ../blocking-threads && cargo run
cd ../async-example && cargo run
```

## Dependencies

Each implementation uses the `reqwest` crate for HTTP requests. The async version also requires `tokio`.

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
tokio = { version = "1", features = ["full"] }
rayon = "1.5"  # Only for the thread pool version
```

## Explanation of Approaches

### 1. Simple Sequential Execution

- Fetches each URL one by one.
- The slowest approach since each request waits for the previous one to finish.

### 2. Thread Pool Execution

- Uses `rayon` to process multiple URLs in parallel.
- Optimized thread allocation improves performance.

### 3. Blocking Threads Execution

- Spawns multiple threads manually.
- Faster than sequential but can be inefficient due to high thread overhead.

### 4. Asynchronous Execution

- Uses `tokio` to fetch all URLs concurrently in a single thread.
- The most efficient approach due to its event-driven model.

## Conclusion

- **Asynchronous Rust (`tokio`) is the fastest** for network-bound tasks.
- **Thread Pool (`rayon`) is a good alternative** if async is not an option.
- **Blocking threads should be used with caution**, as thread creation is expensive.
- **Simple Sequential is not recommended** for multiple network requests.

## Future Improvements

- Adding error handling.
- Logging response times per URL.
- Testing with different numbers of URLs.

---

### Author

*Upendra Singh* 🚀

