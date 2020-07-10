mod util;
mod functional;
mod mutations;
mod types;

use types::State;

extern crate csv;
use std::fs;
use std::io::Write;

use std::env;

use csv::Writer;

// Macros??
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        functional::compose($head, compose!($($tail),+))
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_it = str::parse::<i32>(&args[1]).unwrap();

    // Defining the function composition that will be used
    // to alter the state over time
    let chain = compose!(
        mutations::grow,
        mutations::limit,
        mutations::mutate,
        mutations::relate); 

    // Initial state
    let mut state = State{a:1., b:1.};

    // Technical 
    let mut it = 1;
    let mut wtr = Writer::from_writer(vec![]);

    // Write the header
    wtr.write_record(&["a","b"]).unwrap();

    // Do the thing
    loop {

        // Calls the chain composition on state repeatedly
        state = chain(state);

        // Write state as a CSV row
        wtr.write_record(state.values().iter().map(|f|{f.to_string()})).unwrap();

        it += 1;
        if it > max_it {break};
    }

    // Serialization
    let v = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    let mut f = fs::File::create("out/dat.csv").unwrap();
    f.write_all(v.as_bytes()).unwrap();
}
