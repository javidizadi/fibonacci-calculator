use dialoguer::{theme::ColorfulTheme, Input};
use num_bigint::BigUint;
use num_traits::{One, Zero};
fn main() {
    let nth_of_sequence = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter N")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.trim().parse::<u16>() {
                Ok(_) => Ok(()),
                Err(_) => Err("Please enter number in range (0-65535)."),
            }
        })
        .interact()
        .unwrap();
    let nth_of_sequence: u16 = nth_of_sequence.trim().parse().unwrap();
    let result = fib(nth_of_sequence);
    println!("{}", result);
}
fn fib(n: u16) -> BigUint {
    let mut sequence: [BigUint; 2] = [Zero::zero(), One::one()];
    if n >= 2 {
        let mut new_member: BigUint;
        let mut loop_is_odd: bool = true;
        for _ in 0..(n - 1) {
            new_member = &sequence[0] + &sequence[1];
            if loop_is_odd {
                sequence[0] = new_member;
            } else {
                sequence[1] = new_member;
            }
            loop_is_odd = !loop_is_odd;
        }
    }
    let result_index = (n % 2) as usize;
    sequence[result_index].to_owned()
}
