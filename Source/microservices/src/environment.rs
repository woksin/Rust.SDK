
pub use rudimentary::*;
use rudimentary_derive::ConceptSetup;

#[derive(ConceptSetup)]
pub struct Environment {
    value: String,
}

impl Concept<String> for Environment {
    fn get_value(&self) -> String {
        self.value.clone()
    }
    fn borrow_value(&self) -> &String {
        &self.value
    }
}
