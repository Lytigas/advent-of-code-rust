#[cfg(test)]
mod solutions {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday07 part1: {}", find_root(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday07 part2: {}", balance_tree(TEST_CASE));
    }
}

use std::collections::HashSet;

pub fn find_root(input: &str) -> &str {
    let mut all_programs: HashSet<&str> = HashSet::from(
        input
            .split('\n')
            .map(|x| x.split(' ').nth(0).unwrap())
            .collect(),
    );
    for program in input
        .split('\n')
        .flat_map(|x| x.split(' ').skip(3).map(|x| x.trim_matches(',')))
    {
        all_programs.remove(program);
    }
    // println!("{:?}", all_programs);
    *all_programs.iter().nth(0).unwrap()
}

use std::collections::HashMap;

pub fn balance_tree(input: &'static str) -> u32 {
    let mut programs: HashMap<&str, (u32, Vec<&str>)> = HashMap::new();
    for line in input.split('\n') {
        let mut tokens = line.split(' ');
        programs.insert(
            tokens.nth(0).unwrap(),
            (
                tokens
                    .nth(0)
                    .unwrap()
                    .trim_matches(|x| x == '(' || x == ')')
                    .parse::<u32>()
                    .unwrap(),
                tokens.skip(1).map(|x| x.trim_matches(',')).collect(),
            ),
        );
    }
    let root = find_root(input);
    let mut root_node = build_tree(&programs, root);
    calc_weight(&mut root_node);
    calc_balances(&mut root_node);
    solve(&root_node, None)
}

struct ProgramNode {
    #[allow(dead_code)] // `name` was used for debugging
    name: String,
    children: Vec<Box<ProgramNode>>,
    weight: u32,
    weight_above: u32,
    balanced_above: bool,
}

fn build_tree(map: &HashMap<&'static str, (u32, Vec<&str>)>, root: &str) -> Box<ProgramNode> {
    let program = map.get(root).unwrap();
    let this_node = ProgramNode {
        name: String::from(root),
        weight: program.0,
        weight_above: 0,
        children: program.1.iter().map(|x| build_tree(map, x)).collect(),
        balanced_above: false,
    };
    Box::new(this_node)
}

fn calc_weight(root: &mut ProgramNode) -> u32 {
    if root.children.is_empty() {
        root.weight_above = 0;
        return root.weight;
    };
    root.weight_above = root.children.iter_mut().map(|x| calc_weight(x)).sum();
    return root.weight + root.weight_above;
}

fn calc_balances(node: &mut ProgramNode) {
    node.balanced_above = true;
    if node.children.is_empty() {
        return;
    }
    let weights: Vec<u32> = node.children
        .iter()
        .map(|child| child.weight_above + child.weight)
        .collect();
    let mut last_weight = weights.get(0).unwrap();
    for index in 1..weights.len() {
        if last_weight != weights.get(index).unwrap() {
            node.balanced_above = false;
            break;
        }
        last_weight = weights.get(index).unwrap();
    }
    for child in node.children.iter_mut() {
        calc_balances(child);
    }
}

fn solve(node: &ProgramNode, weight_constraint: Option<u32>) -> u32 {
    // println!("===============\nAt Node {}", node.name);
    // for child in node.children.iter() {
    //     println!(
    //         "child {} has total weight {}",
    //         child.name,
    //         child.weight + child.weight_above
    //     )
    // }

    // if we don't have a weight constraint yet, calc one
    // this doesn't necessarily work in the general case, but works
    // for the given input in which the root node has more than two children
    let extracted_constraint;
    match weight_constraint {
        None => {
            let mut weights: Vec<u32> = node.children
                .iter()
                .map(|child| child.weight_above + child.weight)
                .collect();
            weights.sort();
            extracted_constraint =
                weights.get(weights.len() / 2).unwrap() * weights.len() as u32 + node.weight;
        }
        Some(constraint) => {
            extracted_constraint = constraint;
        }
    };
    // println!("the extracted constraint is {}", extracted_constraint);

    let child_goal_weight = (extracted_constraint - node.weight) / node.children.len() as u32;
    let unbalanced_children: Vec<&Box<ProgramNode>> = node.children
        .iter()
        .filter(|child| !child.balanced_above)
        .collect();
    if unbalanced_children.len() > 0 {
        // println!("Found an unbalanced child, recursing");
        return solve(unbalanced_children.get(0).unwrap(), Some(child_goal_weight));
    };
    // all our children are balanced, but we are not
    // change the child that will make us balanced
    for child in node.children.iter() {
        if child.weight + child.weight_above != child_goal_weight {
            return child_goal_weight - child.weight_above;
        }
    }
    unreachable!();
}
