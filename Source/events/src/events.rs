pub use rudimentary::*;

use artifacts::Artifact;

use rudimentary_derive::ConceptSetup;
use uuid::Uuid;

#[derive(ConceptSetup)]
pub struct EventSourceId {
    value: Uuid,
}

impl Concept<Uuid> for EventSourceId {
    fn get_value(&self) -> Uuid {
        self.value
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}

impl EventSourceId {
    pub fn create() -> EventSourceId {
        EventSourceId::new(Uuid::new_v4())
    }
}

#[derive(PartialEq, Clone)]
pub struct Event {
    event_source: EventSourceId,
    event_type: Artifact,
    public: bool,
    content: String,
}

#[derive(Clone)]
pub struct UncommittedAggregateEvents {
    expected_aggregate_root_version: crate::AggregateRootVersion,
    event_source: EventSourceId,
    aggregate_root: Artifact,
    events: Vec<Event>,
}
