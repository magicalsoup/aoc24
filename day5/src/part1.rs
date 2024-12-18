use std::collections::HashMap;

const MAX_NUM_NODES: usize = 100;

struct Edge {
    from: usize,
    to: usize
}


pub fn solve(init_lines: &Vec<String>, query_lines: &Vec<String>) {

    let mut sum = 0;
    let edges = get_edges(init_lines);

    for query in query_lines {
      check_ordered(&edges, query, &mut sum);
    }
    println!("{}", sum);
}



// fn build_graph(init_lines: &Vec<String>) -> (Vec<Vec<usize>>, usize) {
//     let mut mx_node_sz: usize = 0;
//     for line in init_lines {
//         let parts: Vec<&str> = line.split("|").collect();
//         let u: usize = parts[0].parse().unwrap();
//         let v: usize = parts[1].parse().unwrap();
//         mx_node_sz = max(mx_node_sz, u);
//         mx_node_sz = max(mx_node_sz, v);
//     }

//     let mut graph: Vec<Vec<usize>> = vec![Vec::<usize>::new(); mx_node_sz + 1];
//     for line in init_lines {
//         let parts: Vec<&str> = line.split("|").collect();
//         let u: usize = parts[0].parse().unwrap();
//         let v: usize = parts[1].parse().unwrap();
//         graph[u].push(v);
//     }

//     (graph, mx_node_sz)
// }

// fn dfs(graph: &Vec<Vec<usize>>, u: &usize, vis: &mut Vec<bool>, order: &mut Vec<usize>) {
//     vis[*u] = true;
//     for v in graph[*u].as_slice() {
//         if !vis[*v] {
//             dfs(graph, v, vis, order);
//         }
//     }
//     order.push(*u);
// }

fn get_edges(init_lines: &Vec<String>) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for line in init_lines {
        let parts: Vec<&str> = line.split("|").collect();
        let u: usize = parts[0].parse().unwrap();
        let v: usize = parts[1].parse().unwrap();
        edges.push(Edge{from: u, to: v});
    }
    edges
}

fn check_ordered(edges: &Vec<Edge>, query: &String, sum: &mut i32) {
    let pages: Vec<&str> = query.split(",").collect();
    let mut enabled: HashMap<usize, bool> = HashMap::new();
    let len = pages.len();
    let mut pos = 1;
    let mut ord = vec![0; MAX_NUM_NODES];
    let mut ordered = true;
    let middle = pages[len / 2].parse::<i32>().unwrap().clone();

    // println!("{}: ", query);
    for page in pages {
        let p: usize = page.parse().unwrap();
        enabled.insert(p, true);
        ord[p] = pos.clone();
        pos += 1;
    }


    for edge in edges {
        if !enabled.get(&edge.from).copied().unwrap_or(false) || !enabled.get(&edge.to).copied().unwrap_or(false) {
            continue;
        } 

        if ord[edge.from] > ord[edge.to] {
            ordered = false;
            break;
        }
    }

    if ordered {
      *sum += middle;
    }
   
} 