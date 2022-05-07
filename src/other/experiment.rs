use std::vec::Vec;

fn fizz_buzz(to: usize) -> Vec<String> {
    let mut result = Vec::with_capacity(to);
    for i in 1..=to {
        let s = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        };

        result.push(s);
    }

    result
}

fn main() {
    print_result(&fizz_buzz(100));
}

fn print_result(result: &Vec<String>) {
    for r in result {
        println!("{}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fizz_buzz() {
        let actual = fizz_buzz(15);

        assert_eq!(actual[0], "1");
        assert_eq!(actual[2], "Fizz");
        assert_eq!(actual[4], "Buzz");
        assert_eq!(actual[14], "FizzBuzz");
    }
}
