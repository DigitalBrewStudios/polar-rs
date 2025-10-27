use reqwest::{header, Client, Url};

#[derive(Debug, Clone)]
pub enum Server {
    Production,
    Sandbox,
    SelfHosted(Url),
}

impl Server {
    pub fn url(&self) -> Url {
        match self {
            Self::Production => Url::parse("https://api.polar.sh").unwrap(),
            Self::Sandbox => Url::parse("https://sandbox-api.polar.sh").unwrap(),
            Self::SelfHosted(url) => url.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Polar {
    pub client: Client,
    pub server: Server,
}

impl Polar {
    pub fn new(access_token: &str, server: Server) -> Self {
        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(format!("Bearer {access_token}").as_ref()).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();
        Self {
            client,
            server: server,
        }
    }

    pub fn list_products() {}
}

// impl Products {
//     pub fn get() {}
//     pub fn list() {}
//     pub fn create() {}
//     pub fn update() {}
//     pub fn update_benefits() {}
// }
