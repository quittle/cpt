use cpd::*;
use std::process::ExitCode;
#[cfg(feature = "terminal_ui")]
use {std::io, termion::raw::IntoRawMode};

#[actix_web::main]
async fn main() -> Result<(), ExitCode> {
    let mut battle = Battle::deserialize(
        include_str!("../data/sample-battle.json"),
        Box::<DefaultRandomProvider>::default(),
    )
    .await
    .unwrap();
    #[cfg(feature = "terminal_ui")]
    let (_out, _err) = (
        io::stdout().into_raw_mode().unwrap(),
        io::stderr().into_raw_mode().unwrap(),
    );
    battle.run_to_completion().await?;
    println!("Game over");

    Ok(())
}
