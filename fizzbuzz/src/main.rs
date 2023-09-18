fn fizz_buzz(number: i32) -> String {
    let mut result = String::new();

    for i in 1..=number {
        if i % 3 == 0 && i % 5 == 0 {
            result.push_str("FizzBuzz ");
        } else if i % 3 == 0 {
            result.push_str("Fizz ");
        } else if i % 5 == 0 {
            result.push_str("Buzz ");
        } else {
            result += &(i.to_string() + " ");
        }
    }

    result
}

fn main() {
    let mut num = 10;
    println!("{}", fizz_buzz(num));
    num = 20;
    println!("{}", fizz_buzz(num));
    num = 50;
    println!("{}", fizz_buzz(num));
    num = 100;
    println!("{}", fizz_buzz(num));
}
