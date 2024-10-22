pub struct Node<S> {
    state: S
}

struct Primary {
    
}

struct Standby {
    
}

struct Promoting {
    
}

struct Demoting {
    
}

struct Offline {
    
}

struct Unhealhty {
    
}

struct CatchingUp {
    
}

struct Candidate {
    
}

impl Node<CatchingUp> {
    fn new() -> Self {
        let state = CatchingUp { };
        Node {
            state
        }
    }
}
