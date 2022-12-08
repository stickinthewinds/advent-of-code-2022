use std::collections::HashMap;

use id_tree::{Tree, Node, NodeId, InsertBehavior::*};

#[derive(Clone, Debug)]
enum Entry {
    Dir(String),
    File(i32, String),
}

#[derive(Debug)]
struct TreeEntry {
    loc: Entry,
}

impl TreeEntry {
    fn size(&self) -> i32 {
        match self.loc {
            Entry::Dir(_) => 0,
            Entry::File(size, _) => size,
        }
    }
}

pub fn part_one(lines: Vec<String>) -> i32 {
    let lines: Vec<&String> = lines.iter().filter(|&line| !line.contains("$ ls")).collect();
    let dir_tree = build_tree(lines);
    let mut s = String::new();
    dir_tree.write_formatted(&mut s).unwrap();
    println!("{s}");
    let result = calculate_sizes_for_tree(&dir_tree)
        .iter()
        .filter(|&&v| v <= 100000)
        .sum::<i32>();
    result
}

pub fn part_two(lines: Vec<String>) -> i32 {
    let lines: Vec<&String> = lines.iter().filter(|&line| !line.contains("$ ls")).collect();
    let dir_tree = build_tree(lines);
    let mut result = calculate_sizes_for_tree(&dir_tree);

    result.sort();

    // just assumes that the available is less than the amount needed
    let current_available = 70000000 - result.clone().iter().max().unwrap();
    let amount_needed = 30000000 - current_available;

    *(*result
        .iter()
        .filter(|&&v| v >= amount_needed)
        .collect::<Vec<&i32>>()
        .first()
        .unwrap())
}

fn calculate_sizes_for_tree(dir_tree: &Tree<TreeEntry>) -> Vec<i32> {
    dir_tree
        .traverse_pre_order(dir_tree.root_node_id().unwrap())
        .unwrap()
        .filter(|node| {
            match node.data().loc {
                Entry::Dir(_) => true,
                Entry::File(_, _) => false,
            }
        })
        .map(|node| calculate_size(dir_tree, node))
        .collect()
}

fn calculate_size(dir_tree: &Tree<TreeEntry>, node: &Node<TreeEntry>) -> i32 {
    let mut total = node.data().size();
    node.children().iter().for_each(|c| {
        total += calculate_size(dir_tree, dir_tree.get(c).unwrap());
    });
    total
}

fn build_tree(lines: Vec<&String>) -> Tree<TreeEntry> {
    let mut nodes: HashMap<String, NodeId> = HashMap::new();

    let root = *lines.last().unwrap();
    let root_node = Node::new(TreeEntry { loc: Entry::Dir(root.to_string()) });
    let mut dir_tree: Tree<TreeEntry> = Tree::new();

    let mut current_id = dir_tree.insert(root_node, AsRoot).unwrap();
    nodes.insert(root.clone(), current_id.clone());
    let lines = lines[1..].to_vec();


    for line in lines {
        let parts: Vec<String> = line.split_whitespace().map(|p| p.to_string()).collect();
        if parts.len() == 3 {
            let name = parts.last().unwrap().to_string();
            current_id = if name == ".." {
                dir_tree
                    .get(&current_id)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .clone()
            } else {
                dir_tree
                    .insert(
                        Node::new(TreeEntry { loc: Entry::Dir(name.clone()) }), UnderNode(&current_id))
                    .unwrap()
            };
            nodes.insert(name.clone(), current_id.clone());
        } else {
            let name = parts.last().unwrap().to_string();
            if let Ok(size) = parts.first().unwrap().parse::<i32>() {
                let new_node_id = dir_tree
                    .insert(
                        Node::new(TreeEntry { loc: Entry::File(size, name.clone()) }),
                        UnderNode(&current_id))
                    .unwrap();
                nodes.insert(name.clone(), new_node_id);
            };
        }
    }
    dir_tree
}
