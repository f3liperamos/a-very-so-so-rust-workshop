/// Description:
/// Write a function that takes in a string of one or more words, and returns the same string, but with all five or more letter words reversed
///
/// Strings passed in will consist of only letters and spaces.
/// Spaces will be included only when more than one word is present.
///
/// Examples:
/// reverse_words("Hey fellow warriors") => "Hey wollef sroirraw"
/// reverse_words("This is a test") => "This is a test"
/// reverse_words("This is another test") => "This is rehtona test"

pub fn reverse_words(words: &str) -> String {
    todo!()
}

#[test]
fn tests() {
    assert_eq!(reverse_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(reverse_words("This is a test"), "This is a test");
    assert_eq!(
        reverse_words("This is another test"),
        "This is rehtona test"
    );
    assert_eq!(reverse_words("Welcome"), "emocleW");
    assert_eq!(
        reverse_words("You are almost to the last test"),
        "You are tsomla to the last test"
    );
    assert_eq!(
        reverse_words("Just kidding there is still one more"),
        "Just gniddik ereht is llits one more"
    );
    assert_eq!(
        reverse_words("Seriously this is the last one"),
        "ylsuoireS this is the last one"
    );
}
