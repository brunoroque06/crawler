pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Result<Client, String> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("feed/0.1")
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Client { client })
    }

    pub fn get(&self, url: &str) -> Result<String, String> {
        self.client
            .get(url)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }
}

pub fn build_url(url: &str, params: Vec<(&str, &str)>) -> Result<String, String> {
    let mut url = reqwest::Url::parse(url).map_err(|e| e.to_string())?;

    if !params.is_empty() {
        for (k, v) in params {
            url.query_pairs_mut().append_pair(k, v);
        }
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_url() {
        assert_eq!(
            build_url("invalid-url", vec![]),
            Err("relative URL without a base".to_string())
        );
    }

    const URL: &str = "https://brunoroque06.github.io/";

    #[test]
    fn valid_url() {
        assert_eq!(build_url(URL, vec![]), Ok(URL.to_string()));
    }

    #[test]
    fn valid_url_params() {
        assert_eq!(
            build_url(URL, vec![("hl", "en-US"), ("q", "two words")]),
            Ok("https://brunoroque06.github.io/?hl=en-US&q=two+words".to_string())
        );
    }
}
