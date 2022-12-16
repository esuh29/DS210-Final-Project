use std::collections::HashMap;
use std::fs;
use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

type Vertex = String;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyList = Vec<Vertex>;
type AdjacencyLists = HashMap<Vertex, AdjacencyList>;

#[derive(Debug)]

struct Graph {
    outedges: AdjacencyLists, 
}

// functions that implies graph that helps build the network edges in HashTable 
impl Graph {
    fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            match self.outedges.get_mut(u){
                Some(adj) => adj.push(v.to_string()),
                None => {let _ = self.outedges.insert(u.to_string(), vec![v.to_string()]);}
            }
        }
    }
    //creates graph
    fn new_empty() -> Graph{
        Graph{ 
            outedges: HashMap::new()
        }
    }
}

//function to read in csv file and put into hashmap
fn read_edges() -> Graph{
    let input = fs::read_to_string("nbaplayer_node.csv").expect("Reading the file failed");
    let mut lines = input.trim().split("\n");
    let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut graph = Graph::new_empty();
    let mut edges: ListOfEdges = Vec::new();

    for l in lines{
        let mut vertices = l.split(",");
        let a = vertices.next().unwrap().to_string();
        let b = vertices.next().unwrap().to_string();
        edges.push((a,b))
    }

    graph.add_directed_edges(&edges);
    graph     
}

fn find_degree(graph: &Graph, vertex: &str) -> usize{
    let x = graph.outedges.get(vertex);
    let a = x.unwrap().len();
    a
}


fn main(){
    //importing the graph
    let graph = read_edges();

    //finding the degree for each player
    for _key in &graph.outedges{
        let a = _key.0;
        let b = find_degree(&graph, a);
        println!("{} has {:?} degrees", a, b)
    }

    //plotting a degree distribution
    let mut data = vec![];

    for _key in &graph.outedges{
        let a = _key.0;
        let b = find_degree(&graph, a);
        data.push(b as f64)
    }

    let h = Histogram::from_slice(&data, HistogramBins::Count(10))
    .style(&BoxStyle::new().fill("burlywood"));

    let v = ContinuousView::new().x_max_ticks(11).add(h);

    Page::single(&v).save("histogram.svg").expect("saving svg");
}