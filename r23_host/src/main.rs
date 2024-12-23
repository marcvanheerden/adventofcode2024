use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    name: [char; 2],
}

impl Node {
    fn new(input: &str) -> Self {
        let mut chars = input.chars();
        Self {
            name: [chars.next().unwrap(), chars.next().unwrap()],
        }
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut edges = HashMap::new();

        for line in input.lines() {
            let (left, right) = line.split_once('-').unwrap();
            let left_node = Node::new(left);
            let right_node = Node::new(right);

            edges
                .entry(left_node)
                .and_modify(|nodes: &mut Vec<Node>| nodes.push(right_node))
                .or_insert(vec![right_node]);

            edges
                .entry(right_node)
                .and_modify(|nodes: &mut Vec<Node>| nodes.push(left_node))
                .or_insert(vec![left_node]);
        }

        Graph { edges }
    }
}

fn three_comb(nodes: Vec<Node>, graph: &Graph) -> Vec<[Node; 3]> {
    let mut output = Vec::new();
    for idx1 in 0..(nodes.len() - 2) {
        for idx2 in (idx1 + 1)..(nodes.len() - 1) {
            for idx3 in (idx2 + 1)..nodes.len() {
                let node1 = nodes[idx1];
                let node2 = nodes[idx2];
                let node3 = nodes[idx3];
                let connections1 = graph.edges.get(&node1).unwrap();
                let connections2 = graph.edges.get(&node2).unwrap();

                if connections1.contains(&node2)
                    && connections1.contains(&node3)
                    && connections2.contains(&node3)
                    && [node1, node2, node3].iter().any(|n| n.name[0] == 't')
                {
                    output.push([node1, node2, node3]);
                }
            }
        }
    }

    output
}

fn part1(graph: &Graph) -> usize {
    let nodes: Vec<_> = graph.edges.keys().cloned().collect();
    three_comb(nodes, graph).len()
}

fn part2(graph: &Graph) -> String {
    let mut nodes: Vec<_> = graph.edges.keys().cloned().collect();
    nodes.sort_unstable();

    let mut immediate_networks: Vec<Vec<&Node>> = Vec::new();

    for node1 in nodes.iter() {
        dbg!(node1);
        for node2 in nodes.iter() {
            if node2 <= node1 || !graph.edges.get(node1).unwrap().contains(node2) {
                continue;
            }

            let mut stack = vec![vec![node1, node2]];

            while let Some(task) = stack.pop() {
                let mut found = false;
                for (node, edges) in graph.edges.iter() {
                    if node <= task.last().unwrap() {
                        continue;
                    }
                    if !task.contains(&node) && task.iter().all(|n| edges.contains(n)) {
                        let mut new_task = task.clone();
                        new_task.push(node);
                        stack.push(new_task);
                        found = true;
                    }
                }
                if !found {
                    immediate_networks.push(task);
                }
            }
        }
    }

    let max_len = immediate_networks.iter().map(|v| v.len()).max().unwrap();

    let mut longest: Vec<&Node> = immediate_networks
        .into_iter()
        .find(|v| v.len() == max_len)
        .unwrap();
    longest.sort_unstable();

    longest
        .into_iter()
        .cloned()
        .flat_map(|n| vec![n.name[0], n.name[1], ','])
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let graph = Graph::new(input);
    dbg!(part1(&graph));
    dbg!(part2(&graph));
}
