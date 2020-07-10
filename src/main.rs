use std::fmt;
use rand::Rng;

struct State {
    a: f32,
    b: f32
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"a|{}\nb|{}",self.a,self.b)
    }
}

trait InRange {
    fn in_range(self,begin:Self,end:Self)->bool;
}
impl InRange for f32 {
    fn in_range(self,begin:f32, end:f32)->bool{
        self >=begin && self < end
    }
}


macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn grow(s: State)->State{
    State{
        a: s.a + 1.,
        b: s.b + 1. 
    }
}

fn limit(s: State)->State{
    let new_a = match s.a {
        x if x.in_range(0.,10.) => s.a,
        _ => 10.0
    };
    State{
        a: new_a,
        b: s.b
    }
}

fn mutate(s:State)->State{
    let r = rand::thread_rng().gen::<f32>();
    let new_a = s.a * (r*0.01);
    State {
        a: new_a,
        b: s.b
    }
}

fn relate(s:State)->State{
    State {
        a: s.a * s.b,
        b: s.b
    }
}

fn main() {
    let chain = compose!(grow,limit,mutate,relate); 
    let mut state = State{a:1., b:1.};
    let mut it = 1;
    loop {
        state = chain(state);
        it += 1;
        if it > 100 {break};
    }
    println!("{}\n",state);
}

