pub mod models;
pub mod client;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_polar_client() {
        let _ = client::polar::Polar::new( &env::var("POLAR_SANDBOX_API_KEY").unwrap(), client::polar::Server::Sandbox);
    }
}
