# qubosat

In this sandbox, we will make the prototype of the algorithm which solves Quadratic Unconstrained Binary Optimization problems (QUBO).  Our algorithm finds a solution of QUBO by quering a SAT solver.  We expect that our algorithm have the following benefits.
1. Our algorithm is precise.  That is, the output of the algorithm is the exact solution of the given QUBO, while heuristic solvers such as tabu search based ones or simulated annealing based ones might output not an exact solution.
1. (Hopefully) our algorithm sometimes fast.  Thanks to a sophisticated SAT solver, our algorithm might terminate faster than heuristic solvers that solve QUBO.

Currently, we have the following results:
- We show the naive algorithm and its computational complexity result. [memo](https://github.com/hysok2/qubosat/blob/master/qubo2sat.pdf)
- We implemented the prototype of our algorithm, which utilizes binary search to find a solution and sorter to translate pseudo-boolean constraints into sat.  The technique that uses sorter to translate pseudo-boolean constraints into sat is investigated by Eén & Sörensson[1]. We adopt Varisat[2] as a SAT solver.
    - [1] Eén & Sörensson: Translating pseudo-boolean constraints into SAT. In Journal on Satisfiability, Boolean Modeling and Computation, Volume 2.
    - [2] https://jix.one/project/varisat/
