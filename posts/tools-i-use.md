---
title: The tools I use every day
date: 2026-07-27
tags: [tools, meta, rust]
summary: A practical list of the software I reach for daily, and why each one earns its place.
---

A tool earns its place by disappearing. The best tools are the ones you stop
noticing because they just work. Here is the list I reach for every day.

## The editor

I live in Neovim. It is fast, scriptable, and it never gets in the way.

```lua
-- A tiny snippet of my config
vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.tabstop = 2
vim.opt.shiftwidth = 2
vim.opt.expandtab = true
```

## The terminal

I use a plain terminal with a multiplexer. It lets me keep a server, an
editor, and a shell open side by side.

## The languages

- **Rust** for tools and services. It is fast and safe.
- **Python** for quick scripts and data.
- **Bash** for glue.

## The version control

Git, of course. The habits matter more than the tool:

- Commit small.
- Write clear messages.
- Rebase before you merge.

## A comparison

| Tool | Job | Why I like it |
| ---- | --- | ------------- |
| Neovim | Edit | Fast, scriptable |
| Git | Track | Ubiquitous |
| Rust | Build | Safe, fast |
| Markdown | Write | Portable |

## The principle

> Use the boring tool that works over the exciting tool that breaks.

The best tool is the one you already know and trust. New tools are fun. They
are also a tax. Choose carefully.
