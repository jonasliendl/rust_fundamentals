#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
}

fn full_name(person: &Person) -> String {
    format!("{} {}", person.first_name, person.last_name)
}

fn main() {
    println!("Week 3 of Rust Fundamentals - Labor 01");
    let person = &Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "example@example.de".to_string(),
        phone_number: "+49123456789".to_string(),
    };

    println!("{}", full_name(&person));
}
