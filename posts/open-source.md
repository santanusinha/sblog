---
title: Why I publish open source
date: 2026-08-04
tags: [meta, ideas, tools]
summary: Sharing code is not charity. It is a way to make the work better and to learn in public.
---

I publish open source because it makes my work better. Sharing code is not
charity. It is a discipline that improves the code and the person who wrote it.

## The selfish reasons

- **Review.** When strangers read your code, you write cleaner code.
- **Docs.** A public project forces you to document the parts you would
  otherwise skip.
- **Feedback.** Users find the bugs you never would.

## The generous reasons

- **Reuse.** Someone else does not have to rebuild what you built.
- **Learning.** Reading other people's code is the fastest way to learn.
- **Community.** Good software is a conversation, not a monologue.

## A checklist before you publish

- [x] A clear README.
- [x] A license.
- [x] A minimal example.
- [ ] A changelog.
- [ ] Tests that run with one command.

## The hard part

> The hard part of open source is not writing the code. It is maintaining it
> after the excitement fades.

## A short example

Here is a tiny project layout that I like:

```text
my-project/
  Cargo.toml
  README.md
  LICENSE
  src/
    lib.rs
  examples/
    basic.rs
  tests/
    integration.rs
```

## The long view

Open source is a long game. The projects that last are the ones with clear
scope, a friendly maintainer, and a steady rhythm of small releases. I try to
be that maintainer.
