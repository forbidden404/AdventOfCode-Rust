use std::path::Path;

trait Bit {
    fn set_bit(&mut self, pos: usize);
    fn clear_bit(&mut self, pos: usize);
    fn get_bit(&self, pos: usize) -> bool;
    fn toggle_bit(&mut self, pos: usize);
    /// Returns a number with the first n bits set to 1
    fn set_all(&mut self, pos: usize);
    fn is_power_of_two(&self) -> bool;
}

macro_rules! impl_bit {
    ($type:ty) => {
        impl Bit for $type {
            fn set_bit(&mut self, pos: usize) {
                *self |= (1 << pos);
            }
            fn clear_bit(&mut self, pos: usize) {
                *self &= !(1 << pos);
            }
            fn get_bit(&self, pos: usize) -> bool {
                (self >> pos) % 2 != 0
            }
            // Toggles the i'th bit from 0 -> 1 or 1 -> 0
            fn toggle_bit(&mut self, pos: usize) {
                *self ^= (1 << pos);
            }
            fn set_all(&mut self, pos: usize) {
                *self = (1 << pos) - 1;
            }
            fn is_power_of_two(&self) -> bool {
                *self > 0 && *self & (*self - 1) == 0
            }
        }
    };
}

pub fn main_2015_9() {
    let path = Path::new("src/_2015/inputs/day09.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process(&input);
}

use std::collections::HashMap;
fn process(input: &str) {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut dists: HashMap<(usize, usize), usize> = HashMap::new();
    let mut index = 0;
    for line in input.lines() {
        let splits: Vec<&str> = line.split(" = ").collect();
        let distance = splits[1].parse::<usize>().unwrap();
        let cities: Vec<&str> = splits[0].split(" to ").collect();
        let (lhs, rhs) = (cities[0].to_owned(), cities[1].to_owned());

        let index0 = map.entry(lhs).or_insert(index).clone();
        if index0 == index { index = index + 1; }

        let index1 = map.entry(rhs).or_insert(index).clone();
        if index1 == index { index = index + 1; }

        dists.insert((index0, index1), distance);
    }

    let mut adj_list = WeightedAdjacencyList::with_size(map.len() + 1);
    for ((from, to), weight) in dists.into_iter() {
        adj_list.add_undirected_edge(from, to, weight as f64);
    }

    for i in 0..index {
        adj_list.add_undirected_edge(index, i, 1.);
    }

    let adj_matrix = WeightedAdjacencyMatrix::from(adj_list);
    let ans = tsp_solver(&adj_matrix, index, f64::INFINITY, |a, b| a < b);
    println!("Minimum distance: {}", ans.0 - 2.0);
    println!("Tour: {:?}", ans.1);

    let ans = tsp_solver(&adj_matrix, index, 0.0, |a, b| a > b);
    println!("Maximum distance: {}", ans.0 - 2.0);
    println!("Tour: {:?}", ans.1);
}

fn tsp_solver(distance: &WeightedAdjacencyMatrix, start: usize, starting_state: f64, comparison: fn(f64, f64) -> bool) -> (f64, Vec<usize>) {
    let n = distance.node_count();

    let mut memo = vec![vec![starting_state; 1 << n]; n];
    for i in 0..n {
        memo[i][1 << i | 1 << start] = distance[start][i];
    }
    for r in 3..=n {
        for state in BinaryCombinations::new(n, r as u32).filter(|state| state.get_bit(start)) {
            for next in (0..n).filter(|&node| state.get_bit(node) && node != start) {
                let prev_state = state ^ (1 << next);
                let mut cmp_dist = starting_state;
                for prev_end in
                    (0..n).filter(|&node| state.get_bit(node) && node != start && node != next)
                {
                    let new_dist = memo[prev_end][prev_state] + distance[prev_end][next];
                    if comparison(new_dist, cmp_dist) {
                        cmp_dist = new_dist;
                    }
                }
                memo[next][state] = cmp_dist;
            }
        }
    }

    let end_state = (1 << n) - 1;
    let mut cmp_dist = starting_state;
    for e in (0..start).chain(start + 1..n) {
        let dist = memo[e][end_state] + distance[e][start];
        if comparison(dist, cmp_dist) {
            cmp_dist = dist;
        }
    }

    let mut state = end_state;
    let mut last_index = start;
    let mut tour = vec![start];
    for _ in 1..n {
        let mut best_j = usize::MAX;
        let mut best_dist = starting_state;
        for j in (0..n).filter(|&j| state.get_bit(j) && j != start) {
            let dist = memo[j][state] + distance[j][last_index];
            if comparison(dist, best_dist) {
                best_j = j;
                best_dist = dist;
            }
        }
        tour.push(best_j);
        state ^= 1 << best_j;
        last_index = best_j;
    }

    (cmp_dist, tour) 
}

struct BinaryCombinations {
    curr: usize,
    r: u32,
    n: usize,
}

impl Iterator for BinaryCombinations {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.curr..1 << self.n {
            if i.count_ones() == self.r {
                self.curr = i + 1;
                return Some(i);
            }
        }
        None
    }
}

