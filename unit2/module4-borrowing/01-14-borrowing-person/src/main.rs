struct Person {
    name: String,
}

fn congratulate(person: &Person) {
    println!("Congratulations, {}!!!", person.name);
}

fn main() {
    let p = Person {
        name: String::from("Jake"),
    };

    congratulate(&p);
    println!("Can still use p here: {}", p.name);
}
