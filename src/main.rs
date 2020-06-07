use varisat::dimacs::{DimacsParser, write_dimacs};
use varisat::{CnfFormula, ExtendFormula};
use varisat::{Var, Lit};

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Dvar {
    i : usize,
    k : i32,
}

#[derive(Debug)]
struct Coeff {
    id : usize,
    coeff : i32,
}

fn main() {
    let input = [[2,0],[3,4]];
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

    // pとvの関係をCNFで記述、Nの要素のminusを考慮
    let mut f = CnfFormula::new();
    for i in 0..n {
        for j in 0..(i+1) {
            if N[(i*(i+1))/2+j] >= 0 {
                let x = (i+1) as isize;
                let y = (j+1) as isize;
                let z = ((i*(i+1))/2+j+1+n) as isize;
                f.add_clause(&[!Lit::from_dimacs(x), !Lit::from_dimacs(y), Lit::from_dimacs(z)]);
                f.add_clause(&[!Lit::from_dimacs(x), Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(x), !Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(x), Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
      
            } else {
                let x = (i+1) as isize;
                let y = (j+1) as isize;
                let z = ((i*(i+1))/2+j+1+n) as isize;
                f.add_clause(&[!Lit::from_dimacs(x), !Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
                f.add_clause(&[Lit::from_dimacs(x), !Lit::from_dimacs(y), Lit::from_dimacs(z)]);
                f.add_clause(&[!Lit::from_dimacs(x), Lit::from_dimacs(y), Lit::from_dimacs(z)]);
                f.add_clause(&[!Lit::from_dimacs(x), !Lit::from_dimacs(y), !Lit::from_dimacs(z)]);
            }
        }
    }
    
    //println!("{:?}", N);
    //println!("{:?}", f);

    // ソート前の添字を保存しつつソート
    let mut M = Vec::<Coeff>::new();
    for i in 0..(N.len()) {
        M.push(Coeff{id:i, coeff:N[i].abs()});
    }

    M.sort_by({|i,j| i.coeff.cmp(&j.coeff)});
    //println!("M = {:?}", M);
 
    let mut M_ : Vec<i32> = M.iter().map(|e| e.coeff).collect();
    let mut sumM = M_.clone();

    for i in 0..(M_.len()) {
        if i == 0 {
            continue;
        } else {
            sumM[i] = sumM[i-1] + M_[i];
        }
    }

    //////////ループ開始
    let mut lb : i32 = N.iter().fold(0, |s, c| if *c<0 {*c+s} else {s}) - 1;
    let mut ub : i32 = 0;

    ub = 6;

    while lb + 1 < ub {

        println!("-----");
        let mut vargen = N.len() + n + 1;

        //let i = N.len();
        let q = (lb + ub) / 2;
        let k = p + q;

        println!("k = {}",k);

        let mut Ds = HashMap::new();
        let mut dic = HashSet::<Dvar>::new();

        subg(&mut dic,N.len()-1,k,&M_,&sumM);

        //println!("{:?}",dic);

        // pの線形式をcnfに
        let mut g = CnfFormula::new();
        for h in dic.iter() {
            if h.k == 0 {
                let varid = addget_varid(&mut Ds, h, &mut vargen);
                let mut vv = Vec::<Lit>::new();
                vv.push(Lit::from_dimacs(varid as isize));
                for i in 0..(h.i+1) {
                    vv.push(Lit::from_dimacs((i+n) as isize));
                    g.add_clause(&[!Lit::from_dimacs(varid as isize),!Lit::from_dimacs((i+n) as isize)]);
                }
                g.add_clause(&vv);
            } else if h.k < 0 {
                let varid = addget_varid(&mut Ds, h, &mut vargen);
                g.add_clause(&[!Lit::from_dimacs(varid as isize)]);
            } else if h.k >= sumM[h.i] {
                let varid = addget_varid(&mut Ds, h, &mut vargen);
                g.add_clause(&[Lit::from_dimacs(varid as isize)]);
            } else {
                let (i,k) = (h.i, h.k);

                let varid0 = addget_varid(&mut Ds, h, &mut vargen);
                let varid1 = addget_varid(&mut Ds, &Dvar{i:i-1,k:k-M_[i-1]}, &mut vargen);
                let varid2 = addget_varid(&mut Ds, &Dvar{i:i-1,k:k}, &mut vargen);
                g.add_clause(&[!Lit::from_dimacs(varid1 as isize),Lit::from_dimacs(varid0 as isize)]);
                g.add_clause(&[!Lit::from_dimacs(varid0 as isize),Lit::from_dimacs(varid2 as isize)]);
                g.add_clause(&[!Lit::from_dimacs(varid0 as isize),!Lit::from_dimacs((i+n) as isize),Lit::from_dimacs(varid1 as isize)]);
                g.add_clause(&[!Lit::from_dimacs(varid2 as isize),Lit::from_dimacs((i+n) as isize),Lit::from_dimacs(varid0 as isize)]);
            }
        }
        println!("Ds {:?}",Ds);
        //println!("{:?}",g);

        // sat
        use varisat::solver::Solver;

        let mut solver = Solver::new();
        let (v,id) = Ds.get_key_value(&(Dvar{i:N.len()-1,k:k})).unwrap();

        //println!("v,id = {:?} {}",*v,*id);
        //solver.add_clause(&[Lit::from_dimacs(id)]);
        solver.add_clause(&[Lit::from_dimacs(*id as isize)]);
        solver.add_formula(&g);
        solver.add_formula(&f);
        match solver.solve() {
            Ok(result) => {
                if result {ub = k;} else {lb = k;}
            }
            Err(_) => ()
        }
        //println!("sol = {:?}", solver.model());
    }
    println!("{}", ub);
}

fn subg(d:&mut HashSet<Dvar>, i:usize, k:i32, M:& Vec<i32>, sumM:& Vec<i32>) {
    d.insert(Dvar{i:i,k:k});
    if !(i<=0 || k <= 0 || k >= sumM[i]) {
        subg(d,i-1,k,M,sumM);
        subg(d,i-1,k-M[i],M,sumM);
    }
}

fn addget_varid(d: &mut HashMap<Dvar,usize>, h: & Dvar, vargen: &mut usize)-> usize {
    let varid = match d.get(h) {
        Some(v) => *v,
        None => 
        {   
            d.insert(*h,*vargen);
            *vargen += 1;
            *vargen - 1
        }
    };
    varid
}