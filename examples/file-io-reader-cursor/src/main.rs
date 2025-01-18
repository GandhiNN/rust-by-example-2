// use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
// use std::process::exit;

fn read_file(file: impl Read) -> Vec<String> {
    BufReader::new(file).lines().map_while(Result::ok).collect()
}

fn main() {
    let poem = "I have a little shadow that goes in and out with me,
            And what can be the use of him is more than I can see.
            He is very, very like me from the heels up to the head;
            And I see him jump before me, when I jump into my bed.

            The funniest thing about him is the way he likes to grow -
            Not at all like proper children, which is always very slow;
            For he sometimes shoots up taller like an india-rubber ball,
            And he sometimes gets so little that there’s none of him at all.";

    let mock_file = Cursor::new(poem);

    // let lines = match File::open(mock_file) {
    //     Ok(file) => read_file(file),
    //     Err(e) => {
    //         eprintln!("Error opening {mock_file}: {e}");
    //         exit(1);
    //     }
    // };
    let lines = read_file(mock_file);
    println!("{:?}", lines);
}
