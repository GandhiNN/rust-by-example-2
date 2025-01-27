fn basic_ownership_string_types() {
    // Simple string literal - stored in stack
    let greeting = "Hello";

    // String type - stored in heap
    let message = String::from("Hello, world!");

    // This works fine - greeting is copied (it's on the stack)
    let greeting_copy = greeting;
    println!("Original: {}, Copy: {}", greeting, greeting_copy);

    // This moves ownership - message is no longer valid
    let message_moved = message;

    // This would cause a compilation error
    // println!("{}", message);

    println!("Moved message: {}", message_moved);
}

#[derive(Debug)]
struct UserProfile {
    username: String,
    email: String,
    active: bool,
}

fn update_profile(mut profile: UserProfile) -> UserProfile {
    profile.active = true;
    profile // return ownership back
}

fn display_profile(profile: &UserProfile) {
    println!("Username: {}", profile.username);
}

fn activate_profile(profile: &mut UserProfile) {
    profile.active = true;
}

fn main() {
    // Basic ownership using string types
    basic_ownership_string_types();

    // Understanding move semantics
    let user = UserProfile {
        username: String::from("rustacean"),
        email: String::from("rust@example.com"),
        active: false,
    };

    // Ownership moves to the function
    // i.e. `user` is "consumed" by the function `update_profile()`
    let updated_user = update_profile(user);

    // This would fail:
    // println!("Original user: {:?}", user);

    println!("Updated user: {:?}", updated_user);

    // We have to recreate the `user`
    let mut user = UserProfile {
        username: String::from("rustacean"),
        email: String::from("rust@example.com"),
        active: false,
    };

    // Immutable borrow
    display_profile(&user);

    // Mutable borrow
    activate_profile(&mut user);

    println!("User after activation: {:?}", user);
}
