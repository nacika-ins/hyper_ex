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
