use hitt_parser::HittRequest;

pub struct HittResponse {
    pub url: String,
    pub method: String,
    pub status_code: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
    pub http_version: http::version::Version,
    pub duration: core::time::Duration,
}

#[inline]
pub async fn send_request(
    http_client: &reqwest::Client,
    input: &HittRequest,
    timeout: &Option<core::time::Duration>,
) -> Result<HittResponse, reqwest::Error> {
    let url = input.uri.to_string();

    let mut partial_req = http_client.request(input.method.clone(), &url);

    if let Some(http_version) = input.http_version {
        partial_req = partial_req.version(http_version);
    }

    if !input.headers.is_empty() {
        partial_req = partial_req.headers(input.headers.clone());
    }

    if input.body.is_some() {
        if let Some(body) = input.body.clone() {
            partial_req = partial_req.body(body);
        }
    }

    if timeout.is_some() {
        if let Some(dur) = *timeout {
            partial_req = partial_req.timeout(dur);
        }
    }

    let req = partial_req.build()?;

    // TODO: implement more precise benchmark?
    let start = std::time::Instant::now();
    let res = http_client.execute(req).await?;
    let duration = start.elapsed();

    Ok(HittResponse {
        url,
        method: input.method.to_string(),
        status_code: res.status(),
        headers: res.headers().to_owned(),
        http_version: res.version(),
        duration,
        body: res.text().await.unwrap_or_default(),
    })
}

#[cfg(test)]
mod test_send_request {
    use core::{str::FromStr, time::Duration};

    use http::{HeaderMap, StatusCode};

    use crate::send_request;

    #[tokio::test]
    async fn it_should_return_a_response() {
        let http_client = reqwest::Client::new();

        let timeout = None;

        let uri = http::Uri::from_str("https://dummyjson.com/products/1").unwrap();

        let input = hitt_parser::HittRequest {
            method: http::Method::GET,
            uri: uri.clone(),
            headers: HeaderMap::default(),
            body: None,
            http_version: None,
        };

        let result = send_request(&http_client, &input, &timeout)
            .await
            .expect("it to be successfull");

        assert_eq!(result.url, uri.to_string());

        assert_eq!(result.status_code, StatusCode::OK);
    }

    #[tokio::test]
    async fn timeout_should_work() {
        let http_client = reqwest::Client::new();

        let timeout = Some(Duration::from_millis(5));

        let uri = http::Uri::from_str("https://dummyjson.com/products/1").unwrap();

        let input = hitt_parser::HittRequest {
            method: http::Method::GET,
            uri: uri.clone(),
            headers: HeaderMap::default(),
            body: None,
            http_version: None,
        };

        match send_request(&http_client, &input, &timeout).await {
            Ok(_) => panic!("expected request to timeout"),
            Err(err) => assert!(err.is_timeout()),
        };
    }
}
