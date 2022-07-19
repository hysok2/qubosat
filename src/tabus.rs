pub fn ts_qubo(mat_n : &Vec<i32>, n : usize, ln : usize) -> i32 {

    // tabu table
    let mut tt = vec![0;n];
    // 変数を変更してから、その変数変更を禁止するステップ数
    const TABU_TIME : i8 = 5;

    let mut c = vec![false;n];
    let mut val = cal(&mat_n, n, &c);
    let mut bestval = val;

    for _ in 0..ln {
        let mut pos : usize = 0;
        for x in 0..n {
            if tt[x]==0 {
                let mut cn = c.clone();
                cn[x] = !cn[x];
                let new_val = cal(&mat_n, n, &cn);

                if new_val < val {
                        pos = x;
                        val = new_val;
                }
            }
        }

        tt[pos] = TABU_TIME;
        c[pos] = !c[pos];

        for i in 0..n {
            if tt[i] > 0 {
                tt[i] = tt[i] - 1;
            }
        }
    
        if val < bestval {
            bestval = val;
        }
    }
    
    bestval
}

fn cal(mat_n: & Vec<i32>, n : usize, c: & Vec<bool>) -> i32 {
    let mut acc = 0;
    for i in 0..n {
        for j in 0..(i+1) {
            if c[i] & c[j] == true {
                acc+=mat_n[(i*(i+1))/2+j];
            } 
        }
    }
    acc
}