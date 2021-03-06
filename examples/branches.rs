extern crate env_logger;
extern crate hyper;
extern crate hyper_native_tls;
extern crate hubcaps;

use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use hubcaps::{Credentials, Github};
use std::env;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let github =
                Github::new(format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                            Client::with_connector(HttpsConnector::new(NativeTlsClient::new()
                                                                           .unwrap())),
                            Credentials::Token(token));
            for branch in github
                    .repo("softprops", "hubcaps")
                    .branches()
                    .iter()
                    .unwrap() {
                println!("{:#?}", branch)
            }

            match github
                      .repo("softprops", "hubcaps")
                      .branches()
                      .get("master") {
                Ok(branch) => println!("{:#?}", branch),
                Err(err) => println!("err {:#?}", err),
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
