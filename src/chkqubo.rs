use varisat::{CnfFormula, ExtendFormula};
use varisat::{Lit};

use crate::solqubo::*;

pub fn chkqubo(input:Vec<Vec<i32>>, val: i32, base: i32) -> Result<bool,String> {
    let n = input.len();
    let mut mat_n = Vec::<i32>::new();
    let mut p = 0;

    // mat_nに行列の要素を入れていく
    for i in 0..n {
        for j in 0..(i+1) {
            if i == j {
                let v = input[i][j];
                mat_n.push(v);
                if v < 0 {
                    p = p - v;
                }
            } else {
                let v = input[i][j]+input[j][i];
                mat_n.push(v);
                if v < 0 {
                    p = p - v;
                }
            }
        }
    }

    // QUBOの変数とPseudo boolean constraintsの変数の関係をCNFで記述、mat_nの要素のminusを考慮
    let mut f = CnfFormula::new();
    for i in 0..n {
        for j in 0..(i+1) {
            if mat_n[(i*(i+1))/2+j] >= 0 {
                let x = (i+1) as isize;
                let y = (j+1) as isize;
                let z = ((i*(i+1))/2+j+1+n) as isize;
                f.add_clause(&[!Lit::from_dimacs(x), !Lit::from_dimacs(y), Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(x), !Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
            } else {
                let x = (i+1) as isize;
                let y = (j+1) as isize;
                let z = ((i*(i+1))/2+j+1+n) as isize;
                f.add_clause(&[!Lit::from_dimacs(x), !Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(x), Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(y), Lit::from_dimacs(z)]);
            }
        }
    }
    
    // mat_m=abs(mat_n)
    let mut mat_m = Vec::<i32>::new();
    for i in 0..(mat_n.len()) {
        mat_m.push(mat_n[i].abs());
    }
    
    //係数をbaseで素因数分解
    let mut num_b = Vec::<Vec<i32>>::new();
    for i in 0..(mat_n.len()) {
        let mut tmp = Vec::<i32>::new();
        let mut m = mat_m[i];
        if m == 0 {
            tmp.push(0);
        } else {
            while m > 0 {
                tmp.push(m % base);
                m = m / base;
           }
        }
        num_b.push(tmp);
    }
    let mut num_val = Vec::<i32>::new();
    let mut m = val + p;
    // valが小さすぎるため(QUBOの負の係数の和よりもvalが小さい)、valは解ではない。
    if m < 0 {
        return Ok(false);
    }
    //println!("m {}",m);
    if m == 0 {
        num_val.push(0);
    } else {
        while m > 0 {
            num_val.push(m % base);
            m = m / base;
        }
    }

    //println!("bum_b {:?}",num_b);
    //println!("num_val {:?}", num_val);

    //sorter作成
    let mut sorter_lst = Vec::<Sorter>::new();
    let mut vargen = mat_n.len() + n + 1;
    mk_sorterlst(&mut sorter_lst, & num_b, &mut f, base as usize, n, &mut vargen);

    //println!("{:?}",sorter_lst);
    //println!("{:?}",f);

    //////////解確認開始
    use varisat::solver::Solver;

    let mut solver = Solver::new();
    let mut res;

    // valとなる変数配置があるかを確認(quboの解がval以下かを確認)
    //println!("Check if exists x s.t. x^T Q x <= val");

    let mut zerop = 0;
    let mut zeropos = Vec::<Option<usize>>::new();
    let mut vg = 0;

    if sorter_lst.len() > num_val.len() {
        zerop = sorter_lst.last().unwrap().output.len();
        for _i in 0..(sorter_lst.len() - num_val.len() - 1) {
            num_val.push(0);
        }
    } else if sorter_lst.len() < num_val.len() {
        for _i in 0..(num_val.len() - sorter_lst.len() + 1) {
            zerop *= base as usize;
            zerop += num_val.pop().unwrap() as usize;
        }
        zerop = sorter_lst.last().unwrap().output.len() - (zerop as usize);
    } else {
        zerop = sorter_lst.last().unwrap().output.len() - (*(num_val.last().unwrap()) as usize);
        num_val.pop();
    }
    num_val.reverse();
    // num_val 左の方の値が、高いbaseに対応
    for i in 0..(sorter_lst.len()-1) {
        if sorter_lst[sorter_lst.len() - i - 1 - 1].output.len() == 0 {
            zeropos.push(None);
        } else {
            zeropos.push(Some(num_val[i] as usize));
        }
    }
    //println!("stlst len, zerop, num_val, zeropos = {} {} {:?} {:?}", sorter_lst.len(), zerop, num_val, zeropos);

    vg = vargen;
    solver.add_formula(&f);
    solver.add_formula(&mk_0cons(&sorter_lst, zerop));
    for k in 0..(zeropos.len()) {
        match zeropos[k] {
            Some(mk) => {
                solver.add_formula(
                &mk_0cons_mod(&sorter_lst, sorter_lst.len() - 1 - 1 - k,
                mk as usize, base as usize, &mut vg));
            },
            None => continue,
        }
    }
    match solver.solve() {
        Ok(result) => 
            res = result,
        Err(msg) => return Err(msg.to_string()),
    }
    //println!("res = {}", res);
    if !res {
        return Ok(false);
    }

    
    // quboの答えが、(val-1)以下でないことを確認
    // val-1となる変数配置があるかを確認、あればvalが最小値ではないのでfalseを返す。
    //println!("Check if not exists x s.t. x^T Q x <= val-1");

    let mut num_val2 = Vec::<i32>::new();
    let mut m2 = val - 1 + p;
    // val-1が小さすぎるため(QUBOの負の係数の和よりもval-1が小さい)、val-1は解ではない。
    if m2 < 0 {
        return Ok(true);
    }
    //println!("m2 {}",m2);
    if m2 == 0 {
        num_val2.push(0);
    } else {
        while m2 > 0 {
            num_val2.push(m2 % base);
            m2 = m2 / base;
        }
    }

    let mut zerop2 = 0;
    let mut zeropos2 = Vec::<Option<usize>>::new();
    let mut vg2 = 0;

    if sorter_lst.len() > num_val2.len() {
        zerop2 = sorter_lst.last().unwrap().output.len();
        for _i in 0..(sorter_lst.len() - num_val2.len() - 1) {
            num_val2.push(0);
        }
    } else if sorter_lst.len() < num_val2.len() {
        for _i in 0..(num_val2.len() - sorter_lst.len() + 1) {
            zerop2 *= base as usize;
            zerop2 += num_val2.pop().unwrap() as usize;
        }
        zerop2 = sorter_lst.last().unwrap().output.len() - (zerop2 as usize);
    } else {
        zerop2 = sorter_lst.last().unwrap().output.len() - (*(num_val2.last().unwrap()) as usize);
        num_val2.pop();
    }
    num_val2.reverse();
    // num_val2 左の方の値が、高いbaseに対応
    for i in 0..(sorter_lst.len()-1) {
        if sorter_lst[sorter_lst.len() - i - 1 - 1].output.len() == 0 {
            zeropos2.push(None);
        } else {
            zeropos2.push(Some(num_val2[i] as usize));
        }
    }

    //println!("stlst len, zerop2, num_val2, zeropos2 = {} {} {:?} {:?}", sorter_lst.len(), zerop2, num_val2, zeropos2);

    vg2 = vargen;
    solver = Solver::new();
    solver.add_formula(&f);
    solver.add_formula(&mk_0cons(&sorter_lst, zerop2));
    for k in 0..(zeropos2.len()) {
        match zeropos2[k] {
            Some(mk) => {
                solver.add_formula(
                &mk_0cons_mod(&sorter_lst, sorter_lst.len() - 1 - 1 - k,
                mk as usize, base as usize, &mut vg2));
            },
            None => continue,
        }
    }
    match solver.solve() {
        Ok(result) => 
            res = result,
        Err(msg) => return Err(msg.to_string()),
    }
    //println!("res = {}", res);
    if res {
        return Ok(false);
    }
    
    return Ok(true);
}
