mod operand;

use structopt::StructOpt;

use crate::operand::*;

pub fn start() -> f64 {
    
    let calc = Operand::from_args();
    
    println!("{}", calc);
    
    let result_number = calc.kalkulasi();

    result_number
}
