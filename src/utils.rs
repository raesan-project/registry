// this is used to convert the name of a/an exam/subject/chapter into a proper slug to be used in
// the URL and we also have to replace the "&" character with the actual "and" because for some
// reason it appears in the registry alot
pub fn path_to_slug(input: String) -> String {
    let mut output: String = String::new();
    let mut input_iter = input.chars().peekable();
    while let Some(curr_char) = input_iter.peek() {
        match curr_char {
            '&' => {
                output.push_str("and");
                input_iter.next();
            }
            _ => {
                if curr_char.is_whitespace() {
                    output.push('-');
                    input_iter.next();
                } else {
                    output.push(input_iter.next().unwrap());
                }
            }
        }
    }
    return output.to_lowercase();
}
