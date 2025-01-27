#[derive(Debug)]
struct Session {
    user_id: String,
    login_time: String,
    data: Vec<String>,
}

impl Session {
    fn new(user_id: &str) -> Session {
        Session {
            user_id: String::from(user_id),
            login_time: String::from("2024-01-21"),
            data: Vec::new(),
        }
    }

    fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    fn clear_data(&mut self) {
        self.data.clear();
    }
}

fn process_session(session: &Session) -> usize {
    session.data.len()
}

fn main() {
    let mut user_session = Session::new("user123");

    // Add some data
    user_session.add_data(String::from("page_view"));
    user_session.add_data(String::from("button_click"));

    // Process session (immutable borrow)
    let data_count = process_session(&user_session);
    println!("Session data count: {}", data_count);

    // Clear data (mutable borrow)
    user_session.clear_data();

    println!("Final session state: {:?}", user_session);
}
