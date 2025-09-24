const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0/5.0) + FREEZING_POINT
}

fn is_even(n: i32) -> bool{
    n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret {
        0
    }else if guess < secret {
        -1
    }else {
        1
    }
}

fn main(){
    let mut farenheit = 80.0;
    let celsius = fahrenheit_to_celsius(farenheit);
    println!("Farenheit {} = Celsius {:.1}",farenheit,celsius);

    let mut count = 0;
    while count != 5{
        farenheit += 1.0;
        println!("Farenheit {} = Celsius {:.1}",farenheit,fahrenheit_to_celsius(farenheit));
        count += 1;
    }
     
    let nums: [i32;10] = [30, 1, 142, 15 , 63, 2, 500, 12, 25, 100];

    for i in nums.iter(){
        if i % 3 == 0 && i % 5 == 0{
            println!("Fizzbuzz");
        }else if i % 5 == 0{
            println!("Buzz");
        }else if i % 3 == 0{
            println!("Fizz")
        }else if is_even(*i){
            println!("Num {} is even.",i);
        }else{
            println!("Num {} is odd",i);   
        }
        
    }

    let mut sum = 0;
    let mut j = 0;
    while j < 10{
        sum += nums[j];
        j += 1;
    }
    println!("Sum = {}", sum);

    let mut count = 0;
    let mut temp = nums[0];

    loop {
        if count == 10{
            break;
        }
        
        if temp < nums[count] {
            temp = nums[count];
        }   
        count += 1;
    }

    println!("Largest = {}", temp);

    let mut guess = 10;
    let mut secret = 5;
    let mut guess_count = 0;
    loop{
        let result = check_guess(guess,secret);
        if result == 0 {
            guess_count += 1;
            println!("Correct!");
            break;
        }else if result < 0{
            println!("Too low");
            guess += 1;
            guess_count += 1;
        }else if result == 1{
            println!("Too high");
            guess -= 1;
            guess_count += 1;
        }
    }

    println!("Took {} guesses",guess_count);
   
}