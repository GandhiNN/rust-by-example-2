/* Remember: whatever we put inside the struct needs to live as long
as the struct itself does */

#[derive(Debug)]
pub struct YoutubeVideo {
    title: String,
    description: String,
}

impl YoutubeVideo {
    // factory is generic over a type that must
    // implement Into<String>
    pub fn new<T>(title: T, description: T) -> Self 
    where T: Into<String> 
    {
        Self {
            title: title.into(),
            description: description.into(),
        }
    }
}

fn main() {
    /* Construct a `String` from an `str` */
    let a_string1 = String::from("a string"); // version 1
    let a_string2: String = From::from("a string"); // version 2

    let the_same_string1: String = "a string".into(); // version 3
    let the_same_string2: String = Into::into("a string"); // version 4

    println!("{}", a_string1);
    println!("{}", a_string2);
    println!("{}", the_same_string1);
    println!("{}", the_same_string2);

    // Version 1
    let video1 = YoutubeVideo::new(
        String::from("best video ever"),
        String::from("The best video you will ever watch"),
    );

    let video2 = YoutubeVideo::new("Best video ever".to_string(), "The best video you will ever watch".to_string())
}
