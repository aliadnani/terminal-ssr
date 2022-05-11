use std::{
    convert::Infallible,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use axum::{
    response::{sse::Event, Sse},
    Extension,
};
use futures::stream::{self, Stream};
use sysinfo::{ProcessorExt, System, SystemExt};
use tokio_stream::StreamExt as _;

use crate::{
    display,
    telemetry::sys::{get_info, SystemInfo},
};

pub async fn graph_sse_handler(
    state: Extension<Arc<Mutex<System>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(move || {

        let mut system = state.0.lock();
        let system = system.as_deref_mut().unwrap();

        let info = get_info(system);

        Event::default().data(display::graph::render(info))
    })
    .map(Ok)
    .throttle(Duration::from_millis(2500));

    Sse::new(stream)
}
