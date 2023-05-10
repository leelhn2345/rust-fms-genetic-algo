# Genetic Algorithm for fleet management

a humble attempt at reimplenting an optimization algorithm for a fleet
management system, in **rust**.

[Genetic algorithm](https://towardsdatascience.com/introduction-to-optimization-with-genetic-algorithm-2f5001d9964b
) is a meta-heuristic optimization algorithm.

## Optimization flow

```sh
        Initiliazation
            |
|--->---Population
|           |
|       Mating Pool
^           |
|       Parents selection
|           |
^       Mating------------- 1. Crossover
|           |               2. Mutation
---<----Offsprings
            |
            | (Termination criteria reached)
            |
        best solution, best utility
```

## Paper References

1. Efficient genetic algorithms for optimal assignment of tasks to teams of agents
2. Optimization of task assignment to collaborating agents
3. Solving task allocation to the worker using Genetic Algorithms
4. Multi-heuristic dynamic task allocation using genetic algorithms in a heterogeneous distributed system
5. Genetic algorithm for task allocation in UAV Cooperative Control
