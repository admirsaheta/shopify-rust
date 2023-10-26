# shopify-rust

A Rust library for interacting with the Shopify API.

## Features

- Supports authentication to Shopify store (that is currently the only thing it does)
- Easy to install and use in your own Rust projects

## Installation

To use `shopify-rust` in your own project, add it as a dependency in your `Cargo.toml` file:

```rust
[dependencies]
shopify-rust = "0.1"
```

## Usage

To use `shopify-rust`, include it in your code and call the desired functions:

```rust
use shopify_rust::{ShopifyApp, Credentials, AccessMode};

fn main() {
    let credentials = Credentials::new("ABCD".to_string(), "WXYZ".to_string());
    let access_mode = AccessMode::Online;
    let auth_callback_url = String::from("htpp://127.0.0.1");
    let host = String::from("SHOP_NAME.myshopify.com");
    let scopes = vec!["write_products"];
    
    let shop = ShopApp::new(
        access_mode,
        auth_callback_url,
        credentials,
        host,
        scopes
    )
}
```


## Prerequisites

- A Shopify store
- A Shopify API key and password