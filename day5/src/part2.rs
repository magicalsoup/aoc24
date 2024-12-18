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


fn build_graph(edges: &Vec<Edge>) -> Vec<Vec<usize>>  {
    let mut graph: Vec<Vec<usize>> = vec![Vec::<usize>::new(); MAX_NUM_NODES];
    for edge in edges {
        graph[edge.from].push(edge.to);
    }
    graph
}

fn dfs(graph: &Vec<Vec<usize>>, u: &usize, vis: &mut Vec<bool>, order: &mut Vec<usize>, enabled: &HashMap<usize, bool>) {
    vis[*u] = true;
    for v in graph[*u].as_slice() {
        if !vis[*v] && enabled.get(v).copied().unwrap_or(false) {
            dfs(graph, v, vis, order, enabled);
        }
    }
    order.push(*u);
}

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
    let mut pos = 1;
    let mut ord = vec![0; MAX_NUM_NODES];
    let mut ordered = true;
    // println!("{}: ", query);
    for page in pages {
        let p: usize = page.parse().unwrap();
        enabled.insert(p, true);
        ord[p] = pos.clone();
        pos += 1;
    }


    for edge in edges {
        if !enabled.get(&edge.from).copied().unwrap_or(false) 
            || !enabled.get(&edge.to).copied().unwrap_or(false) {
            continue;
        } 

        if ord[edge.from] > ord[edge.to] {
            ordered = false;
            break;
        }
    }

    if !ordered {

        let mut vis = vec![false; MAX_NUM_NODES];
        let graph = build_graph(&edges);
        let mut order: Vec<usize> = Vec::new();

        for i in 0..MAX_NUM_NODES {
            if !vis[i] && enabled.get(&i).copied().unwrap_or(false) {
                dfs(&graph, &i, &mut vis, &mut order, &enabled);
            }
        }

        order.reverse();
        *sum += order[order.len() / 2] as i32;
    }    
} 