impl BinaryCombinations {
    fn new(n: usize, r: u32) -> Self {
        Self { curr: 0, r, n }
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
    weight: f64,
}

impl Edge {
    fn new(to: usize, weight: f64) -> Self {
        Self { to, weight }
    }
}

struct WeightedAdjacencyList {
    inner: Vec<Vec<Edge>>,
}

impl WeightedAdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    fn with_size(n: usize) -> Self {
        Self {
            inner: vec![vec![]; n],
        }
    }
    /// Add a directed edge from node `u` to node `v` with weight `weight`.
    fn add_directed_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.inner[u].push(Edge::new(v, weight))
    }
    /// Add an undirected edge between nodes `u` and `v`.
    fn add_undirected_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.add_directed_edge(u, v, weight);
        self.add_directed_edge(v, u, weight);
    }
    /// Iterates over all nodes in the graph.
    /// Each item is a tuple of the node id and a `Vec` of all its outgoing edges
    pub fn nodes(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.inner.iter().enumerate()
    }
    /// Number of nodes (vertices) in the graph
    pub fn node_count(&self) -> usize {
        self.inner.len()
    }
}

/// Allows the outgoing edges of a node to be accessed easily.
impl std::ops::Index<usize> for WeightedAdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

struct WeightedAdjacencyMatrix {
    inner: Vec<Vec<f64>>,
}

impl WeightedAdjacencyMatrix {
    fn with_size(n: usize) -> Self {
        let mut inner = vec![vec![f64::INFINITY; n]; n];
        for i in 0..n {
            inner[i][i] = 0.;
        }
        Self { inner }
    }

    fn node_count(&self) -> usize {
        self.inner.len()
    }

    fn from_adjacency_list(inp: &WeightedAdjacencyList) -> Self {
        let mut res = Self::with_size(inp.node_count());
        for (from, edges) in inp.nodes() {
            for &Edge { to, weight } in edges {
                res.inner[from][to] = weight;
            }
        }
        res
    }

    fn from_inner(matrix: Vec<Vec<f64>>) -> Self {
        Self { inner: matrix }
    }
}

impl std::ops::Index<usize> for WeightedAdjacencyMatrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl From<WeightedAdjacencyList> for WeightedAdjacencyMatrix {
    fn from(inp: WeightedAdjacencyList) -> Self {
        Self::from_adjacency_list(&inp)
    }
}

impl From<Vec<Vec<f64>>> for WeightedAdjacencyMatrix {
    fn from(matrix: Vec<Vec<f64>>) -> Self {
        Self::from_inner(matrix)
    }
}

impl_bit!(u8);
impl_bit!(u16);
impl_bit!(u32);
impl_bit!(usize);