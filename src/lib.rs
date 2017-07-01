// Based on http://www.opengeospatial.org/standards/wms

extern crate futures;
extern crate hyper;
#[macro_use] extern crate serde_derive;
extern crate tokio_core;
extern crate url;

pub mod capabilities;

use futures::Future;
use self::hyper::Client as HyperClient;
use self::hyper::client::{HttpConnector};

/// WMS Client
pub struct Client<'a> {
    /// WMS service URL
    base_url: &'a str,
    /// Tokio Core
    core: tokio_core::reactor::Core,
    /// Hyper HTTP Client
    hyper: HyperClient<HttpConnector>
}

/// WMS version supported by client
const VERSION: &str = "1.3.0";

impl<'a> Client<'a> {
    /// Create a new WMS Client
    /// # Example
    ///
    /// ```
    /// let mut client = wms::Client::new("http://sampleserver/wms");
    /// ```
    pub fn new(base_url: &'a str) -> Self {
        let core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();

        let hyper = HyperClient::new(&handle);

        Client { base_url, core, hyper }
    }

    /// Get WMS service capabilities
    /// # Example
    ///
    /// ```
    /// let mut client = wms::Client::new("http://sampleserver/wms");
    ///
    /// client.get_capabilities(|res| {
    ///    println!("Response status: {}", res.status());
    /// });
    /// ```
    pub fn get_capabilities<F>(&mut self, callback: F) where F: Fn(self::hyper::Response) {
        let url = url::Url::parse_with_params(self.base_url, &[("version", VERSION), ("service", "WMS"), ("request", "GetCapabilities")]).unwrap();
        let url = url.into_string().parse::<hyper::Uri>().unwrap();

        let work = self.hyper.get(url).map(callback);
        self.core.run(work).unwrap();
    }

    pub fn get_map<F>(&mut self, callback: F) where F: Fn(self::hyper::Response) {
        let url = url::Url::parse_with_params(self.base_url, &[("version", VERSION), ("request", "GetMap")]).unwrap();
        let url = url.into_string().parse::<hyper::Uri>().unwrap();

        let work = self.hyper.get(url).map(callback);
        self.core.run(work).unwrap();
    }
}
