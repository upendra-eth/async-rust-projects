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

Rayon achieves efficient parallel execution using **work-stealing**, **a global thread pool**, and **fine-grained task scheduling**. Let's break it down step by step.

---

## **1. The Fixed Thread Pool**
Rayon maintains a **fixed-size thread pool**, meaning it creates a set number of worker threads (equal to the number of CPU cores by default) and reuses them throughout execution. This avoids the overhead of repeatedly spawning and destroying threads.

### How is this different from manual threads?
- If you create threads manually (`std::thread::spawn`), each new thread incurs creation and destruction costs.
- Rayon’s **pre-created** threads stay alive and are **reused**, leading to better performance.

---

## **2. Work Stealing – Load Balancing Across Threads**
Rayon uses a **work-stealing algorithm** to balance workloads dynamically.

### **How Work Stealing Works**
1. **Each thread gets a task queue**: When a parallel operation (e.g., `.par_iter().for_each(...)`) starts, the work is split into **small tasks**, each assigned to a thread.
2. **Threads execute their own tasks first**: Each worker thread processes tasks from its queue.
3. **Idle threads steal work from busy threads**: If a thread finishes its queue but other threads still have tasks left, it **steals tasks** from another thread’s queue.

### **Why is this efficient?**
- Prevents some threads from being overloaded while others sit idle.
- Helps when tasks have uneven execution times.
- Leads to **better CPU utilization** without unnecessary thread creation.

---

## **3. Fine-Grained Task Scheduling**
Rayon breaks work into **small units** (often called "jobs" or "tasks"). Unlike manually spawning threads (where each thread gets a large chunk of work), Rayon’s tasks are **dynamically scheduled**.

### Example: Splitting Work
If you use `.par_iter()`, Rayon:
1. **Splits the data into chunks** (based on heuristics).
2. **Assigns chunks to available threads**.
3. **If some threads finish earlier, they take leftover work from others**.

This approach ensures **no single thread is overloaded**.

---

## **4. No Explicit Synchronization Needed**
With manual threads, you often need `Arc<Mutex<T>>` for shared data. Rayon’s **functional-style** APIs (`par_iter()`, `par_map()`, etc.) ensure that:
- **Immutable data** is safely accessed by multiple threads.
- **Mutable data** can be efficiently reduced (e.g., `.par_iter().sum()` avoids locks).

### **Why is this better than `std::thread::spawn`?**
- `std::thread::spawn` requires **explicit thread management** and **manual locking**.
- Rayon handles **data splitting, task distribution, and synchronization** automatically.

---

## **5. Global Thread Pool and Lazy Execution**
Rayon maintains a **global thread pool** that is lazily initialized on first use. Unlike `tokio`, which has its own async runtime, Rayon works within **synchronous Rust** and automatically manages thread scheduling.

- The **first time you call `.par_iter()`**, Rayon initializes the thread pool.
- **Subsequent parallel operations** reuse the same pool, avoiding overhead.

---

### **Comparison: Rayon vs. Manual Threads**
| Feature | Rayon | Manual Threads (`std::thread`) |
|---------|------|----------------|
| **Thread Pool** | Yes (fixed-size) | No (manual thread creation) |
| **Work Stealing** | Yes | No |
| **Automatic Load Balancing** | Yes | No |
| **Task Granularity** | Fine-grained | Coarse (one task per thread) |
| **Reused Threads** | Yes | No |
| **Synchronization Complexity** | Low (built-in safe methods) | High (requires `Mutex`/`Arc`) |

---

## **Conclusion**
Rayon achieves efficient parallelism by:
1. **Reusing** a fixed thread pool (avoiding thread creation/destruction overhead).
2. **Dynamically balancing work** using work-stealing.
3. **Breaking tasks into fine-grained units** to prevent load imbalance.
4. **Minimizing synchronization costs** with functional-style APIs.

It is **not an async runtime** like `tokio`, but for CPU-bound parallel tasks (e.g., numerical computations, data processing), it **massively outperforms manual threads**.

---

Would you like me to illustrate it with a real-world example or diagram? 🚀

