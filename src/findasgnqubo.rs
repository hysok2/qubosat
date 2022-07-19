use varisat::{CnfFormula, ExtendFormula};
use varisat::{Lit};

use std::time::{Instant};
use std::cmp;

use crate::solqubo::*;

// val以下となるQUBOの付値を求める
pub fn findasgnqubo(input:Vec<Vec<i32>>, val: i32, base: i32) -> Result<bool,String> {
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
                    p -= v;
                }
            } else {
                let v = input[i][j]+input[j][i];
                mat_n.push(v);
                if v < 0 {
                    p -= v;
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
                m /= base;
           }
        }
        num_b.push(tmp);
    }
    let mut num_val = Vec::<i32>::new();
    let mut m = val + p;
    //println!("m {}",m);
    if m == 0 {
        num_val.push(0);
    } else {
        while m > 0 {
            num_val.push(m % base);
            m /= base;
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

    let mut solver;
    let mut res;
    let mut satmodel;
    let zerop;
    let mut zeropos = Vec::<Option<usize>>::new();
    let mut vg;

    //println!("sl_length, num_val_length {} {}", sorter_lst.len(), num_val.len());

    match sorter_lst.len().cmp(&num_val.len()){
        cmp::Ordering::Greater => {
            zerop = sorter_lst.last().unwrap().output.len();
            for _i in 0..(sorter_lst.len() - num_val.len() - 1) {
                num_val.push(0);
            }
        },
        cmp::Ordering::Less => {
            let mut onep = 0;
            for _i in 0..(num_val.len() - sorter_lst.len() + 1) {
                onep *= base as usize;
                onep += num_val.pop().unwrap() as usize;
            }
            zerop = sorter_lst.last().unwrap().output.len() - (onep as usize);
        },
        cmp::Ordering::Equal => {
            zerop = sorter_lst.last().unwrap().output.len() - (*(num_val.last().unwrap()) as usize);
            num_val.pop();
        },
    }
    //println!("sl_length, num_val_length {} {}", sorter_lst.len(), num_val.len());
    num_val.reverse();
    // num_val 左の方の値が、高いbaseに対応
    for i in 0..(sorter_lst.len()-1) {
        if sorter_lst[sorter_lst.len() - i - 1 - 1].output.is_empty() {
            zeropos.push(None);
        } else {
            zeropos.push(Some(num_val[i] as usize));
        }
    }
    //println!("stlst len, zerop, num_val, zeropos = {} {} {:?} {:?}", sorter_lst.len(), zerop, num_val, zeropos);

    // val以下となる変数配置があるかをsorterの外から順に確認

    // sorter1個目の処理
    let start = Instant::now();
    solver = Solver::new();
    solver.add_formula(&f);
    solver.add_formula(&mk_0cons(&sorter_lst,zerop));

    match solver.solve() {
        Ok(result) => 
            res = result,
        Err(msg) => return Err(msg.to_string()),
    }

    let end = start.elapsed();
    println!("1st {} {}.{:03}sec", res, end.as_secs(), end.subsec_millis());

    if res {
        satmodel=solver.model().unwrap();

        //変数付値から、最小値の計算
        let mut q = 0;
        for j in n..(n + mat_m.len()) {
            //println!("{:?}",satmodel[i]);
            if satmodel[j] == Lit::from_dimacs((j + 1) as isize) {
                q += mat_m[j-n];
            }
        }

        //println!("The current result {}", q-p);

        if q-p <= val {
            println!("-----result-----");
            print!("x =");
            for i in 0..n {
                print!(" {:?}",satmodel[i]);
            }
            println!();
            return Ok(true);
        }
    } else {
        return Ok(false);
    }

    // sorter2個目以降の処理
    for i in 1..sorter_lst.len() {

        let start = Instant::now();

        vg=vargen;
        solver = Solver::new();
        solver.add_formula(&f);
        solver.add_formula(&mk_0cons(&sorter_lst,zerop));

        for k in 0..(i-1) {
            match zeropos[k] {
                Some(mk) => {
                    solver.add_formula(
                    &mk_0cons_mod(&sorter_lst, sorter_lst.len() - 1 - 1 - k,
                    mk as usize, base as usize, &mut vg));
                },
                None => continue,
            }
        }

        match zeropos[i-1] {
            Some(mk) => {
                solver.add_formula(&mk_0cons_mod_less(&sorter_lst, sorter_lst.len() - 1 - i, 
                (mk + 1) as usize, base as usize, &mut vg));
            },
            None => continue,
        }

        match solver.solve() {
            Ok(result) => 
                res = result,
            Err(msg) => return Err(msg.to_string()),
        }

        let end = start.elapsed();
        println!("{}th {} {}.{:03}sec", i+1, res, end.as_secs(), end.subsec_millis());

        if res {
            satmodel=solver.model().unwrap();

            //変数付値から、最小値の計算
            let mut q = 0;
            for j in n..(n + mat_m.len()) {
                if satmodel[j] == Lit::from_dimacs((j + 1) as isize) {
                    q += mat_m[j-n];
                }
            }

            if q-p <= val {
                println!("-----result-----");
                satmodel=solver.model().unwrap();
                print!("x =");
                for i in 0..n {
                    print!(" {:?}",satmodel[i]);
                }
                println!();
                return Ok(true);
            } else {
                continue;
            }
        } else {
            return Ok(false);
        }

    }
    return Err("Unknown error...".to_string());

}
