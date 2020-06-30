use varisat::dimacs::{DimacsParser, write_dimacs};
use varisat::{CnfFormula, ExtendFormula};
use varisat::{Var, Lit};

use crate::solqubo::*;

pub fn chkqubo(input:Vec<Vec<i32>>, val: i32) -> Result<bool,String> {
    let n = input.len();
    let mut N = Vec::<i32>::new();
    let mut p = 0;

    // Nに行列の要素を入れていく
    for i in 0..n {
        for j in 0..(i+1) {
            if i == j {
                let v = input[i][j];
                N.push(v);
                if v < 0 {
                    p = p - v;
                }
            } else {
                let v = input[i][j]+input[j][i];
                N.push(v);
                if v < 0 {
                    p = p - v;
                }
            }
        }
    }

    // QUBOの変数とPseudo boolean constraintsの変数の関係をCNFで記述、Nの要素のminusを考慮
    let mut f = CnfFormula::new();
    for i in 0..n {
        for j in 0..(i+1) {
            if N[(i*(i+1))/2+j] >= 0 {
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
    
    // M=abs(N)
    let mut M = Vec::<i32>::new();
    for i in 0..(N.len()) {
        M.push(N[i].abs());
    }
    
    //係数をBaseで素因数分解
    let Base = 5;
    let mut num_b = Vec::<Vec<i32>>::new();
    for i in 0..(N.len()) {
        let mut tmp = Vec::<i32>::new();
        let mut m = M[i];
        if m == 0 {
            tmp.push(0);
        } else {
            while m > 0 {
                tmp.push(m % Base);
                m = m / Base;
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
            num_val.push(m % Base);
            m = m / Base;
        }
    }

    println!("bum_b {:?}",num_b);
    println!("num_val {:?}", num_val);

    //sorter作成
    let mut sorter_lst = Vec::<Sorter>::new();
    let mut vargen = N.len() + n + 1;
    mk_sorterlst(&mut sorter_lst, & num_b, &mut f, Base as usize, n, &mut vargen);

    println!("{:?}",sorter_lst);
    //println!("{:?}",f);

    //////////解確認開始
    use varisat::solver::Solver;

    let mut solver = Solver::new();
    let mut res;
    let mut satmodel = Vec::<Lit>::new();
    let mut zerop = 0;
    let mut zeropos = Vec::<Option<usize>>::new();
    let mut vg = 0;

    if sorter_lst.len() > num_val.len() {
        zerop = sorter_lst.last().unwrap().output.len();
        for i in 0..(sorter_lst.len() - num_val.len() - 1) {
            num_val.push(0);
        }
    } else if sorter_lst.len() < num_val.len() {
        for i in 0..(num_val.len() - sorter_lst.len() + 1) {
            zerop *= (Base as usize);
            zerop += (num_val.pop().unwrap() as usize);
        }
        zerop = sorter_lst.last().unwrap().output.len() - (zerop as usize);
    } else {
        zerop = sorter_lst.last().unwrap().output.len() - (*(num_val.last().unwrap()) as usize);
        num_val.pop();
    }
    num_val.reverse();
    // num_val 左の方の値が、高いBaseに対応
    for i in 0..(sorter_lst.len()-1) {
        if sorter_lst[sorter_lst.len() - i - 1 - 1].output.len() == 0 {
            zeropos.push(None);
        } else {
            zeropos.push(Some(num_val[i] as usize));
        }
    }
    println!("stlst len, zerop, num_val, zeropos = {} {} {:?} {:?}", sorter_lst.len(), zerop, num_val, zeropos);

    // valとなる変数配置があるかを確認(quboの解がval以下かを確認)
    vg = vargen;
    solver.add_formula(&f);
    solver.add_formula(&mk_0cons(&sorter_lst, zerop));
    for k in 0..(zeropos.len()) {
        match zeropos[k] {
            Some(mk) => {
                solver.add_formula(
                &mk_0cons_mod(&sorter_lst, sorter_lst.len() - 1 - 1 - k,
                mk as usize, Base as usize, &mut vg));
            },
            None => continue,
        }
    }
    match solver.solve() {
        Ok(result) => 
            res = result,
        Err(msg) => return Err(msg.to_string()),
    }
    if (!res) {
        return Ok(false);
    }

    
    // quboの答えが、valより小さくないか確認
    // sorter 1個目の出力に0を足してsatを解く

    if zerop < sorter_lst.last().unwrap().output.len() {
        solver.add_formula(&f);
        solver.add_formula(&mk_0cons(&sorter_lst, zerop + 1));
        match solver.solve() {
            Ok(result) => 
                res = result,
            Err(msg) => return Err(msg.to_string()),
        }
        if res {
            return Ok(false);
        }
    }

    // sorter 2個目以降を調べる
    
    for i in 0..(zeropos.len()) {
        println!("iter {}", i);
        solver = Solver::new();
        solver.add_formula(&f);
        solver.add_formula(&mk_0cons(&sorter_lst, zerop));
        vg = vargen;

        for k in 0..i {
            match zeropos[k] {
                Some(mk) => {
                    solver.add_formula(
                    &mk_0cons_mod(&sorter_lst, sorter_lst.len() - 1 - 1 - k,
                    mk as usize, Base as usize, &mut vg));
                },
                None => continue,
            }
        }

        if zeropos[i] == None || zeropos[i] == Some(0) {
            continue;
        } else {
            solver.add_formula(&mk_0cons_mod_less(&sorter_lst, sorter_lst.len() - 1 - 1 - i, 
                zeropos[i].unwrap() as usize, Base as usize, &mut vg));
        }
        match solver.solve() {
            Ok(result) => 
                res = result,
            Err(msg) => return Err(msg.to_string()),
        }
        if res {
            return Ok(false);
        }
        
    }
    
    return Ok(true);
}

pub fn mk_0cons_mod_less(stlst:& Vec<Sorter>, pos: usize, l: usize, Base: usize, vargen: &mut usize) -> CnfFormula {
    let mut h = CnfFormula::new();
    let mut j = 0;

    let mut vl = Vec::<Lit>::new();

    for m in 0..l {
        println!("iter {} in mk mod less",m);
        j = m;
        if m == 0 {
            vl.push(Lit::from_dimacs(-(stlst[pos].output[0] as isize)));
            j += Base;
            while j < stlst[pos].output.len() {
                let k1 = j;
                let k0 = j - 1;
                let v1 = stlst[pos].output[k1] as isize;
                let v0 = stlst[pos].output[k0] as isize;
                let o = (*vargen) as isize;
                *vargen += 1;
                vl.push(Lit::from_dimacs(o));
                //v0 = true, v1 = false,
                //o = !v1 and v0
                h.add_clause(&[Lit::from_dimacs(v1), !Lit::from_dimacs(v0), Lit::from_dimacs(o)]);
                h.add_clause(&[!Lit::from_dimacs(v1), !Lit::from_dimacs(o)]);
                h.add_clause(&[Lit::from_dimacs(v0), !Lit::from_dimacs(o)]);
                j += Base;
            }
            if j == stlst[pos].output.len() {vl.push(Lit::from_dimacs(stlst[pos].output[j - 1] as isize));}
        } else {
            while j < stlst[pos].output.len() {
                let k1 = j;
                let k0 = j - 1;
                let v1 = stlst[pos].output[k1] as isize;
                let v0 = stlst[pos].output[k0] as isize;
                let o = (*vargen) as isize;
                *vargen += 1;
                vl.push(Lit::from_dimacs(o));
                //v0 = true, v1 = false,
                //o = !v1 and v0
                h.add_clause(&[Lit::from_dimacs(v1), !Lit::from_dimacs(v0), Lit::from_dimacs(o)]);
                h.add_clause(&[!Lit::from_dimacs(v1), !Lit::from_dimacs(o)]);
                h.add_clause(&[Lit::from_dimacs(v0), !Lit::from_dimacs(o)]);
                j += Base;
            }
            if j == stlst[pos].output.len() {vl.push(Lit::from_dimacs(stlst[pos].output[j - 1] as isize));}
        }
    }
    h.add_clause(&vl);
    println!("h {:?}", h);
    return h;
}