use cpd::*;
use std::{env, fs, ops::Deref, path::PathBuf, process::ExitCode};
#[cfg(feature = "terminal_ui")]
use {std::io, termion::raw::IntoRawMode};

#[actix_web::main]
async fn main() -> Result<(), ExitCode> {
    let args: Vec<String> = env::args().collect();
    let file = args
        .get(1)
        .map(Deref::deref)
        .unwrap_or("sample-battle.json");
    let file_path = format!("data/{file}");

    let battle_file = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Unable to open file: {file_path}"));
    let mut battle = Battle::deserialize(
        &battle_file,
        Some(PathBuf::from("data")),
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
