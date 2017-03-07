# Maze generation

## Task
Generate and show a maze, using the simple Depth-first search algorithm.

1. Start at a random cell.
2. Mark the current cell as visited, and get a list of its neighbours. For each neighbour, starting with a randomly selected neighbour: If that neighbour hasn't been visited, remove the wall between this cell and that neighbour, and then recurse with that neighbour as the current cell.
