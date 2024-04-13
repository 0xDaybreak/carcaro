# Carcaro

Carcaro is a backend project written in Rust, designed to support a website for visualizing cars and swapping their colors. The frontend of the website is developed using React with TypeScript.

## Overview

This backend system leverages various Rust libraries to handle tasks such as HTTP server management, database operations, image processing, and cloud storage interactions. Below are the key libraries used in this project:

- [Tokio](https://github.com/tokio-rs/tokio): Asynchronous runtime for Rust, facilitating concurrent execution of tasks.
- [Warp](https://github.com/seanmonstar/warp): Web server framework for Rust, enabling the creation of RESTful APIs.
- [Reqwest](https://github.com/seanmonstar/reqwest): HTTP client for Rust, used for making requests to external APIs.
- [Serde](https://github.com/serde-rs/serde): Rust library for serializing and deserializing data formats, essential for working with JSON.
- [Serde JSON](https://github.com/serde-rs/json): Implementation of Serde's serialization and deserialization traits for JSON.
- [SQLx](https://github.com/launchbadge/sqlx): Async, pure Rust SQL crate with compile-time checked queries.
- [Image](https://github.com/image-rs/image): Library for image processing in Rust, useful for manipulating car images.
- [Azure Storage Blobs](https://github.com/Azure/azure-sdk-for-rust): Rust SDK for Azure Blob Storage, facilitating interactions with cloud storage.
- [Azure Core](https://github.com/Azure/azure-sdk-for-rust): Azure SDK core library for Rust.
- [Futures](https://github.com/rust-lang/futures-rs): Core async utilities for Rust, used for handling asynchronous computations.
- [Time](https://github.com/time-rs/time): Library for dealing with time-related tasks in Rust.
- [Natord](https://github.com/fitzgen/natord-rs): Natural order comparison library for Rust.
- [Walkdir](https://github.com/BurntSushi/walkdir): Rust library for recursively walking a directory.
- [Rayon](https://github.com/rayon-rs/rayon): Data parallelism library for Rust, useful for concurrent processing.
- [Palette](https://github.com/Ogeon/palette): Library for colors and color spaces in Rust.
- [Pwhash](https://github.com/jedisct1/rust-password-hashing): Password hashing library for Rust.

## Getting Started

To get started with Carcaro, follow these steps:

1. Clone the repository to your local machine.
2. Install Rust and Cargo if you haven't already.
3.  Add your azure access key under src/key.txt in the first
4.  Run 'cargo build' to build the project.
5.  Run 'cargo run' to start the backend server.
6.  Ensure the frontend React application is configured to communicate with this backend server.

