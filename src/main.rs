struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from_email(email: String) -> Self {
        let list: Vec<&str> = email.split('@').collect();
        Self {
            username: list[0].to_string(),
            email: email.to_string(),
            uri: list[1].to_string(),
            active: true,
        }
    }

    fn update_uri(&mut self, uri: String) {
        self.uri = uri;
    }

}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
    let mut user_from_email = User::from_email(String::from("john@example.com"));
    println!("Hello, {}! uri: {}", user_from_email.username, user_from_email.uri);
    user_from_email.update_uri(String::from("https://google.com"));
    println!("uri: {}", user_from_email.uri);
}
