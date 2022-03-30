use std::{thread, time};
struct Paragraph {
    text_sequence: String,
}

impl Paragraph {
    fn new(input_string: String) -> Self {
        Self {
            text_sequence: input_string,
        }
    }
    fn set_String(mut self, input_string: &str) {
        self.text_sequence = input_string.to_string();
    }

    fn type_sequence(&self) {
        for character in self.text_sequence.chars() {
            let sleep_time = time::Duration::from_millis(20);
            winput::send(character);
            thread::sleep(sleep_time);
        }
    }
}
fn read_from_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}
fn main() {
    let initial_file_path = r"C:\dev_projects\Legit-typist\initial-string.txt";
    let following_file_path = r"C:\dev_projects\Legit-typist\string.txt";

    let mut typing_string = read_from_file(&initial_file_path);
    let typing_paragraph: Paragraph = Paragraph::new(typing_string.to_owned());
    typing_paragraph.type_sequence();
    loop {
        let loop_typing_string = Paragraph::new(typing_string.to_owned());
        typing_string = read_from_file(&following_file_path);
        loop_typing_string.set_String(&typing_string);
        loop_typing_string.type_sequence();
    }
}
