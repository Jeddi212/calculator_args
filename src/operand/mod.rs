use core::panic;
use crate::read;

struct Operand {
    num1: f64,
    num2: f64,
}

impl Operand {
    fn addition(&self) -> f64 {
        self.num1 + self.num2
    }
    
    fn substraction(&self) -> f64 {
        self.num1 - self.num2
    }
    
    fn product(&self) -> f64 {
        self.num1 * self.num2
    }
    
    fn division(&self) -> f64 {
        self.num1 / self.num2
    }
    
    fn modulus(&self) -> f64 {
        self.num1 % self.num2
    }
    
    fn power(&self) -> f64 {
        self.num1.powf(self.num2)
    }
    
}

pub fn calculate_1(op: &str, num1: f64) -> f64 {

    let mut num2 = String::new();

    println!("Num 1 : {}", num1);

    print!("Num 2 : ");
    read(&mut num2);
    
    let kalkulasi = Operand { 
        num1: num1.to_string().trim().parse::<f64>().expect("The input should a number"),
        num2: num2.to_string().trim().parse::<f64>().expect("The input should a number"),
    };

    match op {
        "1" => { kalkulasi.addition() },
        "2" => { kalkulasi.substraction() },
        "3" => { kalkulasi.product() },
        "4" => { kalkulasi.division() },
        "5" => { kalkulasi.modulus() },
        "6" => { kalkulasi.power() },
        _ => panic!("No Operand"),
    }
}

pub fn calculate_2(op: &str) -> f64 {

    let mut num1 = String::new();
    let mut num2 = String::new();

    print!("Num 1 : ");
    read(&mut num1);
    
    print!("Num 2 : ");
    read(&mut num2);
    
    let kalkulasi = Operand { 
        num1: num1.to_string().trim().parse::<f64>().expect("The input should a number"),
        num2: num2.to_string().trim().parse::<f64>().expect("The input should a number"),
    };

    match op {
        "1" => { kalkulasi.addition() },
        "2" => { kalkulasi.substraction() },
        "3" => { kalkulasi.product() },
        "4" => { kalkulasi.division() },
        "5" => { kalkulasi.modulus() },
        "6" => { kalkulasi.power() },
        _ => panic!("No Operand"),
    }
}
