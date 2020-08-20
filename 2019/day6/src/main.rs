use std::iter::successors;

type Graph<'a> = std::collections::BTreeMap<&'a str, &'a str>;

fn parse_graph(s: &str) -> Graph {
    s
    .lines()
    .filter_map(|planet| Some((planet.find(')')?, planet)))
    .map(|(idx, line)| (&line[idx + 1..], &line[..idx]))
    .collect()
}

fn walk<'a>(graph: &'a Graph, to: &'a str) -> impl Iterator<Item = &'a str> {
    successors(Some(to), move |to| graph.get(to).copied()).skip(1)
}

fn count_orbits(graph: &Graph) -> usize {
    graph.keys().fold(0, |num, to| num + walk(graph, to).count())
}
fn measure_path(graph: &Graph, a: &str, b: &str) -> Option<usize> {
    let b_orbits: Vec<_> = walk(graph, b).collect();
    walk(graph, a)
        .enumerate()
        .filter_map(|(d, p)| Some(d + b_orbits.iter().rposition(|&x| x == p)?))
        .next()
}

fn main() {
    let planets = parse_graph(include_str!("../input.txt"));
    println!("part 1: {:?}", count_orbits(&planets));
    println!("part 2: {:?}", measure_path(&planets, "YOU", "SAN").unwrap());
}