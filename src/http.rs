pub fn build_url(url: &str, params: Vec<(&str, &str)>) -> Result<String, String> {
    let mut url = reqwest::Url::parse(url).map_err(|e| e.to_string())?;

    if !params.is_empty() {
        for (k, v) in params {
            url.query_pairs_mut().append_pair(k, v);
        }
    }

    Ok(url.to_string())
}

pub fn get(url: &str) -> Result<String, String> {
    reqwest::blocking::Client::builder()
        .user_agent("feed/0.1")
        .build()
        .map_err(|e| e.to_string())?
        .get(url)
        .send()
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
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
