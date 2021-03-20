// From https://github.com/PrismaPhonic/domain_patterns/blob/master/domain_derive/src/value_object.rs
pub use std::convert::{From, TryFrom};
pub use std::fmt::{Debug, Display};

/// Represents a the concept of a specific value type. Like a strongly-typed version if the underlying type.
pub trait Concept<TValue>: Clone + PartialEq + Display + Debug {
    /// Returns the value that is represented as an owned type.
    fn get_value(&self) -> TValue;

    /// Returns the value that is represented as a borrowed type.
    fn borrow_value(&self) -> &TValue;
}

pub trait ConceptAs<TValue>: Concept<TValue> + From<TValue> {
    fn new(value: TValue) -> Self;
}

pub trait ValidatedConceptAs<TValue>: Concept<TValue> + TryFrom<TValue> {
    type ValidationError;
    fn new(value: TValue) -> Result<Self, Self::ValidationError>;
    fn validate(value: &TValue) -> Result<(), Self::ValidationError>;
}
