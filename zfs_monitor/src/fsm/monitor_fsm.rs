pub struct Monitor<S> {
    state: S
}

struct Initialize {
    
}

struct Standby {
    
}

struct Candidate {
    
}

struct Primary {
    
}

struct Offline {
    
}

impl Monitor<Initialize> {
    fn new() -> Self {
        let state = Initialize { };
        Monitor {
            state
        }        
    }
}

impl From<Monitor<Initialize>> for Monitor<Standby> {
    fn from(val: Monitor<Initialize>) -> Monitor<Standby> {
        let state = Standby { };
        Monitor {
            state
        }
    }
}

impl From<Monitor<Standby>> for Monitor<Candidate> {
    fn from(val: Monitor<Standby>) -> Monitor<Candidate> {
        let state = Candidate { };
        Monitor {
            state
        }
    }
}

impl From<Monitor<Candidate>> for Monitor<Standby> {
    fn from(val: Monitor<Candidate>) -> Monitor<Standby> {
        let state = Standby { };
        Monitor {
            state
        }
    }
}

impl From<Monitor<Candidate>> for Monitor<Primary> {
    fn from(val: Monitor<Candidate>) -> Monitor<Primary> {
        let state = Primary { };
        Monitor {
            state
        }
    }
}

impl From<Monitor<Primary>> for Monitor<Standby> {
    fn from(val: Monitor<Primary>) -> Monitor<Standby> {
        let state = Standby { };
        Monitor {
            state
        }
    }
}

impl From<Monitor<Primary>> for Monitor<Offline> {
    fn from(val: Monitor<Primary>) -> Monitor<Offline> {
        let state = Offline { };
        Monitor {
            state
        }
    }
}