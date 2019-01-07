use std::collections::HashMap;

type Vertex = u32;
type Weight = i32;
type AdjList = HashMap<Vertex, Edge>;

pub struct Edge {
    pub src: Vertex,
    pub dst: Vertex,
    pub weight: Weight,
}

pub struct Graph {
    V: usize,
    E: usize,
    directed: bool,
    pub adj: Vec<AdjList>, /* XXX should not be public */
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            V: 0,
            E: 0,
            adj: Vec::new(),
            directed: false,
        }
    }

    pub fn new_directed() -> Graph {
        Graph {
            directed: true,
            ..Graph::new()
        }
    }

    pub fn V(&self) -> usize {
        self.V
    }

    pub fn add_edge(&mut self, src: Vertex, dst: Vertex, weight: Weight) -> bool {
        if !self.add_directed(src, dst, weight) {
            return false;
        }
        if !self.directed && !self.add_directed(dst, src, weight) {
            return false;
        }
        true
    }

    fn add_directed(&mut self, src: Vertex, dst: Vertex, weight: Weight) -> bool {
        let idx = src as usize;

        if idx == self.adj.len() {
            self.V += 1;
            self.adj.push(AdjList::new());
        } else if idx > self.adj.len() {
            // This is too strict.
            panic!("Vertex {} too high!", src);
        }

        let adj = &mut self.adj[idx];
        let insert = !adj.contains_key(&dst);

        if insert {
            self.E += 1;
            adj.insert(dst, Edge { src, dst, weight });
        } else {
            println!("Warning: edge {} → {} already present.", src, dst);
        }

        insert
    }
}
