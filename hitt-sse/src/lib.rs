use futures::StreamExt;
use reqwest_eventsource::{Event, EventSource};

pub enum HittSSEEvent {
    Open(String),
    Message(String, eventsource_stream::Event),
    Error(String, reqwest_eventsource::Error),
}

pub async fn start_sse(
    url: reqwest::Url,
    w: std::sync::mpsc::Sender<HittSSEEvent>,
) -> Result<(), std::sync::mpsc::SendError<HittSSEEvent>> {
    let url_string = url.to_string();

    let mut ev = EventSource::get(url);

    while let Some(event) = ev.next().await {
        match event {
            Ok(Event::Open) => w.send(HittSSEEvent::Open(url_string.clone())),
            Ok(Event::Message(m)) => w.send(HittSSEEvent::Message(url_string.clone(), m)),
            Err(error) => w.send(HittSSEEvent::Error(url_string.clone(), error)),
        }?;
    }

    Ok(())
}
