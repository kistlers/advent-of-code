// Code for the problem https://adventofcode.com/2022/day/07
const YEAR: u32 = 2022;
const DAY: u32 = 07;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");

    let mut root: Node = Node::dir("/".to_string());
    let mut current_path = Vec::new();

    for input_line in input_lines.iter().skip(1) {
        let split = input_line.split_whitespace().collect::<Vec<_>>();

        if split[..2] == ["$", "cd"] {
            match split[2] {
                "/" => continue,
                ".." => {
                    current_path.pop();
                    continue;
                }
                name => {
                    let parent = root.get_parent(&current_path);
                    current_path.push(name.to_owned());
                    parent.children.push(Node::dir(name.to_string()));
                    continue;
                }
            }
        }
        match split[0..2] {
            ["$", "ls"] => { continue; }
            ["dir", name] => {
                let parent = root.get_parent(&current_path);
                parent.children.push(Node::dir(name.to_string()));
                continue;
            }
            [size, name] => {
                let parent = root.get_parent(&current_path);
                parent.children.push(Node::file(name.to_string(), size.parse().unwrap()));
                continue;
            }
            _ => {
                continue;
            }
        }
    }
    root.calculate_sizes();

    let part1 = root.all_children().iter()
        .filter(|node| !node.is_file && node.size <= 100000)
        .map(|node| node.size)
        .sum::<usize>();
    println!("part1: {part1}");

    let space_required = 30000000 + root.size - 70000000;
    let part2 = root.all_children().iter()
        .filter(|node| !node.is_file && node.size >= space_required)
        .map(|node| node.size)
        .min().unwrap();
    println!("part2: {part2}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: String,
    size: usize,
    is_file: bool,
    children: Vec<Node>,
}

impl Node {
    fn dir(name: String) -> Self {
        Self { name, size: 0, is_file: false, children: Vec::new() }
    }
    fn file(name: String, size: usize) -> Self {
        Self { name, size, is_file: true, children: Vec::new() }
    }

    fn get_parent(&mut self, path: &[String]) -> &mut Self {
        let mut curr = self;
        for p in path {
            curr = curr.children.iter_mut().find(|c| c.name == *p).unwrap();
        }
        curr
    }

    fn calculate_sizes(&mut self) -> usize {
        for node in &mut self.children {
            self.size += node.calculate_sizes();
        }
        self.size
    }

    fn all_children(&self) -> Vec<Self> {
        let mut all_children = Vec::new();
        for node in &self.children {
            all_children.push(node.clone());
            all_children.extend(node.all_children());
        }
        all_children
    }
}

