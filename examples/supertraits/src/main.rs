trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// IMplementing Student requires us to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both
// Programmer and Student. Implemeting CompSciStudent requires us
// to impl both supertraits
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
    fn comp_sci_student_greeting(&self) -> String;
}

struct CollegeStudent {
    name: String,
    university: String,
    fav_language: String,
}

impl Person for CollegeStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CollegeStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CollegeStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CollegeStudent {
    fn git_username(&self) -> String {
        format!("{}-{}", self.name, self.university)
    }
    fn comp_sci_student_greeting(&self) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            self.name(),
            self.university(),
            self.fav_language(),
            self.git_username()
        )
    }
}

fn main() {
    println!("Hello, world!");
    let student = CollegeStudent {
        name: "Gandhi".to_string(),
        university: "ITB".to_string(),
        fav_language: "Rust".to_string(),
    };
    println!("{}", student.comp_sci_student_greeting());
}
