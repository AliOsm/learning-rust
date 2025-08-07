fn main() {
    let is_prime = sqrt_is_prime(101);

    println!("{is_prime}")
}

fn sqrt_is_prime(number: i32) -> bool {
    match number {
        0 => false,
        1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        n => {
            let sqrt = (n as f64).sqrt() as i32;
    
            for i in (3..=sqrt).step_by(2) {
                if number % i == 0 {
                    return false
                }
            }
            
            true
        }
    }
}
