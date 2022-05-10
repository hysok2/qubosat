use qubosat::solqubo;
use qubosat::chkqubo;
use qubosat::findasgnqubo;
use qubosat::utils;
use qubosat::tabus;

use clap::Parser;

/// Program to solve QUBO related problems with SAT
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {

    qubo_filename: String,

    /// Change the base number
    #[clap(short, long, default_value = "2", value_name = "INTEGER (greater than 1)")]
    base: i32,

    /// Checking-minimality-mode: Check if a given integer is a solution of a given QUBO problem
    #[clap(short, long, allow_hyphen_values = true, value_name = "INTEGER")]
    check: Option<i32>,

    /// Finding-assignments-mode: Find assignments that make a given quadratic expression less than or equal to a given integer 
    #[clap(short, long, allow_hyphen_values = true, value_name = "INTEGER")]
    findasgn: Option<i32>,

    /// Optimization-mode-with-tabusearch: Solve a QUBO problem with tabu search and SAT
    #[clap(short, long)]
    tabus: bool,
}


fn main() {
    use std::env;
    let arg = Args::parse();
    let filename = arg.qubo_filename;
    let base = arg.base;

    if let Some(c) = arg.check {
        println!("-----------------------------------------------------------------");
        println!("Checking-minimality-mode");
        println!("The base number is {}", base);
        println!("Input: an integer matrix Q (defined in {}), an integer q = {}", filename, c);
        println!("Output: true (minimum) or false (not minimum)");
        println!("-----------------------------------------------------------------");

        match utils::readqubo(filename) {
            Ok(cl) => {
                //println!("{:?}",cl);
                match chkqubo::chkqubo(cl, c, base) {
                    Ok(n) => println!("{}",n),
                    Err(e) => println!("Error : {}",e),
                };
            },
            Err(st) => println!("{}", st),
        }
        return;
    }

    if let Some(c) = arg.findasgn {
        println!("-----------------------------------------------------------------");
        println!("Finding-assignments-mode");
        println!("The base number is {}", base);
        println!("Input: an integer matrix Q (defined in {}), an integer q = {}", filename, c);
        println!("Output: assignments x s.t. x^T Q x <= q");
        println!("-----------------------------------------------------------------");

        match utils::readqubo(filename) {
            Ok(cl) => {
                //println!("{:?}",cl);
                match findasgnqubo::findasgnqubo(cl, c, base) {
                    Ok(n) => println!("{}",n),
                    Err(e) => println!("Error : {}",e),
                };
            },
            Err(st) => println!("{}", st),
        }

        return;
    }

    if arg.tabus {
        println!("-----------------------------------------------------------------");
        println!("Optimization-mode-with-tabusearch");
        println!("The base number is {}", base);
        println!("Input: an integer matrix Q (defined in {})", filename);
        println!("Output: minimum value q and assignments x s.t. min_x x^T Q x = q");
        println!("-----------------------------------------------------------------");

        match utils::readqubo(filename) {
            Ok(cl) => {
                //println!("{:?}",cl);
                match solqubo::solqubo(cl, base, true) {
                    Ok(n) => println!("q = {}",n),
                    Err(e) => println!("Error : {}",e),
                };
            },
            Err(st) => println!("{}", st),
        }
        return;
    }

    println!("-----------------------------------------------------------------");
    println!("Optimization-mode");
    println!("The base number is {}", base);
    println!("Input: an integer matrix Q (defined in {})", filename);
    println!("Output: minimum value q and assignments x s.t. min_x x^T Q x = q");
    println!("-----------------------------------------------------------------");

    match utils::readqubo(filename) {
        Ok(cl) => {
            //println!("{:?}",cl);
            match solqubo::solqubo(cl, base, false) {
                Ok(n) => println!("q = {}",n),
                Err(e) => println!("Error : {}",e),
            };
        },
        Err(st) => println!("{}", st),
    }
}


#[test]
fn test0() {
    assert_eq!(solqubo::solqubo(vec![
        vec![-1,0,1,1,1,0],
        vec![0,-1,0,1,0,0], 
        vec![0,0,-1,0,0,0], 
        vec![0,0,0,-1,0,1], 
        vec![0,0,0,0,-1,0], 
        vec![0,0,0,0,0,-1]],2,false),Ok(-4));
}

fn test1_lst(base : i32, t: bool) {
    assert_eq!(solqubo::solqubo(vec![vec![1]],base,t),Ok(0));
    assert_eq!(solqubo::solqubo(vec![vec![1,0],vec![0,-10]],base,t),Ok(-10));
    assert_eq!(solqubo::solqubo(vec![vec![1,0],vec![-4,1]],base,t),Ok(-2));
    assert_eq!(solqubo::solqubo(vec![vec![10,20],vec![-2,3]],base,t),Ok(0));
    assert_eq!(solqubo::solqubo(vec![vec![-1000,1],vec![0,-1000]],base,t),Ok(-1999));
    //Problems from Glover et al. Quantum Bridge Analytics I: A Tutorial on Formulating and Using QUBO Models
    assert_eq!(solqubo::solqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],base,t),Ok(-11));
    assert_eq!(solqubo::solqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],base,t),Ok(-6889));
}

#[test]
fn test1() {
    let li : Vec<i32> = vec![2,3,5,10];

    for i in li.iter() {
        println!("test with base : {}", *i);
        test1_lst(*i,true);
    }
        
}

#[test]
fn test1_() {
    let li : Vec<i32> = vec![2,3,5,10];

    for i in li.iter() {
        println!("test with base : {}", *i);
        test1_lst(*i,false);
    }
        
}

fn test2_lst(base : i32) {
    assert_eq!(chkqubo::chkqubo(vec![vec![1]],0,base),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![1,0],vec![0,-10]],-10,base),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![1,0],vec![-4,1]],-2,base),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![10,20],vec![-2,3]],0,base),Ok(true));
    //Problems from Glover et al. Quantum Bridge Analytics I: A Tutorial on Formulating and Using QUBO Models
    assert_eq!(chkqubo::chkqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],-11,base),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],-10,base),Ok(false));
    assert_eq!(chkqubo::chkqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],-12,base),Ok(false));
    assert_eq!(chkqubo::chkqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],-6889,base),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],-6890,base),Ok(false));
    assert_eq!(chkqubo::chkqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],-6888,base),Ok(false));
}

#[test]
fn test2 () {

    let li : Vec<i32> = vec![2,3,5,10];

    for i in li.iter() {
        println!("test with base : {}", *i);
        test2_lst(*i);
    }
}
