# [qubosat](https://github.com/hysok2/qubosat)

We will make the prototype of the algorithm which solves Quadratic Unconstrained Binary Optimization problems (QUBO).  Our algorithm finds a solution of QUBO by quering a SAT solver.  We expect that our algorithm have the following benefits.
1. Our algorithm is ***precise***.  That is, the output of the algorithm is the exact solution of the given QUBO, while heuristic solvers such as tabu search based ones or simulated annealing based ones might output not an exact solution.
1. (Hopefully) our algorithm sometimes fast.  Thanks to sophisticated SAT solvers, our algorithm might terminate faster than heuristic solvers that solve QUBO.

Currently, we have the following results:
- We show the naive algorithm and its computational complexity result. [Report](https://arxiv.org/abs/2109.10048)
- We implemented the initial prototype of our algorithm.  We adopt the method that uses sorter to translate pseudo-boolean constraints into sat, which is investigated by Eén & Sörensson[1].  We adopt Varisat[2] as a SAT solver.
    - [1] Eén & Sörensson: Translating pseudo-boolean constraints into SAT. In Journal on Satisfiability, Boolean Modeling and Computation, Volume 2.
    - [2] https://jix.one/project/varisat/
- We implemented 4 functionality.
    - Optimization-mode (default): Solve QUBO problem with SAT.
    - Checking-minimality-mode: Check if a given integer is a solution of a given QUBO problem.
    - Finding-assignments-mode: Find assignments that make a given quadratic expression less than or equal to a given integer.
    - Optimization-mode-with-tabusearch: Solve a QUBO problem with tabu search and SAT.
- Examples
    ``` Optimization-mode
    $ cargo run -- samples/8.qub
    -----------------------------------------------------------------
    Optimization-mode
    The base number is 2
    Input: an integer matrix Q (defined in samples/8.qub)
    Output: minimum value q and assignments x s.t. min_x x^T Q x = q
    -----------------------------------------------------------------
    -----result-----
    x = -1 2 3 -4 5 -6 7 -8
    q = -6889
    ```
    ``` Checking-minimality-mode
    $ cargo run -- -c '-6890' samples/8.qub
    -----------------------------------------------------------------
    Checking-minimality-mode
    The base number is 2
    Input: an integer matrix Q (defined in samples/8.qub), an integer q = -6890
    Output: true (minimum) or false (not minimum)
    -----------------------------------------------------------------
    false
    $ cargo run -- -c '-6889' samples/8.qub
    -----------------------------------------------------------------
    Checking-minimality-mode
    The base number is 2
    Input: an integer matrix Q (defined in samples/8.qub), an integer q = -6889
    Output: true (minimum) or false (not minimum)
    -----------------------------------------------------------------
    true
    ```
    ``` Finding-assignments-mode
    $ cargo run -- -f '-6889' samples/8.qub
    -----------------------------------------------------------------
    Finding-assignments-mode
    The base number is 2
    Input: an integer matrix Q (defined in samples/8.qub), an integer q = -6889
    Output: assignments x s.t. x^T Q x <= q
    -----------------------------------------------------------------
    -----result-----
    x = -1 -2 -3 4 5 -6 -7 8
    true
    $ cargo run -- -f '-6890' samples/8.qub
    -----------------------------------------------------------------
    Finding-assignments-mode
    The base number is 2
    Input: an integer matrix Q (defined in samples/8.qub), an integer q = -6890
    Output: assignments x s.t. x^T Q x <= q
    -----------------------------------------------------------------
    false
    ```
    ``` Optimization-mode-with-tabusearch
    $ cargo run -- -t samples/8.qub
    -----------------------------------------------------------------
    Optimization-mode-with-tabusearch
    The base number is 2
    Input: an integer matrix Q (defined in samples/8.qub)
    Output: minimum value q and assignments x s.t. min_x x^T Q x = q
    -----------------------------------------------------------------
    -----result-----
    x = -1 2 3 -4 5 -6 7 -8
    q = -6889
    ```
    If you want to change the base number, please use -b option.  Changing base number affects the size of the SAT problem that the program produces from the given QUBO problem.  Therefore, chosing an appropriate base number might reduce the size of the SAT problem, and hence the program might solve the problem faster.  Note that chaning the base number does not affect the output of the program.
    ```
    $ cargo run -- -b 5 samples/8.qub
    -----------------------------------------------------------------
    Optimization-mode
    The base number is 5
    Input: an integer matrix Q (defined in samples/8.qub)
    Output: minimum value q and assignments x s.t. min_x x^T Q x = q
    -----------------------------------------------------------------
    -----result-----
    x = 1 2 3 -4 -5 6 7 -8
    q = -6889
    $ cargo run -- -c '-6889' -b 5 samples/8.qub
    -----------------------------------------------------------------
    Checking-minimality-mode
    The base number is 5
    Input: an integer matrix Q (defined in samples/8.qub), an integer q = -6889
    Output: true (minimum) or false (not minimum)
    -----------------------------------------------------------------
    true
    ```