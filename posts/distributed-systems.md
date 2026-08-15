---
title: Notes on distributed systems
date: 2026-08-12
tags: [rust, ideas, tools]
summary: A few hard-won lessons about building systems that span many machines.
---

I have spent years building systems that run across many machines. Here are
the lessons I keep coming back to.

## The network is not reliable

The first rule of distributed systems is that the network can fail at any
time. A request can be lost, duplicated, or delayed. You must design for all
three.

- **Lost** requests need retries.
- **Duplicated** requests need idempotency.
- **Delayed** requests need timeouts.

## Consistency is a spectrum

There is no single "correct" answer for consistency. You choose a point on a
spectrum and pay for it.

| Model | Cost | Use when |
| ----- | ---- | -------- |
| Strong | High | Money, accounts |
| Causal | Medium | Feeds, chat |
| Eventual | Low | Caches, counters |

## A worked example

Here is a small Rust sketch of a retry with backoff:

```rust
use std::time::Duration;

async fn call_with_retry<F, T>(mut f: F) -> Result<T, String>
where
    F: FnMut() -> Result<T, String>,
{
    let mut delay = Duration::from_millis(100);
    for _ in 0..5 {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                tokio::time::sleep(delay).await;
                delay *= 2;
                let _ = e;
            }
        }
    }
    Err("gave up".to_string())
}
```

## The hard part

> The hard part of distributed systems is not the happy path. It is the
> failure path that you did not think to write.

Design the failure path first. The happy path will take care of itself.
