fn is_even(n: i32) -> bool {
    0 == n % 2
}

fn main() {
    let int_array: [i32; 10] = [-7, 3, 6, 15, 26, 33, -4, 0, -11, -10];

    for i in 0..10 {
        if int_array[i] % 5 == 0 && int_array[i] % 3 == 0 {
            print!("FizzBuzz ");
        } else if int_array[i] % 5 == 0 {
            print!("Buzz ");
        } else if int_array[i] % 3 == 0 {
            print!("Fizz ");
        } else if is_even(int_array[i]) {
            print!("Even ");
        } else {
            print!("Odd ");
        }
    }
    println!();

    let mut counter = 0;
    let mut sum = 0;
    while counter < 10 {
        sum += int_array[counter];
        counter += 1;
    }
    println!("Total sum of array: {}", sum);

    let mut biggest = 0;
    for i in 0..10 {
       if i == 0 {
        biggest = int_array[i];
       }
       if biggest < int_array[i] {
        biggest = int_array[i];
       }
    }
    println!("Biggest number in array: {}", biggest);
}