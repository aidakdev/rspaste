<p align="center">
    <h1 align="center"><b>rspaste</b></h1>
    <p align="center">
        A simple, fast and fully-typed JSPaste API wrapper for Rust.
        <br />
        <a href="https://aidak.tk"><strong>aidak.tk »</strong></a>
    </p>
</p>

<p align="center">
    <a href="https://aidak.tk">
        <img src="https://img.shields.io/badge/Aidak-Official%20Product-8877ff" />
    </a>
    <a href="https://discordredirect.discordsafe.com/users/152569284390944768">
        <img src="https://img.shields.io/badge/contact-me?logo=discord&logoColor=white&color=5865F2">
    </a>
    <a href="https://twitter.com/realaidak">
        <img src="https://img.shields.io/badge/Twitter-00acee?logo=twitter&logoColor=white" />
    </a>
    <img src="https://img.shields.io/static/v1?label=License&message=Apache%202.0&color=000" />
    <img src="https://img.shields.io/static/v1?label=Stage&message=Stable 0.1&color=2BB4AB" />
    <a href="https://crates.io/crates/rspaste">
      <img src="https://img.shields.io/crates/v/rspaste">
    </a>
    <br />
</p>

# Installation
Put the desired version of the crate into the `dependencies` section of your `Cargo.toml`:
```toml
[dependencies]
rspaste = "*"
```

# Examples
First of all, JSPaste API can return errors if a key or secret is invalid, so a good way of handling responses is:
```rs
match some_doc {
    Ok(d) => todo!(), // ...
    Err(e) => todo!() // ...
}
```

We recommend doing this with every request to ensure that all cases are covered.

```rs
use rspaste;

fn main() {
    let doc = rspaste::api::get("key"); // get document
    let posted_doc = rspaste::api::post("content"); // post document
    let deleted_doc = rspaste::api::delete("key", "secret"); // delete document
}
```