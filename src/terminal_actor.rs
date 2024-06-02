use std::rc::Rc;

use crate::*;
use async_trait::async_trait;

pub struct TerminalActor {
    pub character_id: CharacterId,
}

impl TerminalActor {
    fn get_valid_target(
        &self,
        blocks: &mut Vec<TerminalBlock>,
        mut menu: BattleMenu,
        battle: &Battle,
    ) -> Result<Option<(CharacterId, CardId)>, ActionError> {
        blocks.push(TerminalBlock::default());
        blocks.push(TerminalBlock {
            prefix: TerminalSpan {
                contents: "".into(),
                color: Some(Box::new(termion::color::Yellow)),
            },
            contents: "".into(),
            ..Default::default()
        });

        let action = menu.wait_for_selection(blocks, battle)?;
        match action {
            BattleMenuOutput::Pass => Ok(None),
            BattleMenuOutput::Card { target, card } => Ok(Some((target, card))),
        }
    }

    fn get_battle_status(&self, battle: &Battle) -> Vec<TerminalBlock> {
        let mut blocks = vec![];

        for entry in &battle.history {
            blocks.push(TerminalBlock::new(
                entry
                    .iter()
                    .map(battle_history_to_terminal_string)
                    .collect::<Vec<String>>()
                    .join(" "),
            ));
        }

        if !battle.history.is_empty() {
            blocks.push(TerminalBlock::default());
        }

        for team in &battle.teams {
            blocks.push(TerminalBlock::new(format!("Team: {}", team.name)));

            for (team_id, actor) in &battle.actors {
                if team_id != &team.id {
                    continue;
                }
                let character = battle.get_character(actor.as_ref());
                blocks.push(TerminalBlock::new(if character.is_dead() {
                    format!("- {} ({}). Dead 💀", character.name, character.id)
                } else {
                    format!(
                        "- {} ({}). Health: {}",
                        character.name, character.id, character.health,
                    )
                }));
            }
        }
        blocks
    }

    fn get_enemies(&self, battle: &Battle) -> Vec<CharacterId> {
        let mut enemies = vec![];
        let my_team = battle.get_team_for_actor(self);
        for team in &battle.teams {
            for (team_id, actor) in &battle.actors {
                if team_id != &team.id {
                    continue;
                }
                let character = battle.get_character(actor.as_ref());
                if Some(*team_id) != my_team && !character.is_dead() {
                    enemies.push(character.id);
                }
            }
        }
        enemies
    }
}

fn battle_history_to_terminal_string(battle_history: &BattleHistory) -> String {
    match battle_history {
        BattleHistory::Text(text) => text.clone(),
        BattleHistory::Id(text) => {
            format!("{}{}{}", termion::style::Bold, text, termion::style::Reset)
        }
        BattleHistory::Attack(text) => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Yellow),
            text,
            termion::color::Fg(termion::color::Reset),
        ),
        BattleHistory::Damage(text) => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Red),
            text,
            termion::color::Fg(termion::color::Reset),
        ),
    }
}

#[async_trait]
impl Actor for TerminalActor {
    fn get_character_id(&self) -> &CharacterId {
        &self.character_id
    }

    async fn act(&self, battle: &Battle) -> ActionResult {
        let mut blocks = self.get_battle_status(battle);
        blocks.push(TerminalBlock::default());

        let menu = BattleMenu::new(vec![
            Rc::new(ActionsMenu {
                me: self.character_id,
                cards: battle.get_character(self).cards.clone(),
                targets: self.get_enemies(battle),
            }),
            Rc::new(PassMenuItem {}),
        ]);

        if let Some((target_id, card_id)) = self.get_valid_target(&mut blocks, menu, battle)? {
            Ok(Action::Act(card_id, target_id))
        } else {
            Ok(Action::Pass)
        }
    }

    async fn on_game_over(&self, battle: &Battle) {
        let mut blocks = self.get_battle_status(battle);

        blocks.push(TerminalBlock::new(if self.get_enemies(battle).is_empty() {
            "You win!"
        } else {
            "You lose!"
        }));
        blocks.push(TerminalBlock::default());

        let _ = TerminalUi::draw(&blocks);
    }
}
