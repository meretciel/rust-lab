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
        // serializer.serialize_str(&format!("name={}", self.name))?;
        // serializer.serialize_struct("upper_left", 2)?;

        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("upper_left", &format!("[{},{}]", self.upper_left.x, self.upper_left.y))?;
        map.serialize_entry("bottom_right", &format!("[{},{}]", self.bottom_right.x, self.bottom_right.y))?;
        map.end()
        //

        // let mut state = serializer.serialize_struct("Rectangle", 1)?;
        // state.serialize_field("upper_left", &self.upper_left.x)?;
        // state.serialize_field("bottom_right", &self.bottom_right.x)?;
        // state.end()
    }
}


struct RectangleVisitor;


impl<'de> Visitor<'de> for RectangleVisitor {
    type Value = Rectangle;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("some error message")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let key = map.next_key::<String>()?.unwrap();
        map.next_value::<String>()?;
        map.next_key::<String>()?.unwrap();
        map.next_value::<String>()?;
        map.next_key::<String>()?.unwrap();
        map.next_value::<String>()?;

        println!("key: {}", key);

        Ok(
        Rectangle{
            name: "test".to_string(),
            upper_left: Point{x: 0., y: 0.},
            bottom_right: Point{x: 0., y: 0.},
        })
    }
}

impl<'de> Deserialize<'de> for Rectangle {
    fn deserialize<D>(deserializer: D) -> Result<Rectangle, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RectangleVisitor)
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

    // let deserialized: Rectangle = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    return Ok(());

}