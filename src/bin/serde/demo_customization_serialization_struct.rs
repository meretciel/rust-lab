use std::fmt;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::{SerializeMap, SerializeStruct};

#[derive(Debug)]
struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    name: String,
    upper_left: Point,
    bottom_right: Point,
}

impl Serialize for Rectangle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut state = serializer.serialize_struct("Rectangle", 1)?;
        state.serialize_field("upper_left", &format!("[{},{}]", self.upper_left.x, self.upper_left.y))?;
        state.serialize_field("bottom_right", &format!("[{},{}]", self.bottom_right.x, self.bottom_right.y))?;
        state.end()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    let rectangle = Rectangle{
        name: "MyRectangle".to_string(),
        upper_left: Point{x: 10., y: 20.},
        bottom_right: Point{x: 40., y: 60.}
    };

    let serialized = serde_json::to_string(&rectangle).unwrap();
    println!("serialized = {}", serialized);

    return Ok(());
}