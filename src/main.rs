use std::os::raw::c_char;

pub type NodeIndex = usize;
pub type TransitionIndex = usize;


struct FiniteStateMachine {
    //nameFSM: String,
    transitions: Vec<Transition>,
    nodes: Vec<Node>
}

struct Node {
    pub name: i32,
    outputTransition: Vec<TransitionIndex>,
    inputTransition: Vec<TransitionIndex>
}

struct Transition {
    pub letter: char,
    outputNodes: NodeIndex,
    inputNodes: NodeIndex,
}

impl FiniteStateMachine {
    pub fn new() -> FiniteStateMachine {
        return FiniteStateMachine {
           // nameFSM,
            transitions: Vec::new(),
            nodes: Vec::new()
        };
    }

    pub fn addNode(&mut self, name: i32) -> NodeIndex {
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

    pub fn displayNode(&self,nodeI: NodeIndex) {
        for nodeNumber in &self.nodes{
            print!("{} {}",nodeNumber.name, " ");
        }
    }

    pub fn displayTransition(&self,nodeI: NodeIndex) {
        print!("{} {}", self.nodes[nodeI].name, " ");

        for nodeNumber in &self.nodes{
            print!("{} {}",nodeNumber.name, " ");
        }
    }

    pub fn displayFSM(&self) {
        println!("{}","Finite state machine display : ");

        for nodeNumber in &self.nodes{
            print!("{} {}",nodeNumber.name, " ");
        }
    }
}
/*
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

    let mut test = FiniteStateMachine::new();
    let name = 1;
    let name2 = 2;
    let name3 = 3;

    test.addNode(name);
    test.addNode(name2);
    test.addNode(name3);

    test.addTransition('a',0,1);
    test.addTransition('b',0,2);
    test.addTransition('c',1,2);
    test.addTransition('d',2,2);

    test.displayFSM();

}
