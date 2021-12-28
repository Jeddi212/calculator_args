mod operand;

use std::io::{stdin, stdout, Write};

pub fn start() -> String {

    let mut result_number = 0_f64;
    let mut user_choose = 2_i8;
    let mut input = String::new();

    loop {

        // choose Operand
        print_operand();
        
        read(&mut input);

        input = input.trim().into();

        // do Calculate
        if input == "1" || 
            input == "2" || 
            input == "3" || 
            input == "4" || 
            input == "5" || 
            input == "6" 
        {

            match user_choose {
                1 => result_number = operand::calculate_1(&input, result_number),
                2 => result_number = operand::calculate_2(&input),
                _ => println!("Is you choose the right number option ?")
            }
            println!("Result : {}\n", &result_number);

        } 
        else 
        {
            
            println!("Wish to exit ?");

        }

        // re-use Calculator?
        println!("Continue ?
    1. Use last result
    2. Reset
    3. Quit");
        
        read(&mut input);

        input = input.trim().into();

        if keep_going(&input, &mut user_choose) {
            continue
        } 
        break "Thank You\n".to_owned();
    }

    
}

fn print_operand() {

    print!(r#"
        Please input The Operand : 
        1. Addition (+)
        2. Substracting (-)
        3. Product (*)
        4. Division (/)
        5. Modulus (%)
        6. Power (^)
        7. Exit
        -> "#);

}

fn keep_going(input: &str, user_choose: &mut i8) -> bool {

    match input {
        "1" => {
            *user_choose = 1;
            true
        },
        "2" => {
            *user_choose = 2;
            true
        },
        _ => {
            *user_choose = 3;
            false
        },
    }

}

fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush!");
    stdin().read_line(input).expect("Failed to read input!");
}
