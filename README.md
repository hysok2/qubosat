# [qubosat](https://github.com/hysok2/qubosat)

We will make the prototype of the algorithm which solves Quadratic Unconstrained Binary Optimization problems (QUBO).  Our algorithm finds a solution of QUBO by quering a SAT solver.  We expect that our algorithm have the following benefits.
1. Our algorithm is precise.  That is, the output of the algorithm is the exact solution of the given QUBO, while heuristic solvers such as tabu search based ones or simulated annealing based ones might output not an exact solution.
1. (Hopefully) our algorithm sometimes fast.  Thanks to sophisticated SAT solvers, our algorithm might terminate faster than heuristic solvers that solve QUBO.

Currently, we have the following results:
- We show the naive algorithm and its computational complexity result. [memo](https://arxiv.org/abs/2109.10048)
- We implemented the initial prototype of our algorithm, which utilizes binary search to find a solution and sorter to translate pseudo-boolean constraints into sat.  We adopt the method that uses sorter to translate pseudo-boolean constraints into sat, which is investigated by Eén & Sörensson[1].  We adopt Varisat[2] as a SAT solver.
    - [1] Eén & Sörensson: Translating pseudo-boolean constraints into SAT. In Journal on Satisfiability, Boolean Modeling and Computation, Volume 2.
    - [2] https://jix.one/project/varisat/
- We implemented 4 functionality.
    - Optimization-mode (default): Solve QUBO problem with SAT.
    - Checking-minimality-mode: Check if a given integer is a solution of a given QUBO problem.
    - Finding-assifnments-mode: Find assignments that make a given quadratic expression less than or equal to a given integer.
    - Optimization-mode-with-tabusearch: Solve QUBO problem with tabu search and SAT.

