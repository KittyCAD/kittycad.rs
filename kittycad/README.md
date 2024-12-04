# `kittycad`

A fully generated & opinionated API client for the KittyCAD API.

[![docs.rs](https://docs.rs/kittycad/badge.svg)](https://docs.rs/kittycad)

## API Details

API server for Zoo



### Contact


| url | email |
|----|----|
| <https://zoo.dev> | api@zoo.dev |



## Client Details

This client is generated from the [OpenAPI specs](https://api.zoo.dev) based on API spec version `0.1.0`. This way it will remain up to date as features are added.

The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
kittycad = "0.3.27"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use kittycad::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `KITTYCAD_API_TOKEN`
- `ZOO_API_TOKEN`

And then you can create a client from the environment.

```rust,no_run
use kittycad::Client;

let client = Client::new_from_env();
```
