pub struct Graph {
    size: usize,
    buffer: Vec<u32>,
    graph: Vec<Vec<u32>>,
}

impl Graph {
    pub fn with_size(size: usize) -> Graph {
        let buffer = vec![0; 6];
        let graph = vec![vec![0; size]; size];
        Graph {
            size,
            buffer,
            graph
        }
    }

    pub fn initialize(&mut self, graph6: &str) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.graph[i][j] = 0;
            }
        }
        let mut chars = graph6.trim().chars();
        chars.next();
        let mut row: usize = 0;
        let mut column: usize = 1;
        for c in chars {
            let mut byte = (c as u32) - 63;
            for i in (0..6).rev() {
                self.buffer[i] = byte % 2;
                byte /= 2;
            }
            for bit in self.buffer.iter() {
                self.graph[row][column] = *bit;
                self.graph[column][row] = *bit;
                row += 1;
                if row == column {
                    row = 0;
                    column += 1;
                }
                if column >= self.size {
                    break;
                }
            }
        }
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.graph[from][to] != 0
    }
}

use std::fmt;

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{} ", self.graph[i][j]);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
