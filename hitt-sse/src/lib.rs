use futures::StreamExt;
use reqwest_eventsource::EventSource;
use tokio::sync::mpsc::{UnboundedSender, error::SendError};

pub type Error = reqwest_eventsource::Error;

#[derive(Debug)]
pub enum Event {
    Open,
    Message(String),
    Error(Error),
}

pub async fn start_sse(
    url: reqwest::Url,
    tx: UnboundedSender<Event>,
) -> Result<(), SendError<Event>> {
    let mut ev = EventSource::get(url);

    while let Some(event) = ev.next().await {
        match event {
            Ok(reqwest_eventsource::Event::Open) => {
                tx.send(Event::Open)?;
            }
            Ok(reqwest_eventsource::Event::Message(m)) => {
                tx.send(Event::Message(m.data))?;
            }
            Err(error) => {
                tx.send(Event::Error(error))?;

                ev.close();
            }
        }
    }

    Ok(())
}
