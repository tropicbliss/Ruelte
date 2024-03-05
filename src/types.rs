use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Serialize)]
struct MyStruct {
    my_name: String,
    my_age: u32,
}

#[typeshare]
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "content")]
enum MyEnum {
    MyVariant(bool),
    MyOtherVariant,
    MyNumber(u32),
}
