use super::dfa::DFA;
use std::collections::{HashMap, BTreeSet, HashSet};

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Char {
    Alphabet(char),
    EmptyCharacter
}

pub struct NFA {
    pub adj_list: Vec<Vec<(usize, Char)>>,
    pub start_state: usize,
    pub accept_states: HashSet<usize>,
    pub alphabet: Vec<char>
}

impl std::fmt::Debug for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Char::Alphabet(c) => write!(f, "{}", c),
            Char::EmptyCharacter => write!(f, "ε")
        }
    }
}

impl NFA {
    pub fn to_dfa(&self) -> DFA {
        let mut set_state_nodes: HashMap<BTreeSet<usize>, usize> = HashMap::new();
        let mut adj_list: Vec<Vec<(usize, char)>> = Vec::from([Vec::new()]);
        let mut accept_states: HashSet<usize> = HashSet::new();
        let mut stack: Vec<(BTreeSet<usize>, char)> = Vec::new();

        let mut start = BTreeSet::new();
        start.insert(self.start_state);
        let inital_set_state = epsilon_closure(&self.adj_list, &start);

        add_alphabet(&inital_set_state, &self.alphabet, &mut stack);

        while let Some((current_set_state, current_char)) = stack.pop() {

            let (current_state, _) = get_dfa_state(&current_set_state, &mut set_state_nodes);

            let moved = get_next_set_state(&self.adj_list, &current_set_state, &current_char);
            let new_set_state = epsilon_closure(&self.adj_list, &moved);

            let (new_state, is_new_state) = get_dfa_state(&new_set_state, &mut set_state_nodes);

            if is_new_state {
                adj_list.push(Vec::new());
                add_alphabet(&new_set_state, &self.alphabet, &mut stack);
            }

            adj_list[current_state].push((new_state, current_char));
        }


        let (start_state, _) = get_dfa_state(&inital_set_state, &mut set_state_nodes);

        for (set_states, mapped_state) in set_state_nodes {
            for state in set_states {
                if self.accept_states.contains(&state) {
                    accept_states.insert(mapped_state);
                }
            }
        }

        DFA {
            adj_list: adj_list,
            start_state: start_state,
            accept_states: accept_states,
            alphabet: self.alphabet.clone()
        }
    }
}

fn get_next_set_state(
    adj_list: &Vec<Vec<(usize, Char)>>, current_state: &BTreeSet<usize>, char: &char
) -> BTreeSet<usize> {


    let mut set: BTreeSet<usize> = BTreeSet::new();
    for state in current_state {
        let mut visited = HashSet::new();
        set.append(&mut get_edge_verticies(adj_list, *state, char, &mut visited))
    }

    set
}

fn get_edge_verticies(
    adj_list: &Vec<Vec<(usize, Char)>>,
    current_state: usize,
    char: &char,
    visited: &mut HashSet<usize>
) -> BTreeSet<usize> {

    if visited.contains(&current_state) {
        return BTreeSet::new();
    }

    visited.insert(current_state);

    let mut set = BTreeSet::new();

    if let Some(edges) = adj_list.get(current_state) {
        for (vertex, c) in edges {
            match c {
                Char::Alphabet(c2) => {
                    if c2 == char {
                        set.insert(*vertex);
                    }
                }
                Char::EmptyCharacter => {
                    set.append(&mut get_edge_verticies(
                        adj_list,
                        *vertex,
                        char,
                        visited
                    ));
                }
            }
        }
    }

    set
}

fn epsilon_closure(
    adj_list: &Vec<Vec<(usize, Char)>>,
    start_states: &BTreeSet<usize>
) -> BTreeSet<usize> {
    let mut closure = start_states.clone();
    let mut stack: Vec<usize> = start_states.iter().cloned().collect();

    while let Some(state) = stack.pop() {
        if let Some(edges) = adj_list.get(state) {
            for (next, c) in edges {
                if matches!(c, Char::EmptyCharacter) && !closure.contains(next) {
                    closure.insert(*next);
                    stack.push(*next);
                }
            }
        }
    }

    closure
}

fn add_alphabet(
    node: &BTreeSet<usize>, alphabet: &Vec<char>, stack: &mut Vec<(BTreeSet<usize>, char)>
) {

    let mut alphabet = alphabet.iter();
    while let Some(c) = alphabet.next() {
        stack.push((node.clone(), *c))
    }
}

fn get_dfa_state(
    set_of_states: &BTreeSet<usize>, set_state_nodes: &mut HashMap<BTreeSet<usize>, usize>
) -> (usize, bool) {

    match set_state_nodes.get(set_of_states) {
        Some(node) => {
            // node exists
            (*node, false)
        },
        None => {
            // create the new node
            set_state_nodes.insert(set_of_states.clone(), set_state_nodes.len());
            (set_state_nodes.len() - 1, true)
        }
    }
}