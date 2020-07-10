use std::fmt;

pub struct State {
    pub a: f32,
    pub b: f32
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"a|{}\nb|{}",self.a,self.b)
    }
}

