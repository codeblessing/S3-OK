mod utils;
mod generator;
mod greedy;
mod pretty_print;
mod io;

use utils::{Case, Schedule};

// use utils::case::Case;

fn main() {
    let (_, optimal) = Case::generate(3, 5, 1, 100, 20);

    optimal.print();

//     case.save("./generated.test").unwrap();

//     let solution = greedy::schedule(&case);

//     println!("Greedy generated solution: {}", solution.max_time());
//     println!("Optimal solution: {}", optimal);

}
