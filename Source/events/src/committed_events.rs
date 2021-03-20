use rudimentary::Concept;
use rudimentary_derive::ConceptSetup;

use artifacts::Artifact;
use chrono::{DateTime, Utc};
use execution::ExecutionContext;
use std::vec::Vec;

#[derive(ConceptSetup)]
pub struct EventLogSequenceNumber {
    value: u32,
}

impl Concept<u32> for EventLogSequenceNumber {
    fn get_value(&self) -> u32 {
        self.value
    }
    fn borrow_value(&self) -> &u32 {
        &self.value
    }
}

#[derive(PartialEq, Clone)]
pub struct CommittedEvent {
    event_log_sequence_number: EventLogSequenceNumber,
    occurred: DateTime<Utc>,
    execution_context: ExecutionContext,
    event: crate::Event,
}

#[derive(PartialEq, Clone)]
pub struct CommittedAggregateEvent {
    aggregate_root: Artifact,
    aggregate_root_version: crate::AggregateRootVersion,
    event: CommittedEvent,
}

#[derive(Clone)]
pub struct CommittedAggregateEvents {
    event_source: crate::EventSourceId,
    aggregate_root: Artifact,
    events: Vec<CommittedAggregateEvent>,
}
