<img src="https://file.coffee/u/_TcMC_ybVUnLqBGfIAaeY.png" width="150">

A minimal, blazing-fast and fully-typed JSPaste API library for Rust

---

![Crates.io Version](https://img.shields.io/crates/v/rspaste)
![GitHub License](https://img.shields.io/github/license/aidakdev/rspaste)
<a href="https://twitter.com/prfzpx">
    <img src="https://img.shields.io/badge/Twitter-00acee?logo=twitter&logoColor=white" />
</a>

### Features
- **Simple.** Perform requests to JSPaste out of the box with simple functions.
- **Robust.** RSPaste is built in Rust, the world's most loved programming language.
- **Safe.** RSPaste is completely typesafe, meaning that all data received from the API is typed correctly.

### Install

Put the desired version of the crate into the dependencies section of your Cargo.toml:

```toml
[dependencies]
rspaste = { version = "0.1.5" }
```

### Example
First of all, the JSPaste API may return errors if something goes wrong, so using pattern matching is a good idea:
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
    let created_doc = rspaste::api::post("content"); // post document
    let deleted_doc = rspaste::api::delete("key", "secret"); // delete document
}
```
```