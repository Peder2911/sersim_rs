struct State {
    a: i32,
    b: i32
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
        a: s.a + 1,
        b: s.b + 1 
    }
}

fn limit(s: State)->State{
    let new_a = match s.a {
        1..=10 => s.a,
        _ => 10
    };
    State{
        a: new_a,
        b: s.b
    }
}

fn main() {
    let chain = compose!(grow,limit); 
    let mut state = State{a:1, b:1};
    let mut it = 1;
    loop {
        state = chain(state);
        println!("{:?}| {:?}",it,state.a);
        println!("{:?}| {:?}",it,state.b);
        it += 1;
        if it > 100 {break};
    }

    println!("Final| {:?}",state.a);
    println!("Final| {:?}",state.b);
}

