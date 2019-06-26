struct Person {
    name: String,
    middle_name: String,
    last_name: String,
    title: String,
    suffix: String,
    phone_number: String,
    address: String,
    email: String,
    zodiac: String,
    blood_type: String,
}

fn congratulate(person: &Person) {
    println!("Congratulations, {}!!!", person.name);
}

fn main() {
    let p = Person {
        name: String::from("Jake"),
        middle_name: String::from("Unsafe"),
        last_name: String::from("Goulding"),
        title: String::from("Mr."),
        suffix: String::from("IV"),
        phone_number: String::from("555-555-5555"),
        address: String::from("123 Main St"),
        email: String::from("wouldntyouliketoknow@example.com"),
        zodiac: String::from("Aries"),
        blood_type: String::from("O"),
    };

    congratulate(&p);
    println!("Can still use p here: {}", p.name);
}
