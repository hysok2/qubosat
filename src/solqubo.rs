use varisat::dimacs::{DimacsParser, write_dimacs};
use varisat::{CnfFormula, ExtendFormula};
use varisat::{Var, Lit};

#[derive(Debug)]
pub struct Sorter {
    pub input : Vec<usize>,
    pub output : Vec<usize>,
    pub numcarr : usize,
}

pub fn solqubo(input:Vec<Vec<i32>>) -> Result<i32,String> {
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
    println!("bum_b {:?}",num_b);

    //sorter作成
    let mut sorter_lst = Vec::<Sorter>::new();
    let mut vargen = N.len() + n + 1;
    mk_sorterlst(&mut sorter_lst, & num_b, &mut f, Base as usize, n, &mut vargen);

    println!("{:?}",sorter_lst);
    println!("{:?}",f);

    //////////解探索開始
    // sorter 1個目の出力を上から順に0を制約として入れて、satを解く
    use varisat::solver::Solver;

    let mut zerop = 0;
    let mut solver = Solver::new();
    let mut res;
    let mut satmodel = Vec::<Lit>::new();
    
    solver.add_formula(&f);
    solver.add_formula(&mk_0cons(&sorter_lst, zerop));
    match solver.solve() {
        Ok(result) => 
            res = result,
        Err(msg) => return Err(msg.to_string()),
    }

    while res && zerop < sorter_lst[sorter_lst.len() - 1].output.len() {
        zerop += 1;
        solver = Solver::new();
        solver.add_formula(&f);
        solver.add_formula(&mk_0cons(&sorter_lst, zerop));
        match solver.solve() {
            Ok(result) => 
                res = result,
            Err(msg) => return Err(msg.to_string()),
        }
    }
    
    if !res {
        zerop -= 1;
        solver = Solver::new();
        solver.add_formula(&f);
        solver.add_formula(&mk_0cons(&sorter_lst, zerop));
        match solver.solve() {
            Ok(result) => 
                res = result,
            Err(msg) => return Err(msg.to_string()),
        }
    }
    
    println!("zerop {}", zerop);
    let mut vg = 0;

    //sorter 2個目以降
    let mut zeropos = Vec::<Option<usize>>::new();
    for i in 1..(sorter_lst.len()) {
        if sorter_lst[i].output.len() == 0 {
            zeropos.push(None);
            continue;
        }
        for j in 0..Base {
            //println!("iter {}",j);
            vg = vargen;
            solver = Solver::new();
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
            
            solver.add_formula(&mk_0cons_mod(&sorter_lst, sorter_lst.len() - 1 - i, 
                j as usize, Base as usize, &mut vg));
            
            match solver.solve() {
                Ok(result) => 
                    res = result,
                Err(msg) => return Err(msg.to_string()),
            }
            if res {
                zeropos.push(Some(j as usize));
                break;
            }
        }
        
    }

    satmodel=solver.model().unwrap();
    
    println!("-----result-----");
    println!("N = {:?}",N);
    println!("sorter_lst {:?}",sorter_lst);
    println!("zeropos {} {:?}",sorter_lst[sorter_lst.len() - 1].output.len() - zerop, zeropos);
    println!("model {:?}",satmodel);
        
    let mut q = 0;
    for i in n..(n + M.len()) {
        println!("{:?}",satmodel[i]);
        if satmodel[i] == Lit::from_dimacs((i + 1) as isize) {
            q = q + M[i-n];
        } else {

        }
        //q = q + ((sorter_lst[i].output.len() - zeropos[i]) as i32) * Base.pow(j);
        //println!("{}",q);
    }
    //println!("sorter_lst {:?}",sorter_lst);
    println!("min val, q, p = {} {} {}",q-p, q, p);
    return Ok(q-p);
}

