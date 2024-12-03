use std::vec;

fn main() { 
    let mut s1: String = String::from("alo mundo azul");

    let word = first_word(&s1);

    s1 = String::from("alo2 mundo azul");

    let word2 = first_word(&s1);

    let v = vec![1, 2, 3, 4, 5];
    

    println!("word1: {}, word2: {}", word, v[0]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
