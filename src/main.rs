use cpd::*;
use futures::executor::block_on;

fn main() {
    let a = Box::new(TerminalActor {
        character: Character {
            id: CharacterId::new(0),
            name: "Person A".into(),
            race: CharacterRace::Human,
            base_attack: Attack::new(1),
            health: Health::new(20),
        },
    });
    let b = Box::new(DumbActor {
        character: Character {
            id: CharacterId::new(1),
            name: "Person B".into(),
            race: CharacterRace::Human,
            base_attack: Attack::new(2),
            health: Health::new(5),
        },
    });
    let mut battle = Battle {
        actors: vec![(TeamId::new(0), a), (TeamId::new(1), b)],
        teams: vec![
            Team {
                name: "Player".into(),
                id: TeamId::new(0),
            },
            Team {
                name: "Computer".into(),
                id: TeamId::new(1),
            },
        ],
        history: vec![],
        random_provider: Box::new(DefaultRandomProvider::default()),
    };
    block_on(battle.run_to_completion());
    println!("Game over");
}
