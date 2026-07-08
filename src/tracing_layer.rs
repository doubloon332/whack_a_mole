// layer to send traces to Debug panel

use crate::message_format::{PanelKind, PanelOp, PanelUpdate, RenderMessage};

use tokio::sync::mpsc;
use tracing_subscriber::Layer;

use chrono::Local;

pub struct TracingLayer {
    event_tx: mpsc::UnboundedSender<RenderMessage>,
}

impl TracingLayer {
    pub fn new(tx: mpsc::UnboundedSender<RenderMessage>) -> Self {
        Self { event_tx: tx }
    }
}

impl<S> Layer<S> for TracingLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = DebugVisitor {
            message: String::from(""),
        };
        event.record(&mut visitor);

        let msg = RenderMessage::Panel(PanelUpdate {
            target: PanelKind::Debug,
            op: PanelOp::Append(format!("{}: {}\n", Local::now().time(), visitor.message)),
        });

        let _ = self.event_tx.send(msg);
    }
}

// "visit" all the values of the event
struct DebugVisitor {
    message: String,
}

impl tracing::field::Visit for DebugVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{value:?}");
        }
    }
}
