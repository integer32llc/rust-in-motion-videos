#[derive(Debug)]
struct Version {
    id: i32,
    published_by: Option<i32>,
}

impl Version {
    pub fn published_by(&self) -> Option<User> {
        self.published_by.map(|pb| User::find(pb))
    }
}

#[derive(Debug)]
struct User {
    id: i32,
    login: String,
}

impl User {
    fn find(id: i32) -> User {
        // Actual implementation that looks up a user in the database by
        // its ID removed for simplicity
        User {
            id,
            login: String::from("some_username"),
        }
    }
}

fn main() {
    let v1 = Version {
        id: 1,
        published_by: Some(3),
    };
    println!("version 1 published by user = {:?}", v1.published_by());


    let v2 = Version {
        id: 2,
        published_by: None,
    };
    println!("version 2 published by user = {:?}", v2.published_by());
}
