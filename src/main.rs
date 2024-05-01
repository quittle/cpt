use cpd::*;
use futures::executor::block_on;

fn main() {
    let mut battle = Battle::deserialize(
        include_str!("../data/sample-battle.json"),
        Box::<DefaultRandomProvider>::default(),
    )
    .unwrap();
    block_on(battle.run_to_completion());
    println!("Game over");
}
