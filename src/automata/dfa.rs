use std::collections::HashSet;
use crate::automata::automata::Parse;

pub struct DFA {
    pub adj_list: Vec<Vec<(usize, char)>>,
    pub start_state: usize,
    pub accept_states: HashSet<usize>,
    pub alphabet: Vec<char>
}

fn get_next_state<'a>(c: &'a char, transitions: &'a [(usize, char)]) -> &'a usize {
    for (state, char) in transitions {
        if c == char {
            return state
        }
    }

    unreachable!();
}

impl Parse for DFA {
    fn parse(&self, s: &str) -> bool {


        let mut state = self.start_state;

        let mut s = s.chars();
        while let Some(c) = s.next() {
            let transitions = self.adj_list.get(state).unwrap();
            state = *get_next_state(&c, &transitions);
        }
        self.accept_states.contains(&state)
    }
}