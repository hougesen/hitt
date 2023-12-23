use hitt_parser::HittRequest;

pub struct HittResponse {
    pub url: String,
    pub method: String,
    pub status_code: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
    pub http_version: http::version::Version,
    pub duration: std::time::Duration,
}

pub async fn send_request(
    http_client: &reqwest::Client,
    input: &HittRequest,
) -> Result<HittResponse, reqwest::Error> {
    let url = input.uri.to_string();
    let method = input.method.to_string();

    let req = http_client
        .request(input.method.clone(), &url)
        .headers(input.headers.clone())
        .body(input.body.clone().unwrap_or_default())
        .version(input.http_version.unwrap_or(reqwest::Version::HTTP_11))
        .build()?;

    // TODO: implement more precise benchmark?
    let start = std::time::Instant::now();
    let res = http_client.execute(req).await?;
    let duration = start.elapsed();

    let status_code = res.status();
    let headers = res.headers().to_owned();

    let http_version = res.version();

    let body = res.text().await.unwrap_or_default();

    Ok(HittResponse {
        url,
        method,
        status_code,
        headers,
        body,
        http_version,
        duration,
    })
}
