# Understanding Thread Pool vs. Blocking Threads Execution

This document explains the difference between **Thread Pool Execution** and **Blocking Threads Execution** in the context of our concurrency benchmarking project.

## Why Compare These Two Approaches?
Both **Thread Pool Execution** and **Blocking Threads Execution** use multiple threads to fetch URLs concurrently, but they differ in **how threads are managed** and their **performance characteristics**. Understanding these differences helps in choosing the right approach for real-world applications.

## Key Differences

| Feature                 | Thread Pool Execution (`rayon`) | Blocking Threads Execution (`thread::spawn`) |
|-------------------------|--------------------------------|--------------------------------------------|
| **Thread Management**   | Uses a **fixed thread pool** managed by Rayon | Spawns a **new OS thread** for each task |
| **Work Distribution**   | Uses **work-stealing** to balance load | No load balancing, threads can be idle |
| **Thread Reusability**  | Threads are **reused**, reducing overhead | New threads are created and destroyed per request |
| **Performance**         | **Efficient for large workloads** | Less efficient when too many threads are created |
| **Ease of Implementation** | Simple API (`par_iter()`) | Requires manual thread creation & synchronization |

## Thread Pool Execution (`thread-pool/`)
**Implementation:** Uses the **Rayon** crate, which provides a parallel iterator API (`par_iter()`).

```rust
urls.par_iter().for_each(|&url| {
    fetch_url(url);
});
```

✅ **Advantages:**
- Uses a **fixed number of threads** (typically equal to CPU cores).
- **Work-stealing** ensures balanced workload distribution.
- Threads are **reused**, reducing the cost of thread creation.

🚨 **When to Use:**
- When you have **many** tasks to execute concurrently.
- When you want **efficient CPU-bound parallelism**.

---

## Blocking Threads Execution (`blocking-threads/`)
**Implementation:** Uses **manual thread creation** with `thread::spawn()`.

```rust
let handles: Vec<_> = urls.into_iter().map(|url| {
    thread::spawn(move || {
        fetch_url(url);
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

✅ **Advantages:**
- Allows **precise control** over thread creation.
- Can be useful for **small-scale concurrency**.

🚨 **Disadvantages:**
- **New OS thread per task** → **high overhead** for large workloads.
- Threads **remain idle** if tasks finish at different times.
- **Not scalable** when dealing with hundreds or thousands of tasks.

🚨 **When to Use:**
- When you need **fine-grained control** over threads.
- When you have **only a few tasks to run concurrently**.

---

## Final Thoughts
- **Rayon's thread pool (`par_iter()`) is the recommended approach** for most parallel workloads because it efficiently manages threads and balances work.
- **Manual threads (`thread::spawn()`) should be used only for small-scale concurrency** or when precise control over threads is required.

By benchmarking these two methods, we demonstrate the importance of **thread management** in concurrent Rust programs. Understanding their differences will help in designing efficient multi-threaded applications.

For further comparison, check out the `async-example/` folder, which demonstrates the **Tokio async runtime**, an even better approach for I/O-bound tasks.

---

🔹 **Next Steps:** Explore `async-example/` to see how **asynchronous execution** compares to these multi-threading approaches!

