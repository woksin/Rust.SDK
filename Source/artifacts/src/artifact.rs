use std::{fmt, fmt::Display, fmt::Formatter};

pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;
use uuid::Uuid;

#[derive(ConceptSetup)]
pub struct ArtifactId {
    value: Uuid,
}

impl Concept<Uuid> for ArtifactId {
    fn get_value(&self) -> Uuid {
        self.value
    }
    fn borrow_value(&self) -> &Uuid {
        &self.value
    }
}

#[derive(ConceptSetup)]
pub struct ArtifactGeneration {
    value: u32,
}

impl Concept<u32> for ArtifactGeneration {
    fn get_value(&self) -> u32 {
        self.value
    }
    fn borrow_value(&self) -> &u32 {
        &self.value
    }
}

#[derive(PartialEq, Clone)]
pub struct Artifact {
    id: ArtifactId,
    generation: ArtifactGeneration,
}

impl Display for Artifact {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Artifact({}, {})", self.id, self.generation)
    }
}
