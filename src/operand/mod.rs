use std::fmt;
use structopt::StructOpt;
/// Basic Onetime Run Calculator Writen in Rust
#[derive(StructOpt)]
pub struct Operand {
    /// The operand for calculator
    /// 
    /// Addition (+)
    /// 
    /// Substracting (-)
    /// 
    /// Product (*)
    /// 
    /// Division (/)
    /// 
    /// Modulus (%)
    /// 
    /// Power (^)
    pub operand: char,

    /// The first number
    pub num1: f64,

    /// The second number
    pub num2: f64,
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

    pub fn kalkulasi(&self) -> f64 {
        match self.operand {
            '+' => { self.addition() },
            '-' => { self.substraction() },
            '*' => { self.product() },
            '/' => { self.division() },
            '%' => { self.modulus() },
            '^' => { self.power() },
            _ => panic!("No Operand"),
        }
    }
}

// Implement `Display` for `Operand Struct`
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize print the Operand struct
        write!(f, "
    Operand : {}
    Num 1   : {}
    Num 2   : {}"
            , self.operand, self.num1, self.num2
        )
    }
}