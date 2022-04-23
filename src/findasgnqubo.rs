use varisat::{CnfFormula, ExtendFormula};
use varisat::{Lit};

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
    let mut satmodel = Vec::<Lit>::new();
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

    // valとなる変数配置があるかを確認
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
    if !res {
        return Ok(false);
    }
    
    println!("-----result-----");
    satmodel=solver.model().unwrap();
    print!("model");
    for i in 0..n {
        print!(" {:?}",satmodel[i]);
    }
    println!();
    return Ok(true);
}

pub fn mk_0cons_mod_less(stlst:& Vec<Sorter>, pos: usize, l: usize, base: usize, vargen: &mut usize) -> CnfFormula {
    let mut h = CnfFormula::new();
    let mut j = 0;

    let mut vl = Vec::<Lit>::new();

    for m in 0..l {
        //println!("iter {} in mk mod less",m);
        j = m;
        if m == 0 {
            vl.push(Lit::from_dimacs(-(stlst[pos].output[0] as isize)));
            j += base;
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
                j += base;
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
                j += base;
            }
            if j == stlst[pos].output.len() {vl.push(Lit::from_dimacs(stlst[pos].output[j - 1] as isize));}
        }
    }
    h.add_clause(&vl);
    //println!("h {:?}", h);
    return h;
}

pub fn mk_0cons_mod_not_grt(stlst:& Vec<Sorter>, pos: usize, l: usize, base: usize, vargen: &mut usize) -> CnfFormula {
    let mut h = CnfFormula::new();
    let mut j = 0;

    let mut vl = Vec::<Lit>::new();

    for m in l..base {
        //println!("iter {} in mk mod less",m);
        j = m;
        if m == 0 {
            vl.push(Lit::from_dimacs(-(stlst[pos].output[0] as isize)));
            j += base;
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
                j += base;
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
                j += base;
            }
            if j == stlst[pos].output.len() {vl.push(Lit::from_dimacs(stlst[pos].output[j - 1] as isize));}
        }
    }
    for e in vl.iter() {
        h.add_clause(&[!(*e)]);
    }
    //println!("h {:?}", h);
    return h;
}