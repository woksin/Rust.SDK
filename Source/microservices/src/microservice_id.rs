
pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;
use uuid::Uuid;

#[derive(ConceptSetup)]
pub struct MicroserviceId {
    value: Uuid,
}

impl Concept<Uuid> for MicroserviceId {
    fn get_value(&self) -> Uuid {
        self.value.clone()
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}
