extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use futures::{Future, Stream};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde_json::Value as JsValue;
use std::cell::RefCell;
use std::io;
use tokio_core::reactor::Core;
use url::Url;

type HttpsClient = Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

#[derive(Deserialize, Debug)]
pub struct ZipCode {
    #[serde(rename(deserialize = "cep"))]
    pub zip: String,
    #[serde(rename(deserialize = "logradouro"))]
    pub address: String,
    #[serde(rename(deserialize = "complemento"))]
    pub complement: String,
    #[serde(rename(deserialize = "bairro"))]
    pub neighborhood: String,
    #[serde(rename(deserialize = "localidade"))]
    pub city: String,
    #[serde(rename(deserialize = "uf"))]
    pub state_initials: String,
    #[serde(rename(deserialize = "unidade"))]
    pub unit: String,
    pub ibge: String,
    pub gia: String
}

fn to_io_error<E>(err: E) -> io::Error
where
    E : Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

struct UriMaker {
    api_base: String,
}

impl UriMaker {
    pub fn new(api_base: String) -> UriMaker {
        UriMaker {
            api_base,
        }
    }

    fn url_to_uri(url: &url::Url) -> Uri {
        url.as_str().parse().unwrap()
    }

    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let url = Url::parse(&self.api_base)?.join(path)?;
        Ok(url)
    }

    pub fn get_by_zipcode(&self, zip_code: &str) -> Uri {
        let url = self.build_url(format!("ws/{}/json", zip_code).as_str()).unwrap();
        Self::url_to_uri(&url)
    }

    pub fn get_by_address(&self, state_initials: &str, city: &str, address: &str) -> Uri {
        let url = self.build_url(format!("ws/{}/{}/{}/json", state_initials, city, address).as_str()).unwrap();
        Self::url_to_uri(&url)
    }
}

pub struct ViaCepClient {
    uri_maker: UriMaker,
    core: RefCell<Core>,
    http: HttpsClient,
}

impl ViaCepClient {
    pub fn new() -> ViaCepClient {
        let core = Core::new().unwrap();
        let http = {
            let handle = core.handle();
            let connector = HttpsConnector::new(4, &handle).unwrap();
            Client::configure().connector(connector).build(&handle)
        };
        let uri_maker = UriMaker::new("https://viacep.com.br".to_owned(),);
        ViaCepClient {
            uri_maker,
            core: RefCell::new(core),
            http,
        }
    }

    fn get_json(&self, uri: hyper::Uri) -> Box<dyn Future<Item = JsValue, Error = io::Error>> {
        debug!("GET {}", uri);
        let f = self.http
            .get(uri)
            .and_then(|res| {
                debug!("Response: {}", res.status());
                res.body().concat2().and_then(move |body| {
                    let value: serde_json::Value = 
                        serde_json::from_slice(&body).map_err(to_io_error)?;

                        Ok(value)
                })
            })
            .map_err(to_io_error);        
        Box::new(f)
    }

    pub fn get_zipcode(&self, zip_code: &str) -> Result<Option<ZipCode>, io::Error> {
        let uri = self.uri_maker.get_by_zipcode(zip_code);
        let work = self.get_json(uri).and_then(|value| {
            let wrapper: Option<ZipCode> = 
                serde_json::from_value(value).map_err(to_io_error)?;
            Ok(wrapper)
        });
        self.core.borrow_mut().run(work)
    }

    pub fn search(&self, state_initials: &str, city: &str, address: &str) -> Result<Option<Vec<ZipCode>>, io::Error>{ 
        let uri = self.uri_maker.get_by_address(state_initials, city, address);
        let work = self.get_json(uri).and_then(|value| {
            let wrapper: Option<Vec<ZipCode>> = 
                serde_json::from_value(value).map_err(to_io_error)?;
            Ok(wrapper)
        });
        self.core.borrow_mut().run(work)
    }
}