pub type NodeIndex = usize;
pub type TransitionIndex = usize;

struct FiniteStateMachine {
    nameFSM: String,
    transitions: Vec<Transition>,
    nodes: Vec<Node>
}


struct Node {
    pub name: String,
    outputTransition: Vec<TransitionIndex>,
    inputTransition: Vec<TransitionIndex>
}

struct Transition {
    pub letter: char,
    outputNodes: NodeIndex,
    inputNodes: NodeIndex,
}

impl FiniteStateMachine {
    pub fn new(nameFSM: String) -> FiniteStateMachine {
        return FiniteStateMachine {
            nameFSM,
            transitions: Vec::new(),
            nodes: Vec::new()
        };
    }

    pub fn addNode(&mut self, name: String) -> NodeIndex {
        let index: usize = self.nodes.len();
        self.nodes.push(Node {
            name,
            outputTransition: Vec::new(),
            inputTransition: Vec::new(),
        });
        return index;
    }

    pub fn addTransition(&mut self, letter: char, outputNodes: NodeIndex, inputNodes: NodeIndex) -> TransitionIndex {
        let index: usize = self.transitions.len();
        self.transitions.push(Transition {
            letter,
            outputNodes,
            inputNodes,
        });
        return index;
    }


    pub fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool {
        if self.transitions[transitionTest].letter == c_char {
            return true;
        } else {
            return false;
        }
    }
}
/*
pub fn displayFsm(&self) {
    for node in self.node {

    }


}

pub fn display_petrinet(&self) {
    for edge in &self.edges {
        match edge.orientation {
            Orientation::Place => {
                println!("{place} -({w})-> {trans}", trans = &self.transitions[edge.transition].name, place = &self.places[edge.place].name, w = edge.weight);
            },
            Orientation::Transition => {
                println!("{trans} -({w})-> {place}", trans = &self.transitions[edge.transition].name, place = &self.places[edge.place].name, w = edge.weight);
            }
        }
    }
}*/

fn main() {
    println!("Hello, world!");
}
