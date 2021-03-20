use rudimentary::Concept;
use rudimentary_derive::ConceptSetup;

#[derive(ConceptSetup)]
pub struct AggregateRootVersion {
    value: u32,
}

impl Concept<u32> for AggregateRootVersion {
    fn get_value(&self) -> u32 {
        self.value
    }
    fn borrow_value(&self) -> &u32 {
        &self.value
    }
}
