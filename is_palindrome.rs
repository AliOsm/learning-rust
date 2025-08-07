fn main() {
    let is_palindrome_with_string = is_palindrome_string_rev(&String::from("123454321"));
    let is_palindrome_with_str = is_palindrome_string_rev("123454321");

    println!("{is_palindrome_with_string}");
    println!("{is_palindrome_with_str}");
}

fn is_palindrome_string_rev(string: &str) -> bool {
    string == string.chars().rev().collect::<String>()
}

fn is_palindrome_vector_rev(string: &str) -> bool {
    let characters: Vec<char> = string.chars().collect();

    characters.iter().eq(characters.iter().rev())
}

fn is_palindrome_n(string: &str) -> bool {
    let characters: Vec<char> = string.chars().collect();
    let mut i = 0;
    let mut j = string.len() - 1;

    while i < j { // O(N)
        if characters[i] == characters[j] {
            i += 1;
            j -= 1;
        } else {
            return false;
        }
    }

    true
}

fn is_palindrome_n_square(string: &str) -> bool {
    let mut i = 0;
    let mut j = string.len() - 1;

    while i < j { // O(N)
        if string.chars().nth(i).unwrap() == string.chars().nth(j).unwrap() { // O(N)
            i += 1;
            j -= 1;
        } else {
            return false;
        }
    }

    true
}
