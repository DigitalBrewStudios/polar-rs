use super::polar::Polar;

pub struct Products<'a>{
    client: &'a Polar,
}

impl<'a> Products<'a> {
    pub async fn list_products(&self) {
        let body = self.client.client.get(self.client.server.url().join("v1/products").unwrap()).send().await.unwrap().text().await.unwrap();
        println!("{body:?}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::*;

    #[tokio::test]
    async fn test_list_products() {
        let client = client::polar::Polar::new( &env::var("POLAR_SANDBOX_API_KEY").unwrap(), client::polar::Server::Sandbox);
        let products = Products{client: &client};
        products.list_products().await;
    }
}
