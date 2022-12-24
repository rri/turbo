
---
**NOTE**

The authoritative version of this documentation is [this README.md file](https://github.com/rri/turbo#readme).

---

 # Introduction

 **turbo** is a lightweight terminal-based modal editor for working with structured text. That is quite a mouthful, so let's break it down!

 **turbo** is:

 * **Lightweight.** Its process takes up a tiny memory and compute footprint (see [benchmarks](#benchmarks)).
 * **Terminal-based.** It is built on top of the minimalistic [crossterm] crate.
 * **Modal.** It supports distinct input modes that optimize for different kinds of interactions (à la [vim](https://github.com/vim/vim)).
 * **Designed for structured text.** Its buffers are designed to allow for a variety of standard or custom parsers to be plugged in.

 All in all, **turbo** has the rather ambitious goal of displacing your favorite integrated development environment ("IDE") as your tool of choice for building software. Being able to understand the parse tree for your target programming language (with incremental parsing and error correction) are thus key to its value proposition.

 # Installation

 # Usage

 Assuming you have the `turbo` binary in your operating system path, simply run the executable:

 ```
 $ turbo
 ```

 # Benchmarks

 # Contributing

 Fork the source GitHub repository at [rri/turbo](https://github.com/rri/turbo) and submit an upstream pull request with your changes. Before you start work on any feature, please discuss the proposed feature on the [GitHub issue tracker](https://github.com/rri/turbo/issues) to avoid later surprises.