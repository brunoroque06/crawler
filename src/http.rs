pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Result<Self, String> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("feed/0.1")
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { client })
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

pub fn build_url(url: &str, params: &[(&str, &str)]) -> Result<String, String> {
    let mut url = reqwest::Url::parse(url).map_err(|e| e.to_string())?;

    if !params.is_empty() {
        url.query_pairs_mut().extend_pairs(params);
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_url() {
        assert_eq!(
            build_url("invalid-url", &[]),
            Err("relative URL without a base".to_owned())
        );
    }

    const URL: &str = "https://brunoroque06.github.io/";

    #[test]
    fn valid_url() {
        assert_eq!(build_url(URL, &[]), Ok(URL.to_owned()));
    }

    #[test]
    fn valid_url_params() {
        assert_eq!(
            build_url(URL, &[("hl", "en-US"), ("q", "two words")]),
            Ok("https://brunoroque06.github.io/?hl=en-US&q=two+words".to_owned())
        );
    }
}
