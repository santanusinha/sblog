---
title: The command line is a friend
date: 2026-08-14
tags: [tools, meta]
summary: A short ode to the terminal and the small habits that make it fast.
---

I spend most of my day in a terminal. It is not because I am nostalgic. It is
because the command line is the fastest way to move ideas into a machine.

## Small habits

- Use `git status` before you commit.
- Use `git diff` before you push.
- Use `history` to find the command you forgot.

## A tiny script

```bash
#!/usr/bin/env bash
# Count the lines in every markdown file.
for f in posts/*.md; do
  printf "%4d  %s\n" "$(wc -l < "$f")" "$f"
done
```

## Why it matters

> The command line rewards the curious. Every flag you learn is a tool you
> keep forever.

That is the whole post. Short on purpose.
