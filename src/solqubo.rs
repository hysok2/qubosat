use varisat::dimacs::{DimacsParser, write_dimacs};
use varisat::{CnfFormula, ExtendFormula};
use varisat::{Var, Lit};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Dvar {
    i : usize,
    k : i32,
}

#[derive(Debug)]
struct Coeff {
    id : usize,
    coeff : i32,
}

#[derive(Debug)]
struct Sorter {
    input : Vec<usize>,
    output : Vec<usize>,
}

pub fn solqubo(input:Vec<Vec<i32>>) -> i32 {
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
    
    // ソート前の添字を保存しつつソート
    let mut M = Vec::<Coeff>::new();
    for i in 0..(N.len()) {
        M.push(Coeff{id:i, coeff:N[i].abs()});
    }

    //M.sort_by({|i,j| i.coeff.cmp(&j.coeff)});
    //println!("M = {:?}", M);
    
    /*
    let mut M_ : Vec<i32> = M.iter().map(|e| e.coeff).collect();
    let mut sumM = M_.clone();

    for i in 0..(M_.len()) {
        if i == 0 {
            continue;
        } else {
            sumM[i] = sumM[i-1] + M_[i];
        }
    }
    */

    //係数をBaseで素因数分解
    let Base = 3;
    let mut num_b = Vec::<Vec<i32>>::new();
    for i in 0..(M.len()) {
        let mut tmp = Vec::<i32>::new();
        let mut m = M[i].coeff;
        while m > 0 {
            tmp.push(m % Base);
            m = m / Base;
        }
        num_b.push(tmp);
    }
    println!("bum_b {:?}",num_b);

    //sorter作成
    let mut sorter_lst = Vec::<Sorter>::new();
    let mut carr = Vec::<usize>::new();

    let mut vargen = N.len() + n + 1;
    let mut g = f;

    for i in 0..(num_b.iter().fold(0, |max, a| if max < a.len() {a.len()} else {max})) {
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
                let newo1 = vargen as isize;
                vargen += 1;
                let newo2 = vargen as isize;
                vargen += 1;
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
        sorter_lst.push(Sorter {input:inp, output:oup});
    }
    println!("{:?}",sorter_lst);
    println!("{:?}",g);

    //////////ループ開始
    // sorterの出力の上から順に0を制約として入れて、satを解く
    let mut zeropos = Vec::<usize>::new();
    for i in 0..(sorter_lst.len()) {
        zeropos.push(0);
    }
    
    let mut pos = sorter_lst.len() - 1;
    use varisat::solver::Solver;
    let mut res = false;

    let mut solver = Solver::new();
    solver.add_formula(&g);
    solver.add_formula(&mk_0cons(&sorter_lst, &zeropos));
    match solver.solve() {
        Ok(result) => 
            res = result,
        Err(_) => (),
    }
    
    while !(pos == 0 && !res) && !(pos == 0 && zeropos[pos] == sorter_lst[pos].output.len()) {

        println!("-----");
        //println!("sorter_lst {:?}",sorter_lst);
        println!("zeropos {:?}", zeropos);
        println!("{:?}",solver.model());

        if res {
            println!("sat");
        } else {
            println!("unsat");
        }

        if !res {
            decnum(& sorter_lst, &mut zeropos, &mut pos, Base as usize);
            solver = Solver::new();
            solver.add_formula(&g);
            //println!("{:?}",h);
            solver.add_formula(&mk_0cons(&sorter_lst, &zeropos));
            match solver.solve() {
                Ok(result) => 
                    res = result,
                Err(_) => (),
            }
                
        } else {
            nexnum(&sorter_lst, &mut zeropos, &mut pos, Base as usize);

            solver = Solver::new();
            solver.add_formula(&g);
            //println!("{:?}",h);
            solver.add_formula(&mk_0cons(&sorter_lst, &zeropos));
            match solver.solve() {
                Ok(result) => 
                    res = result,
                Err(_) => (),
            }
            
        }
    }
    
    println!("-----");
    println!("zeropos {:?}", zeropos);
    if res {
        println!("sat");
    } else {
        println!("unsat");
    }

    if !res {
        zeropos[pos]-=1;
    }
    
    println!("-----result-----");
    println!("sorter_lst {:?}",sorter_lst);
    println!("zeropos {:?}", zeropos);
    //println!("{:?}",solver.model());

    let mut q = 0;
    for i in 0..(zeropos.len()) {
        let j = i as u32;
        q = q + (((sorter_lst[i].output.len() - zeropos[i]) as i32) % Base) * Base.pow(j);
        //println!("{}",q);
    }
    //println!("sorter_lst {:?}",sorter_lst);
    println!("min val, q, p = {} {} {}",q-p, q, p);
    return q-p;
}

//satのあとに次に調べる制約を指定
fn nexnum(stlst: & Vec<Sorter>, zeropos : &mut Vec<usize>, pos: &mut usize, Base : usize) {
    if zeropos[*pos] < stlst[*pos].output.len() {
        zeropos[*pos]+=1;
    } else {
        *pos-=1;
        while stlst[*pos].output.len() == 0 {
            *pos-=1;
        }
        if stlst[*pos].output.len() >= Base {
            zeropos[*pos] = stlst[*pos].output.len() - ((Base - 1) as usize);
        } else {
            zeropos[*pos]+=1;
        }
    }
}

//unsatのあとに次に調べる制約を指定
fn decnum(stlst: & Vec<Sorter>, zeropos : &mut Vec<usize>, pos: &mut usize, Base : usize) {
    zeropos[*pos]-=1;
    *pos-=1;
    while stlst[*pos].output.len() == 0 {
        *pos-=1;
    }
    if stlst[*pos].output.len() >= Base {
        zeropos[*pos] = stlst[*pos].output.len() - ((Base - 1) as usize);
    } else {
        zeropos[*pos]+=1;
    }
}

fn mk_0cons(stlst:& Vec<Sorter>, zeropos:& Vec<usize>) -> CnfFormula {
    let mut h = CnfFormula::new();
    for i in 0..zeropos.len() {
        for j in 0..zeropos[i] {
            let k = stlst[i].output.len() - j - 1;
            h.add_clause(&[!Lit::from_dimacs(stlst[i].output[k] as isize)]);
            //println!("{}", sorter_lst[i].output[k]);
        }
    }
    //println!("0 assigns {:?}",h);
    return h;
}

fn mksorter(x: &mut Vec<(usize,usize)>, l: usize, h:usize) {
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