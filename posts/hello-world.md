---
title: Hello, world
date: 2026-08-13
tags: [meta, rust]
summary: The first post on this blog. It explains what this site is and how it is built.
---

This is the first post on this blog.

The site is a static blog. I write posts as markdown files with a small
amount of frontmatter at the top. A tiny Rust program reads the files and
generates plain HTML and CSS. Nginx serves the result.

## Why a static site

A static site is fast, simple, and easy to host. There is no database, no
server-side code, and nothing to break. The browser receives plain HTML and
CSS, and nothing else.

## What comes next

I will write more posts here. They will cover the tools I use, the things I
learn, and the projects I build.

```rust
fn main() {
    println!("Hello, world!");
}
```

> A blog is a conversation with your future self.
