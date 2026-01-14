use crate::error::Error;

pub(crate) fn parse_base_url(value: &str) -> Result<url::Url, Error> {
    let mut url = url::Url::parse(value)
        .map_err(|e| Error::invalid_config("invalid base url", Some(Box::new(e))))?;
    if url.query().is_some() || url.fragment().is_some() {
        return Err(Error::invalid_config(
            "base url must not include query or fragment",
            None,
        ));
    }

    if !url.path().ends_with('/') {
        let new_path = format!("{}/", url.path().trim_end_matches('/'));
        url.set_path(&new_path);
    }

    Ok(url)
}

pub(crate) fn endpoint(base_url: &url::Url, segments: &[&str]) -> Result<url::Url, Error> {
    let mut url = base_url.clone();
    {
        let mut path = url
            .path_segments_mut()
            .map_err(|_| Error::invalid_config("base url must be hierarchical", None))?;
        path.pop_if_empty();
        for segment in segments {
            path.push(segment);
        }
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_base_url_requires_no_query_or_fragment() {
        assert!(parse_base_url("https://example.com/?a=b").is_err());
        assert!(parse_base_url("https://example.com/#frag").is_err());
    }

    #[test]
    fn parse_base_url_normalizes_trailing_slash() {
        let url = parse_base_url("https://example.com/api").unwrap();
        assert_eq!(url.as_str(), "https://example.com/api/");
    }

    #[test]
    fn endpoint_pushes_encoded_path_segments() {
        let base = parse_base_url("https://example.com/api").unwrap();
        let url = endpoint(&base, &["a", "b c", "d/e"]).unwrap();
        assert_eq!(url.path(), "/api/a/b%20c/d%2Fe");
    }
}
