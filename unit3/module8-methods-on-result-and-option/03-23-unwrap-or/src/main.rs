extern crate serde; // 1.0.78
extern crate serde_json; // 1.0.27

#[macro_use]
extern crate serde_derive; //1.0.78

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Margaret Hamilton",
    }"#);

    let first_inner = first.unwrap_or(Person { name: String::from("unknown") });

    println!("first's name = {:?}", first_inner.name);
}
