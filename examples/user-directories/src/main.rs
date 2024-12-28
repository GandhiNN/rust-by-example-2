use directories::UserDirs;

fn main() {
    let user_dirs = UserDirs::new().expect("Could not resolve user HOME.");
    let home_dir = user_dirs.home_dir();
    println!("{}", home_dir.display());
}
