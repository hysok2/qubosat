//extern crate qubosat;

use qubosat::solqubo;

fn main() {
    let input = vec![vec![1,0],vec![1,0]];
    solqubo::solqubo(input);
}


#[test]
fn test1() {
    assert_eq!(solqubo::solqubo(vec![vec![1]]),0);
    assert_eq!(solqubo::solqubo(vec![vec![1,0],vec![0,-10]]),-10);
    assert_eq!(solqubo::solqubo(vec![vec![1,0],vec![-4,1]]),-2);
}