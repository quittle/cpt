use std::{
    io::{stderr, stdout},
    process::ExitCode,
};

use cpd::*;
use futures::executor::block_on;
use termion::raw::IntoRawMode;

fn main() -> Result<(), ExitCode> {
    let mut battle = Battle::deserialize(
        include_str!("../data/sample-battle.json"),
        Box::<DefaultRandomProvider>::default(),
    )
    .unwrap();
    let (_out, _err) = (
        stdout().into_raw_mode().unwrap(),
        stderr().into_raw_mode().unwrap(),
    );
    block_on(battle.run_to_completion())?;
    println!("Game over");

    Ok(())
}