pub fn mk_sorterlst(sorter_lst: &mut Vec<Sorter>, num_b: &Vec<Vec<i32>>, 
    g: &mut CnfFormula, Base: usize, n: usize, vargen: &mut usize) {

    let mut carr = Vec::<usize>::new();

    for i in 0..(num_b.iter().fold(0, |max, a| if max < a.len() {a.len()} else {max})) {
        let cn = carr.len();
        let mut inp = carr;

        //let mut oup = Vec::<usize>::new();
        for j in 0..(num_b.len()) {
            if num_b[j].len() > i {
                for k in 0..(num_b[j][i]) {
                    inp.push(j + n + 1);
                }
            }
        }

        let j = inp.len();
        let mut ve = Vec::<(usize,usize)>::new();
        mksorter(&mut ve, 0, j.next_power_of_two() - 1);
        //println!("ve {:?}", ve);

        let mut layer = inp.clone();
        for k in j..(j.next_power_of_two()) {
            layer.push(0);
            //inp.push(0);
        }
        //println!("inp {:?}",inp);
        for k in ve.iter() {
            let in0 = (*k).0;
            let in1 = (*k).1;

            if layer[in0] == 0 && layer[in1] == 0 {
                continue;
            } else if layer[in0] == 0 {
                layer[in0] = layer[in1];
                layer[in1] = 0;
            } else if layer[in1] == 0 {
                continue;
            } else {
                let newo1 = *vargen as isize;
                *vargen += 1;
                let newo2 = *vargen as isize;
                *vargen += 1;
                let oldi1 = layer[in0] as isize;
                let oldi2 = layer[in1] as isize;
                // oldi1 and oldi2 = newo2
                // oldi1 or oldi2 = newo1
                g.add_clause(&[!Lit::from_dimacs(oldi1), !Lit::from_dimacs(oldi2), Lit::from_dimacs(newo2)]);
                g.add_clause(&[Lit::from_dimacs(oldi1), !Lit::from_dimacs(newo2)]);
                g.add_clause(&[Lit::from_dimacs(oldi2), !Lit::from_dimacs(newo2)]);

                g.add_clause(&[Lit::from_dimacs(oldi1), Lit::from_dimacs(oldi2), !Lit::from_dimacs(newo1)]);
                g.add_clause(&[!Lit::from_dimacs(oldi1), Lit::from_dimacs(newo1)]);
                g.add_clause(&[!Lit::from_dimacs(oldi2), Lit::from_dimacs(newo1)]);

                layer[in0]=newo1 as usize;
                layer[in1]=newo2 as usize;
            }
            //println!("{:?}",layer);

        }

        let mut oup = Vec::<usize>::new();
        for k in 0..(inp.len()) {
            oup.push(layer[k]);
        }

        carr = Vec::<usize>::new();
        for k in 0..(j / (Base as usize)) {
            carr.push(oup[(k + 1) * (Base as usize) - 1]);
        }
        sorter_lst.push(Sorter {input:inp, output:oup, numcarr:cn});
    }
}

pub fn mk_0cons(stlst:& Vec<Sorter>, zeropos:usize) -> CnfFormula {
    let mut h = CnfFormula::new();
    for j in 0..zeropos {
        let k = stlst[stlst.len() - 1].output.len() - j - 1;
        h.add_clause(&[!Lit::from_dimacs(stlst[stlst.len() - 1].output[k] as isize)]);
        //println!("{}", sorter_lst[i].output[k]);
    }
    //println!("0 assigns {:?}",h);
    return h;
}

pub fn mk_0cons_mod(stlst:& Vec<Sorter>, pos: usize, l: usize, Base: usize, vargen: &mut usize) -> CnfFormula {
    let mut h = CnfFormula::new();
    let mut j = l;
    if l == 0 {
        
        let mut vl = Vec::<Lit>::new();
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
        h.add_clause(&vl);
    } else {
        let mut vl = Vec::<Lit>::new();
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
        h.add_clause(&vl);
        
    }
    //println!("h {:?}", h);
    return h;
}

pub fn mksorter(x: &mut Vec<(usize,usize)>, l: usize, h:usize) {
    if (h - l) >= 1 {
        let mid = l + ((h - l) / 2);
        mksorter(x, l, mid);
        mksorter(x, mid + 1, h);
        sorter_merge(x, l, h, 1);
    }
}

fn compare_and_swap(x: &mut Vec<(usize,usize)>, a:usize, b:usize) { 
    x.push((a,b));
}
fn sorter_merge(x: &mut Vec<(usize,usize)>, l:usize, h:usize, r:usize) {
    let step = r * 2;
    if step < h - l {
        sorter_merge(x, l, h, step);
        sorter_merge(x, l + r, h, step);
        let mut i = l + r;
        while i < h - r {
            compare_and_swap(x, i, i + r);
            i += step;
        }
    } else {
        compare_and_swap(x, l, l + r);
    }
}