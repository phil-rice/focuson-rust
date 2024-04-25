# Focuson CAS Library

The Focuson CAS (Content Addressable Storage) Library provides a robust and efficient way to store
and retrieve data based on content-derived identifiers. This Rust library includes a `FileSystemCAS`
implementation for file system storage and `StringStorage` extensions for convenient string data
handling.

## Features

- **ContentAddressableStorage**: A trait that defines the interface for content-addressable storage
- **FileSystemCAS**: Manages data storage in a file system, ensuring that data is stored and
  retrieved based on content-derived identifiers
- **StringStorage**: A trait extension that simplifies storing and retrieving string data. It
  automatically handles conversion to and from bytes for storage.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your machine. Visit [rust-lang.org](https://rust-lang.org)
to install them if you haven't already.

### Installation

Add this to your Cargo.toml:

```toml
[dependencies]
focuson_cas = "0.1.0"
