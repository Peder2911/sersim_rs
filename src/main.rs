mod util;
mod functional;
mod mutations;
mod types;

use types::State;

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        functional::compose($head, compose!($($tail),+))
    };
}

fn main() {
    let max_it = 100;

    let chain = compose!(
        mutations::grow,
        mutations::limit,
        mutations::mutate,
        mutations::relate); 

    let mut state = State{a:1., b:1.};
    let mut it = 1;
    loop {
        state = chain(state);
        it += 1;
        if it > max_it {break};
    }
    println!("{}\n",state);
}

