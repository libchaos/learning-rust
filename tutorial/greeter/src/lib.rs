//cargo test -- --nocapture 去运行测试
use std::fmt;
use std::result::Result;

fn greet() {
    println!("hello Rust");
}
#[derive(Debug)]
enum Language {
    English,
    Chinese,
}

struct Greeter {
    language: Language,
}

impl fmt::Display for Greeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let greeting = match  self.language {
            Language::English => "hello",
            Language::Chinese => "你好",
        };
        write!(f, "{}, Rust", greeting)
    }
}

impl Greeter {
    fn new() -> Greeter {
        Greeter {
            language: Language::English,
        }
    }
    fn with_language(mut self, language: Language) -> Greeter {
        self.language = language;
        self
    }

    // fn greet(self) {
    //     let greeting = match  self.language {
    //         Language::English => "hello",
    //         Language::Chinese => "你好",
    //     };
    //     println!("{}, Rust", greeting);
    // }
}

fn greet_language(language: String) {
    if language == "english" {
        println!("hello, {}", language);
    } else {
        println!("the language is： {}", language);
    }
}

fn greet_enum(language: Language) {
    match language {
        Language::English => println!("hello, rust"),
        Language::Chinese => println!("你好，生锈的语言"),
        _ => println!("None"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        greet();
        greet_language("中文".into());
        greet_enum(Language::Chinese);

        let greeter = Greeter::new().with_language(Language::Chinese);
        // greeter.greet();

        assert_eq!(format!("{}", greeter), "你好, Rust");
    }
}
