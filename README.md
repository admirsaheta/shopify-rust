# shopify-rust

A Rust library for interacting with the Shopify API.

## Features

- Supports all Shopify API endpoints
- Allows you to retrieve, create, update, and delete products, orders, and other resources
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
extern crate shopify_rust;

use shopify_rust::{Client, Product};

fn main() {
  let client = Client::new("MY_SHOP_DOMAIN", "MY_ACCESS_TOKEN");
  let products = client.get_products().unwrap();
    for product in products {
      println!("{} - {}", product.id, product.title);
    }
}
```


## Documentation

For more detailed documentation and examples, see the [API reference](https://docs.rs/shopify-rust) and the [examples](https://github.com/admirsaheta/shopify-rust/tree/master/examples).

## Prerequisites

- A Shopify store
- A Shopify API key and password

## Contributing

We welcome contributions to `shopify-rust`. If you'd like to contribute, please follow the [contributing guidelines](CONTRIBUTING.md).
