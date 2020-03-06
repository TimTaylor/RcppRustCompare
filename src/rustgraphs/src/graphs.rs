#![allow(clippy::many_single_char_names)]
use rand::Rng;

// TODO check all the `as` conversions. Where necessary change in to `TryInto`

/// Adjacency list Graph struct
///
/// Uses `i32` for easy compatibility with `R` (TODO - a generic would be better
/// then we would not be restricting ourselves for R compatibility).
pub struct ListGraph {
    graph: Vec<Vec<i32>>,
}

impl ListGraph {
    /// Returns the underlying vector of vector representation of the ListGraph.
    /// This consumes the ListGraph so not usable once called (TODO - consider
    /// if this is appropriate and maybe use lifetimes)
    pub fn get_vec(self) -> Vec<Vec<i32>> {
        self.graph
    }
    /// Generate an [Erdos-Renyi](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model)
    /// random graph; G(n, p).  For sparse graphs you should instead use the
    /// `fast_erdos` function.
    ///
    /// # Arguments
    ///
    /// * `n` the number of nodes in the graph
    /// * `p` the probability of an individual edge being present
    pub fn erdos(n: usize, p: f64) -> ListGraph {
        let mut rng = rand::thread_rng();
        let mut graph = vec![Vec::new(); n];
        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                if rng.gen::<f64>() < p {
                    graph[i].push(j as i32);
                    graph[j].push(i as i32);
                }
            }
        }
        ListGraph { graph }
    }

    /// Generate [Erdos-Renyi](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model)
    /// random graph; G(n, p).  For sparse graphs this will be much quicker than
    /// the `erdos` function.
    ///
    /// # Arguments
    ///
    /// * `n` the number of nodes in the graph
    /// * `p` the probability of an individual edge being present
    pub fn fast_erdos(n: usize, p: f64) -> ListGraph {
        let mut graph = vec![Vec::new(); n];
        let mut v: i32 = 1;
        let mut w: i32 = -1;
        let mut rng = rand::thread_rng();
        while v < n as i32 {
            let r = rng.gen::<f64>();
            w = w + 1 + (((1.0 - r).log10() / (1.0 - p).log10()).floor() as i32);
            while w >= v && v < n as i32 {
                w -= v;
                v += 1;
            }
            if v < n as i32 {
                graph[v as usize].push(w);
                graph[w as usize].push(v)
            }
        }
        ListGraph { graph }
    }
}
