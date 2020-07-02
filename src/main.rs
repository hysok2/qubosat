
use qubosat::solqubo;
use qubosat::chkqubo;
use qubosat::utils;

fn main() {
    use std::env;
    let filename = env::args().nth(1).expect("filename required");
    match utils::readqubo(filename) {
        Ok(cl) => {
            //println!("{:?}",cl);
            match solqubo::solqubo(cl) {
                Ok(n) => println!("res = {}",n),
                Err(e) => println!("Error : {}",e),
            };
        },
        Err(st) => println!("{}", st),
    }
}


#[test]
fn test1() {
    assert_eq!(solqubo::solqubo(vec![vec![1]]),Ok(0));
    assert_eq!(solqubo::solqubo(vec![vec![1,0],vec![0,-10]]),Ok(-10));
    assert_eq!(solqubo::solqubo(vec![vec![1,0],vec![-4,1]]),Ok(-2));
    assert_eq!(solqubo::solqubo(vec![vec![10,20],vec![-2,3]]),Ok(0));
    //Problems from Glover et al. Quantum Bridge Analytics I: A Tutorial on Formulating and Using QUBO Models
    assert_eq!(solqubo::solqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]]),Ok(-11));
    assert_eq!(solqubo::solqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ]),Ok(-6889));
        
}
/*
#[test]
fn test2 () {
    assert_eq!(chkqubo::chkqubo(vec![vec![1]],0),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![1,0],vec![0,-10]],-10),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![1,0],vec![-4,1]],-2),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![10,20],vec![-2,3]],0),Ok(true));
    //Problems from Glover et al. Quantum Bridge Analytics I: A Tutorial on Formulating and Using QUBO Models
    assert_eq!(chkqubo::chkqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],-11),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],-10),Ok(false));
    assert_eq!(chkqubo::chkqubo(vec![vec![-5,2,4,0],vec![2,-3,1,0],vec![4,1,-8,5],vec![0,0,5,-6]],-12),Ok(false));
    assert_eq!(chkqubo::chkqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],-6889),Ok(true));
    assert_eq!(chkqubo::chkqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],-6890),Ok(false));
    assert_eq!(chkqubo::chkqubo(vec![
        vec![-3525, 175, 325, 775, 1050, 425, 525, 250],
        vec![175, -1113, 91, 217, 294, 119, 147, 70],
        vec![325, 91, -1989, 403, 546, 221, 273, 130],
        vec![775, 217, 403, -4185, 1302, 527, 651, 310],
        vec![1050, 294, 546, 1302, -5208, 714, 882, 420],
        vec![425, 119, 221, 527, 714, -2533, 357, 170],
        vec![525, 147, 273, 651, 882, 357, -3045, 210],
        vec![250, 70, 130, 310, 420, 170, 210, -1560],
        ],-6888),Ok(false));
}
*/