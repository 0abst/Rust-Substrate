//16 10 2022

/*Exercise taken from:
https://exercism.org/tracks/rust/exercises/armstrong-numbers

Instructions

Reverse a string

For example: input: "cool" output: "looc"*/

fn main() {
    
    let input1 = "foo";

    reverse(input1);

}

pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect::<String>();
}
