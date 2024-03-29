use std::os::raw::c_char;

pub type NodeIndex = usize;
pub type TransitionIndex = usize;

struct NamedElement{
    name: String
}

struct FiniteStateMachine {
    nameFSM: String,
    transitions: Vec<Transition>,
    nodes: Vec<Node>
}

struct Node {
    pub name: i32,
    outputTransitions: Vec<TransitionIndex>,
    inputTransitions: Vec<TransitionIndex>
}

struct Transition {
    pub letter: char,
    outputNode: NodeIndex,
    inputNode: NodeIndex,
}

impl FiniteStateMachine {
    pub fn new(nameFSM: &str) -> FiniteStateMachine {
        return FiniteStateMachine {
            nameFSM: nameFSM.into(),
            transitions: Vec::new(),
            nodes: Vec::new()
        };
    }

    pub fn addNode(&mut self, name: i32) -> NodeIndex {
        let index: usize = self.nodes.len();
        self.nodes.push(Node {
            name,
            outputTransitions: Vec::new(),
            inputTransitions: Vec::new(),
        });
        return index;
    }

    pub fn addTransition(&mut self, letter: char, inputNode: NodeIndex, outputNode: NodeIndex) -> TransitionIndex {
        let index: usize = self.transitions.len();
        let new_trans = Transition {
            letter,
            outputNode,
            inputNode,
        };
        self.transitions.push(new_trans);

        self.nodes[inputNode].outputTransitions.push(index);
        self.nodes[outputNode].inputTransitions.push(index);
        return index;
    }


    pub fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool {
        return self.transitions[transitionTest].letter == c_char;
    }

    pub fn displayTransition(&self) {
        println!("\n");
        for transitionNumber in &self.transitions {
            println!("{} {} {} {} {}", self.nodes[transitionNumber.inputNode].name, "-", transitionNumber.letter, "->", self.nodes[transitionNumber.outputNode].name);
        }
    }

    pub fn displayFSM(&self) {
        println!("{}", "\nDisplay Finite state machine : \n");
        print!("State : ");

        for nodeNumber in &self.nodes {
            print!("{} {}", nodeNumber.name, " ");
        }

        self.displayTransition()
    }


    pub fn process_fsm(&mut self, nodeIndex: NodeIndex, word: &str) -> Vec<(usize, usize)> {
        let mut vec_edge = Vec::new();

        let octets = word.as_bytes();

        let mut iterator_word = 0;
        let mut iterator_trans = 0;
        let mut state = 0;

        for (iterator_word, &char) in octets.iter().enumerate() {
            let char_trans = self.transitions[self.nodes[nodeIndex].outputTransitions[iterator_trans]].letter;

            while iterator_trans < self.nodes[nodeIndex].outputTransitions.len() && char_trans != char as char {
                iterator_trans += 1;
            }
            if self.transitions[iterator_trans].outputNode < self.nodes.len() {
                vec_edge.push((state,self.transitions[iterator_trans].outputNode));
                state = iterator_trans;
                iterator_trans = 0;
            }
        }

        if vec_edge.len() < word.len(){
            println!("Pas de chemin complet trouver pour ce mot.")
        } else {
            println!("Chemin trouvé pour ce mot.")
        }
        return vec_edge;

    }

    fn print_vec(mut vec : Vec<Vec<&str>>) {
        let state_nb = vec.len();

        for i in 0..state_nb {
            for j in 0..state_nb {
                print!("{}", vec[i][j]);
            }
            println!();
        }
        println!();
    }


}
/*
       for edge_index in &self.transitions[trans_index].input_edges {
            let place_index = &self.edges[*edge_index].place;
            self.places[*place_index].tokens -= &self.edges[*edge_index].weight;
        }
        for edge_index in &self.transitions[trans_index].output_edges {
            let place_index = &self.edges[*edge_index].place;
            self.places[*place_index].tokens += &self.edges[*edge_index].weight;
        }
    }*/



fn main() {

    let mut test = FiniteStateMachine::new("Test");
    let name = 0;
    let name2 = 1;
    let name3 = 2;

    test.addNode(name);
    test.addNode(name2);
    test.addNode(name3);

    test.addTransition('a',0,1);
    test.addTransition('b',0,2);
    test.addTransition('c',1,2);
    test.addTransition('d',2,2);

    test.displayFSM();

    let word1 = "acd";
    let mut result = test.process_fsm(0,word1);
    let lenght = result.len();
    print!("Taille du chemin pour ce mot.");
    print!("{}",lenght);
    dbg!(result);
}