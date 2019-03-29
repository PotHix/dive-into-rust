struct Person {
    name: String,
    marital_status: Status,
    school_degree: SchoolDegree
}

enum Status {
    Single,
    Married(String),
}

// Exercise 1: Define SchoolDegree

impl Person {
    fn partner(&self) -> Option<String> {
        match self.marital_status {
            Status::Married(ref partner) => Some(partner.clone()),
            _ => None
        }
    }

    // Exercise 2: Define a method that returns true if the person completed high school
}

fn main() {
    // Exercise 3: Build a married person
}
