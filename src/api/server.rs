use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use futures::{stream::Stream, StreamExt};
use tokio::sync::Mutex;

use crate::{display, telemetry::sys::SystemInfoService};

pub async fn get_event_string(
    is_start: bool,
    system_info_service: Arc<Mutex<dyn SystemInfoService + Sync + Send>>,
) -> Event {
    let info = system_info_service.lock().await.get_info();

    let disp = display::graph::render(info);

    // render(info) generates a string that clears out the previous output
    // On the first run where there is no previous output, it will clear the unrelated lines above
    // hence we need to pad the terminal with newlines on the first run 
    if is_start {
        let newlines = "\n".repeat(disp.split('\n').count() + 1) + &disp;
        return Event::default().data(newlines);
    }

    Event::default().data(disp)
}

pub async fn graph_sse_handler(
    system_info_service: State<Arc<Mutex<dyn SystemInfoService + Sync + Send>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut first_start = true;

    let stream =
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .then(move |_| {
                let cloned = system_info_service.0.clone();
                let event = async move { get_event_string(first_start, cloned).await };
                first_start = false;

                event
            })
            .map(Ok);

    Sse::new(stream)
}
