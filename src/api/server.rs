use std::{
    convert::Infallible,
    time::Duration, sync::Arc,
};

use axum::{
    response::{sse::Event, Sse},
    Extension,
};
use futures::stream::{self, Stream};
use parking_lot::Mutex;
use sysinfo::{System};
use tokio_stream::StreamExt as _;

use crate::{
    display,
    telemetry::sys::{get_info},
};

pub async fn graph_sse_handler(
    state: Extension<Arc<Mutex<System>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut is_start = true;
    let stream = stream::repeat_with(move || {
        if is_start {
            is_start = false;
            return Event::default().data("\n\n\n\n\n")
        }
        let mut system = state.0.try_lock().unwrap();

        let info = get_info(&mut system);

        Event::default().data(display::graph::render(info))
    })
    .map(Ok)
    .throttle(Duration::from_millis(2500));

    Sse::new(stream)
}
