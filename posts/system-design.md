---
title: A practical guide to system design
date: 2026-07-24
tags: [ideas, tools, rust]
summary: System design is not about drawing boxes. It is about making trade-offs you can defend.
---

System design is not about drawing boxes and arrows. It is about making
trade-offs you can defend. This post is a practical walk through the process.

## Start with the problem

Do not start with a diagram. Start with the problem.

- What are we building?
- Who uses it?
- How much traffic?
- What can fail?

## The building blocks

Most systems are a small set of pieces arranged differently.

- **Load balancer** — spreads traffic.
- **Application server** — runs the logic.
- **Database** — stores the state.
- **Cache** — makes reads fast.
- **Queue** — decouples work.
- **Object store** — holds files.

## A worked example

Suppose we build a URL shortener.

```text
User -> Load balancer -> App -> Cache -> Database
                              \-> Queue -> Analytics
```

The flow:

1. The user sends a long URL.
2. The app generates a short code.
3. The app stores the mapping in the database.
4. The app writes an event to the queue.
5. A worker reads the queue and updates analytics.

## The trade-offs

Every choice has a cost.

| Decision | Trade-off |
| -------- | --------- |
| Cache | Speed for staleness |
| Queue | Decoupling for complexity |
| Sharding | Scale for harder queries |
| Replication | Availability for consistency |

## A checklist

- [x] Define the problem.
- [x] Estimate the scale.
- [x] Pick the building blocks.
- [x] Identify the bottlenecks.
- [x] Plan for failure.

## The closing thought

> A good design is not the one with the most boxes. It is the one that fails
> gracefully and is easy to change.

Start simple. Add complexity only when the problem demands it.
