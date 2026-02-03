//Assignment 1

const FREEZING_POINT_F: f64 = 32.0;
// °C = (°F-32)/1.8
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}
//°F = (°C × 1.8) + 32.
fn celsius_to_fahrenheit(c: f64) -> f64{ 
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    let mut temp_f: f64 = 50.0;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F is {:.2}°C", temp_c);

    for _ in 1..=5{
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F is {:.2}°C", temp_c);
    }
}





//Assignment 2


fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [3, 8, 15, 22, 7, 30, 11, 4, 9, 18];

    // --- For loop: Even/Odd and FizzBuzz logic ---
    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    // --- While loop: Sum of all numbers ---
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("Sum of numbers: {}", sum);

    // --- loop: Find largest number ---
    let mut largest = numbers[0];
    let mut i = 0;

    loop {
        if i >= numbers.len() {
            break;
        }

        if numbers[i] > largest {
            largest = numbers[i];
        }

        i += 1;
    }

    println!("Largest number: {}", largest);
}


//Assignment 3

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 7;
    let mut guess_count = 0;

    loop {
        let guess = guess_count + 3; // Simulated guesses: 3, 4, 5, 6, 7...
        guess_count += 1;

        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Guess {}: Correct!", guess);
            break;
        } else if result == 1 {
            println!("Guess {}: Too high!", guess);
        } else {
            println!("Guess {}: Too low!", guess);
        }
    }

    println!("It took {} guesses to find the secret number.", guess_count);
}