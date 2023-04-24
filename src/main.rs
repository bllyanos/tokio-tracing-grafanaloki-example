use std::{process, time::Duration};

use tracing::{event, metadata::LevelFilter, span, Level};
use tracing_loki::url::Url;
use tracing_subscriber::{
    fmt::Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), tracing_loki::Error> {
    let (layer, task) = tracing_loki::builder()
        .label("job", "tracer")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(layer)
        .with(Layer::new())
        .init();

    tokio::spawn(task);

    let span = span!(Level::INFO, "my_span", trace_id = 123);
    let _enter = span.enter();
    let child = span!(Level::INFO, "child_span");
    let _enter = child.enter();

    event!(Level::INFO, name = "billy", "hello");

    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
