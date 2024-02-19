use crate::*;
use async_trait::async_trait;
use std::io::{self, Write};

pub struct TerminalActor {
    pub character: Character,
}

impl TerminalActor {
    // fn writer(&self) -> impl Write {
    //     io::stdout()
    // }
}

#[async_trait]
impl Actor for TerminalActor {
    async fn act(&self, _battle: &Battle, character_id: CharacterId) -> ActionResult {
        print!("Action for {character_id}? ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok(ActionRequest {
            description: line.trim().into(),
        })
    }

    fn get_character_id(&self) -> CharacterId {
        self.character.id
    }
}
