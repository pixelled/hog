use hog::{Game, PureStrategy, State, ImpureStrategy};

fn main() {
    // let mut final_strategy = PureStrategy::import("final.strat").unwrap();
    let mut new_strategy = PureStrategy::import("learn.strat").unwrap();
    let ultimate_strategy = ImpureStrategy::build();
//    print!("{}", Game::new().win_rate(&new_strategy, &final_strategy));
    new_strategy.improve(&ultimate_strategy);
    new_strategy.export("learn.strat");
}
