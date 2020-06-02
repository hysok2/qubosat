# qubosat

In this sandbox, we will make the prototype of the algorithm which solves Quadratic Unconstrained Binary Optimization problems (QUBO).  Our algorithm finds a solution of QUBO by quering a SAT solver.  We expect that our algorithm have the following benefits.
1. Our algorithm is precise.  That is, the output of the algorithm is the exact solution of the given QUBO, while heuristic solvers such as tabu search based ones or simulated annealing based ones might output not a exact solution.
1. (Hopefully) our algorithm sometimes fast.  Thanks to a sophisticated SAT solver, our algorithm might terminate faster than heuristic solvers that solve QUBO.

Currently, we show the naive algorithm and its computational complexity result. 
[memo](https://github.com/hysok2/qubosat/blob/master/qubo2sat.pdf)
