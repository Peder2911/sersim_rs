
use rand::Rng;
use crate::types::State;
use crate::util::InRange;

pub fn grow(s: State)->State{
    State{
        a: s.a + 1.,
        b: s.b + 1. 
    }
}

pub fn limit(s: State)->State{
    let new_a = match s.a {
        x if x.in_range(0.,10.) => s.a,
        _ => 10.0
    };
    State{
        a: new_a,
        b: s.b
    }
}

pub fn mutate(s:State)->State{
    let r = rand::thread_rng().gen::<f32>();
    let new_a = s.a * (r*0.01);
    State {
        a: new_a,
        b: s.b
    }
}

pub fn relate(s:State)->State{
    State {
        a: s.a * s.b,
        b: s.b
    }
}

