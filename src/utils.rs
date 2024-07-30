pub(crate) fn extract_whitespace(s: &str) -> (&str, &str){
    take_while(|c| c == ' ', s)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str){
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str){
    match &s[0..1]{
        "+" | "-" | "*" | "/" => {}
        _ => panic!("Bad operator!")
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn _extract_ident(s: &str) -> (&str, &str){
    take_while(|c| c.is_ascii_alphabetic(), s)
}



pub(crate) fn take_while(accept:impl Fn(char) -> bool, s:&str) -> (&str, &str){
    let extracted_end = s
    .char_indices()
    .find_map(|(idx, c)| if accept(c){None} else {Some(idx)} )
    .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    } 
}