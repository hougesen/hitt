use hitt_formatter::ContentType;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    config::SSECommandArguments,
    error::HittCliError,
    terminal::{self, sse::print_sse_connection_open},
};

#[inline]
pub async fn sse_command<W: std::io::Write + Send>(
    term: &mut W,
    args: &SSECommandArguments,
) -> Result<(), HittCliError> {
    let sse_url =
        reqwest::Url::parse(&args.url).map_err(|_| HittCliError::SSEParseUrl(args.url.clone()))?;

    let (tx, mut rx) = unbounded_channel::<hitt_sse::Event>();

    let _sse_listener = tokio::spawn(async move { hitt_sse::start_sse(sse_url, tx).await });

    while let Some(sse_event) = rx.recv().await {
        match sse_event {
            hitt_sse::Event::Open => {
                print_sse_connection_open(term, &args.url).map_err(HittCliError::Io)
            }
            hitt_sse::Event::Message(message) => {
                terminal::body::print_body(term, &message, ContentType::Unknown, true)
                    .map_err(HittCliError::Io)
            }
            hitt_sse::Event::Error(error) => Err(HittCliError::SSEError(error)),
        }?;
    }

    Ok(())
}
