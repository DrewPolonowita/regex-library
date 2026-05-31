use crate::automata::nfa::{NFA, Char};
use std::collections::HashSet;

//const ASCII: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

const ASCII: [char; 2] = ['0', '1'];

pub struct NfaBuilder {
    nfa: NFA
}


impl NfaBuilder {
    pub fn new(chars: &[char]) -> NfaBuilder {
        let mut transitions: Vec<(usize, Char)> = Vec::new();

        for char in chars {
            if ASCII.contains(char) {
                transitions.push((2usize, Char::Alphabet(*char)))
            }
        }

        NfaBuilder {
            nfa: NFA {
                adj_list: Vec::from([
                    Vec::from([(1usize, Char::EmptyCharacter)]),
                    Vec::from(transitions),
                    Vec::from([(3usize, Char::EmptyCharacter)]),
                    Vec::from([])
                ]),
                start_state: 0,
                accept_states: HashSet::from([3]),
                alphabet: Vec::from(ASCII)
            }
        }

    }

    pub fn build(self) -> NFA {
        self.nfa
    }

    pub fn star(self) -> NfaBuilder {
        let mut new_adj_list = Vec::from([Vec::from([(1usize, Char::EmptyCharacter)])]);

        // add all edges and set new according pointers
        for transitions in self.nfa.adj_list {
            let mut next_vec = Vec::new();

            for (next_q, transition) in transitions {
                next_vec.push((next_q + 1, transition.clone()));
            }

            new_adj_list.push(next_vec);
        }

        new_adj_list.push(Vec::new());

        // create the epsilon transitions that create the star
        let n = new_adj_list.len();
        new_adj_list[1].push((n - 2, Char::EmptyCharacter));
        new_adj_list[n-2].push((1, Char::EmptyCharacter));
        new_adj_list[n-2].push((n-1, Char::EmptyCharacter));

        NfaBuilder {
            nfa: NFA {
                adj_list: new_adj_list,
                start_state: 0,
                accept_states: HashSet::from([n-1]),
                alphabet: self.nfa.alphabet
            }
        }
    }

    pub fn add(self, other: NfaBuilder) -> NfaBuilder {
        let mut new_adj_list = self.nfa.adj_list;
        let n = new_adj_list.len();

        for transitions in other.nfa.adj_list {
            let mut next_vec = Vec::new();

            for (next_q, transition) in transitions {
                next_vec.push((next_q + n, transition))
            }

            new_adj_list.push(next_vec);
        }

        new_adj_list[n-1].push((n, Char::EmptyCharacter));

        let n = new_adj_list.len();

        NfaBuilder {
            nfa: NFA {
                adj_list: new_adj_list,
                start_state: 0,
                accept_states: HashSet::from([n - 1]),
                alphabet: self.nfa.alphabet
            }
        }
    }

    pub fn or(self, other: NfaBuilder) -> NfaBuilder {
        let mut new_adj_list = Vec::from([Vec::new()]); // (1usize, Char::EmptyCharacter)

        // add all edges and set new according pointers for self
        for transitions in self.nfa.adj_list {
            let mut next_vec = Vec::new();

            for (next_q, transition) in transitions {
                next_vec.push((next_q + 1, transition.clone()));
            }

            new_adj_list.push(next_vec);
        }

        let n = new_adj_list.len();

        // add all edges and set new according pointers for other
        for transitions in other.nfa.adj_list {
            let mut next_vec = Vec::new();

            for (next_q, transition) in transitions {
                next_vec.push((next_q + n, transition))
            }

            new_adj_list.push(next_vec);
        }

        new_adj_list.push(Vec::new());
        let m = new_adj_list.len();

        new_adj_list[0].push((1, Char::EmptyCharacter));
        new_adj_list[0].push((n, Char::EmptyCharacter));

        new_adj_list[n-1].push((m-1, Char::EmptyCharacter));
        new_adj_list[m-2].push((m-1, Char::EmptyCharacter));

        NfaBuilder {
            nfa: NFA {
                adj_list: new_adj_list,
                start_state: 0,
                accept_states: HashSet::from([m - 1]),
                alphabet: self.nfa.alphabet
            }
        }
    }
}