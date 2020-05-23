use rsat::Lit;
use rsat::Solution;

struct Sudoku {
    grid: [[u32; 9]; 9],
    solver: rsat::msat::Solver,
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-------+-------+-------+")?;
        for i in 0..9 {
            write!(f, "|")?;
            for j in 0..9 {
                if self.grid[i][j] == 0 {
                    write!(f, " _")?;
                } else {
                    write!(f, " {}", self.grid[i][j])?;
                }
                if j == 2 || j == 5 || j == 8 {
                    write!(f, " |")?;
                }
            }
            writeln!(f)?;
            if i == 2 || i == 5 || i == 8 {
                writeln!(f, "+-------+-------+-------+")?;
            }
        }
        Ok(())
    }
}

impl Sudoku {
    fn new(grid: [[u32; 9]; 9]) -> Self {
        let mut solver = rsat::msat::Solver::new(rsat::msat::SolverOptions::default());
        let mut lits = [[[Lit::new(0, false); 9]; 9]; 9];

        for lits_i in &mut lits {
            for lits_ij in lits_i.iter_mut().take(9) {
                for lits_ijk in lits_ij.iter_mut().take(9) {
                    // Cell (i, j) is assigned k+1
                    *lits_ijk = Lit::new(solver.new_var(), false);
                }
            }
        }

        // Exactly one value is assigned to each cell
        // Each horizontal line contains k exactly once
        // Each vertical line contains k exactly once
        // Each 3x3 grid contains k exactly once
        for i in 0..9 {
            for j in 0..9 {
                let mut cl = vec![];
                for k in 0..9 {
                    cl.push(lits[i][j][k]);
                    for l in 0..9 {
                        if k != l {
                            // Cell(i, j) == k+1 => Cell(i, j) != l+1 for k != l
                            solver.new_clause(vec![!lits[i][j][k], !lits[i][j][l]]);
                        }
                        if j != l {
                            // Cell(i, j) == k+1 => Cell(i, l) != k+1 for j != l
                            solver.new_clause(vec![!lits[i][j][k], !lits[i][l][k]]);
                        }
                        if i != l {
                            // Cell(i, j) == k+1 => Cell(l, j) != k+1 for i != l
                            solver.new_clause(vec![!lits[i][j][k], !lits[l][j][k]]);
                        }

                        let mod_i = (i / 3) * 3 + l / 3;
                        let mod_j = (j / 3) * 3 + l % 3;
                        if i != mod_i || j != mod_j {
                            // Cell(i, j) == k+1 => Cell(mod_i, mod_j) != k+1 for i != mod_i, j != mod_j
                            solver.new_clause(vec![!lits[i][j][k], !lits[mod_i][mod_j][k]]);
                        }
                    }
                }

                // At least one of 1..=9 is assigned to Cell(i, j)
                solver.new_clause(cl);

                if grid[i][j] != 0 {
                    // Unit clause for already assigned cells
                    solver.new_clause(vec![lits[i][j][grid[i][j] as usize - 1]]);
                }
            }
        }

        Sudoku { grid, solver }
    }

    fn solve(&mut self) {
        match self.solver.solve(vec![]) {
            Solution::Sat(sol) => {
                for i in 0..9 {
                    for j in 0..9 {
                        for k in 0..9 {
                            if sol[9 * 9 * i + 9 * j + k] {
                                if self.grid[i][j] != 0 && self.grid[i][j] != k as u32 + 1 {
                                    panic!("Something wrong, couldn't solve!");
                                }
                                self.grid[i][j] = k as u32 + 1;
                            }
                        }
                    }
                }
            }
            Solution::Unsat | Solution::Unknown | Solution::Best(_) => panic!("Couldn't solve!"),
        }
    }
}

fn main() {
    let mut sudoku = Sudoku::new(read_grid_from_stdin().unwrap());
    println!("Input:\n{}", sudoku);
    sudoku.solve();
    println!("Output:\n{}", sudoku);
}

fn read_grid_from_stdin() -> Option<[[u32; 9]; 9]> {
    let mut grid = [[0u32; 9]; 9];
    for grid_i in &mut grid {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.len() < 9 {
            return None;
        }
        for (j, grid_ij) in grid_i.iter_mut().enumerate().take(9) {
            *grid_ij = match line.chars().collect::<Vec<char>>()[j] {
                c @ '1'..='9' => (c as u32 - '0' as u32),
                _ => 0,
            };
        }
    }
    Some(grid)
}
