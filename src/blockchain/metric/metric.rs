/// Simple representation of any blockchain metric data.

use serde::Serialize;

#[derive(
    Debug, Clone,
    Serialize, Deserialize
)]
pub struct Metric<T> where T: Serialize {
    name: String,
    value: T
}

impl<T> Metric<T> where T: Serialize {

    /// Create a new `Metric` instance.
    pub fn new(name: &str, value: T) -> Self
        where T: Serialize
    {
        Metric {
            name: String::from(name),
            value: value
        }
    }
}