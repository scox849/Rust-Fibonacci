
use std::env;
use std::thread;
use num_bigint::BigInt;
use std::mem::replace;
use num_traits::{Zero, One};

const STACK_SIZE: usize = 500 * 1024 * 1024;

//Bulk of logic for fibonacci sequence
fn calc_fib(){
   
    //Command line args. Expect Cargo run -- X. Where X is a positive int.
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 || args.len() == 1 {
        println!("Too many or too few arguments");
        return;
    }

    let num = args[1].parse::<u32>().unwrap();

    //Initialize start values for fib sequence
    let mut current: BigInt = Zero::zero();
    let mut next: BigInt = One::one();
    
    //Loop num times
    for _ in 0..num{
        
        //0,1,2 = 0,1,1 after first 3 indexs calc remains the same
        if num == 2{
            current = next;
            break;
        }

        //Get new fib number and replace values for next iteration
        let new_num = &current + &next;
        next = replace(&mut current, new_num);

    }
    //Print final fib value
    println!("Fibonacci of {num} iterations: {current}");


}


fn main() {
    
    //Run on separted thread to control stack memory
    let child = thread::Builder::new().stack_size(STACK_SIZE).spawn(calc_fib).unwrap();
    child.join().unwrap();

}
