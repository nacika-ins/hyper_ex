# hyper-ex

Add the cookie-sync function to hyper

## WARNING!!
This crate please use for development and verification.
Security risk many of the Cookie, this crate does not guarantee the safety of the security.

## Usage

It will `git clone` in any directory.

```bash
git clone https://github.com/nacika-ins/hyper_ex.git /path/to/hyper_ex
```

Add this to your `Cargo.toml`:

```toml
[dependencies.hyper_ex]
path = "/path/to/hyper_ex"
```

## Example

### Get request

```rust
extern crate hyper_ex;
use hyper_ex::Client;

fn main() {
    let mut client = Client::new();
    client.change_useragent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_0) AppleWebKit/537.36 \
                             (KHTML, like Gecko) Chrome/46.0.2490.80 Safari/537.36"
                                .to_owned());
    let body = client.get_body("http://google.com").unwrap();
    println!("{}", body);
}

```

### Post request

```rust
extern crate hyper_ex;
use hyper_ex::Client;

fn main() {
    let mut client = Client::new();
    client.change_useragent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_0) AppleWebKit/537.36 \
                             (KHTML, like Gecko) Chrome/46.0.2490.80 Safari/537.36"
                                .to_owned());
    let body = client.post_body("http://google.com", "data").unwrap();
    println!("{}", body);
}

```