use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    name: String,
    output: Vec<String>,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let output = if parts.len() > 1 {
            parts[1].split_whitespace().map(|s| s.to_string()).collect()
        } else {
            vec![]
        };
        Ok(Node { name, output })
    }
}

fn count_paths_internal(
    node_name: &str,
    nodes: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&count) = memo.get(node_name) {
        return count;
    }
    let count = if let Some(outputs) = nodes.get(node_name) {
        outputs
            .iter()
            .map(|output| count_paths_internal(output, nodes, memo))
            .sum()
    } else {
        0
    };
    memo.insert(node_name.to_string(), count);
    count
}

fn count_paths(node_src: &str, node_dest: &str, nodes: &HashMap<String, Vec<String>>) -> u64 {
    let mut memo: HashMap<String, u64> = HashMap::new();
    memo.insert(node_dest.to_string(), 1);
    count_paths_internal(node_src, nodes, &mut memo)
}

pub fn part1(input: &str) -> String {
    let nodes: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| line.parse::<Node>())
        .collect::<Result<Vec<Node>, ()>>()
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut acc, node| {
            acc.insert(node.name.clone(), node.output.clone());
            acc
        });

    count_paths("you", "out", &nodes).to_string()
}

pub fn part2(input: &str) -> String {
    let nodes: HashMap<String, Vec<String>> = input
        .lines()
        .map(|line| line.parse::<Node>())
        .collect::<Result<Vec<Node>, ()>>()
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut acc, node| {
            acc.insert(node.name.clone(), node.output.clone());
            acc
        });

    let svr2dac = count_paths("svr", "dac", &nodes);
    let dac2fft = count_paths("dac", "fft", &nodes);
    let fft2out = count_paths("fft", "out", &nodes);

    let svr2fft = count_paths("svr", "fft", &nodes);
    let fft2dac = count_paths("fft", "dac", &nodes);
    let dac2out = count_paths("dac", "out", &nodes);

    (svr2dac * dac2fft * fft2out + svr2fft * fft2dac * dac2out).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;
        assert_eq!(part1(input), "5");
    }

    #[test]
    fn test_part2() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
        assert_eq!(part2(input), "2");
    }
}
