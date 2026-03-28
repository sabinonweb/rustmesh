use tracing::{info, span, Level};
use tracing_subscriber::fmt;

fn main() {
    // Initialize tracing subscriber (prints to console)
    fmt::init();

    // Create a span for a task
    let main_span = span!(Level::INFO, "processing_task");
    let _enter = main_span.enter(); // Enter the span (it’s active now)

    info!("Starting the task");
    // Simulate some work, like handling a request
    info!("Doing some work inside the task");
    info!("Finishing the task");
}
