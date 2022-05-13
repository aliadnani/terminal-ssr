use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    response::{sse::Event, Sse},
    Extension,
};
use futures::stream::{self, Stream};
use sysinfo::System;
use tokio_stream::StreamExt as _;

use crate::{display, telemetry::sys::get_info};

pub async fn graph_sse_handler(
    state: Extension<Arc<Mutex<System>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut is_start = true;

    let stream = stream::repeat_with(move || {
        let mut system = state.0.try_lock().unwrap();
        let info = get_info(&mut system);

        let disp = display::graph::render(info);

        // Need to add some newlines on start - otherwise previous terminal buffer will be overwritten
        if is_start {
            is_start = false;
            let newlines = "\n".repeat(disp.split('\n').count() - 1);
            return Event::default().data(newlines);
        }

        Event::default().data(disp)
    })
    .map(Ok)
    .throttle(Duration::from_millis(1250));

    Sse::new(stream)
}
