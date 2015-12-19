use std::io::Read;
use hyper;
use hyper::header::Cookie;
use hyper::header::Headers;
use cookie::Cookie as CookiePair;
use hyper::header::SetCookie;
use hyper::header::UserAgent;
use hyper::error::Error;
use hyper::client::RedirectPolicy;
use url::Url;
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::FORM_URLENCODED_ENCODE_SET;
use hyper::status::StatusCode;

pub fn encode_uri_component(text: &str) -> String {
    utf8_percent_encode(&text, FORM_URLENCODED_ENCODE_SET)
}

pub struct Client {
    client: hyper::Client,
    headers: Headers,
}

impl<'a> Client {
    fn cookie_sync<'b>(&'b mut self, response_headers: &'b mut Headers, domain: &str) {

        let set_cookies = response_headers.iter()
                                          .filter_map(|header| {
                                              if header.is::<SetCookie>() {
                                                  header.value::<SetCookie>()
                                              } else {
                                                  None
                                              }
                                          })
                                          .next();

        let new_cookies = match set_cookies {
            Some(v) => {
                let mut new_cookies: Vec<CookiePair> = vec![];
                let cookies = &self.headers
                                   .iter()
                                   .filter_map(|header| {
                                       if header.is::<Cookie>() {
                                           header.value::<Cookie>()
                                       } else {
                                           None
                                       }
                                   })
                                   .next()
                                   .unwrap();

                for cookie in v.iter() {
                    let mut cp = cookie.clone() as CookiePair;
                    if cp.domain.is_none() {
                        cp.domain = Some(domain.to_string().clone());
                    }
                    new_cookies.push(cp);
                }

                for cookie in cookies.iter() {
                    let name = cookie.name.to_owned();
                    let old_cookie = v.iter().find(|&r| r.name.to_owned() == name);
                    match old_cookie {
                        Some(_) => (),
                        None => new_cookies.push(cookie.clone() as CookiePair),
                    }
                }
                new_cookies
            }
            None => {
                let mut new_cookies: Vec<CookiePair> = vec![];
                let cookies = &self.headers
                                   .iter()
                                   .filter_map(|header| {
                                       if header.is::<Cookie>() {
                                           header.value::<Cookie>()
                                       } else {
                                           None
                                       }
                                   })
                                   .next()
                                   .unwrap();

                for cookie in cookies.iter() {
                    new_cookies.push(cookie.clone() as CookiePair)
                }
                new_cookies
            }
        };
        self.headers.set(Cookie(new_cookies));
    }

    pub fn set_header<'b>(&'b mut self, name: &str, value: &str) {
        self.headers.set_raw(name.to_owned().clone(),
                             vec![value.to_owned().clone().into_bytes()]);
    }

    pub fn new() -> Client {
        let mut headers = Headers::new();
        let useragent = "hyper-ex".to_owned();
        headers.set(UserAgent(useragent));
        headers.set(Cookie(vec![]));
        let mut client = Client {
            client: hyper::Client::new(),
            headers: headers,
        };
        client.client.set_redirect_policy(RedirectPolicy::FollowNone);
        client
    }

    pub fn change_useragent<'b>(&'b mut self, useragent: String) {
        self.headers.set(UserAgent(useragent));
    }

    pub fn get_body<'b>(&'b mut self, url: &'b str) -> Result<String, Error> {

        let orig_url = Url::parse(url).unwrap();
        let domain = orig_url.domain().unwrap();

        let res = self.client
                      .get(url)
                      .headers(self.headers.clone())
                      .send();

        let mut body = String::new();
        match res {
            Ok(mut v) => {

                self.cookie_sync(&mut v.headers, domain.clone());
                v.read_to_string(&mut body).unwrap();

                if v.status == StatusCode::MovedPermanently || v.status == StatusCode::Found {
                    let location = String::from_utf8(v.headers
                                                      .get_raw("Location")
                                                      .unwrap()
                                                      .to_owned()
                                                      .pop()
                                                      .unwrap())
                                       .unwrap();
                    body = self.get_body(&location).unwrap();
                }
                Ok(body)
            }
            Err(e) => Err(e),
        }
    }

    pub fn post_body<'b>(&'b mut self,
                         url: &'b str,
                         request_body: &'b str)
                         -> Result<String, Error> {

        let orig_url = Url::parse(url).unwrap();
        let domain = orig_url.domain().unwrap();

        let res = self.client
                      .post(url)
                      .body(&request_body.to_owned())
                      .headers(self.headers.clone())
                      .send();

        let mut body = String::new();
        match res {
            Ok(mut v) => {
                self.cookie_sync(&mut v.headers, domain.clone());
                v.read_to_string(&mut body).unwrap();
                if v.status == StatusCode::MovedPermanently || v.status == StatusCode::Found {
                    let location = String::from_utf8(v.headers
                                                      .get_raw("Location")
                                                      .unwrap()
                                                      .to_owned()
                                                      .pop()
                                                      .unwrap())
                                       .unwrap();
                    body = self.post_body(&location, request_body).unwrap();
                }
                Ok(body)
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete<'b>(&'b mut self, url: &'b str) -> Result<String, Error> {

        let orig_url = Url::parse(url).unwrap();
        let domain = orig_url.domain().unwrap();

        let res = self.client
                      .delete(url)
                      .headers(self.headers.clone())
                      .send();
        let mut body = String::new();
        match res {
            Ok(mut v) => {
                self.cookie_sync(&mut v.headers, domain.clone());
                v.read_to_string(&mut body).unwrap();
                Ok(body)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_cookies(&self) -> Vec<CookiePair> {
        let cookies = self.headers
                          .iter()
                          .filter_map(|header| {
                              if header.is::<Cookie>() {
                                  header.value::<Cookie>()
                              } else {
                                  None
                              }
                          })
                          .next()
                          .unwrap();
        let clone_cookies: Vec<CookiePair> = cookies.iter().cloned().collect();
        clone_cookies
    }

    pub fn set_cookie<'b>(&'b mut self,
                          name: String,
                          value: String,
                          domain: String,
                          path: String) {
        let mut headers = Headers::new();
        let mut cookie = CookiePair::new(name.to_owned(), value.to_owned());
        cookie.path = Some(path.to_owned());
        cookie.domain = Some(domain.to_owned());
        headers.set(SetCookie(vec![cookie]));
        self.cookie_sync(&mut headers, &domain);
    }
}

#[test]
fn test_client() {
    let mut client = Client::new();
    client.change_useragent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_0) AppleWebKit/537.36 \
                             (KHTML, like Gecko) Chrome/46.0.2490.80 Safari/537.36"
                                .to_owned());
    let body = client.get_body("http://google.com").unwrap();
    println!("{}", body);
    let body = client.post_body("http://google.com", "data").unwrap();
    println!("{}", body);

}
