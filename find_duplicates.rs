fn main() {
    let numbers = vec![1, 2, 3, 2, 4, 5, 3, 6, 1, 1];

    let duplicates = find_duplicated(&numbers);

    println!("{duplicates:#?}");
}

fn find_duplicated(numbers: &[i32]) -> Vec<i32> {
    let mut duplicates = vec![];

    for (index, element) in numbers.iter().enumerate() {
        if numbers[index + 1..].contains(element) && !duplicates.contains(element) {
            duplicates.push(*element)
        }
    }

    duplicates
}
