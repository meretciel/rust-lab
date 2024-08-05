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
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("upper_left", &format!("[{},{}]", self.upper_left.x, self.upper_left.y))?;
        map.serialize_entry("bottom_right", &format!("[{},{}]", self.bottom_right.x, self.bottom_right.y))?;
        map.end()
    }
}


struct RectangleVisitor;

impl RectangleVisitor {
    fn extract_point(s: &str) -> Point {
        let s = s.replace("[", "").replace("]", "");
        let mut parts = s.split(",");
        let x: f64 = parts.next().unwrap().parse().unwrap();
        let y: f64 = parts.next().unwrap().parse().unwrap();

        Point{x, y}
    }
}

impl<'de> Visitor<'de> for RectangleVisitor {
    type Value = Rectangle;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("some error message")
    }


    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let k1 = map.next_key::<String>()?.unwrap();
        let name = map.next_value::<String>()?;

        let k2 = map.next_key::<String>()?.unwrap();
        let upper_left = map.next_value::<String>()?;

        let k3 = map.next_key::<String>()?.unwrap();
        let bottom_right = map.next_value::<String>()?;

        println!("Extracted entries: {k1}={name}, {k2}={upper_left}, {k3}={bottom_right}");
        let upper_left = RectangleVisitor::extract_point(&upper_left);
        let bottom_right = RectangleVisitor::extract_point(&bottom_right);

        Ok(Rectangle{name, upper_left, bottom_right})
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

    let deserialized: Rectangle = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    return Ok(());
}