use cpd::*;
use futures::executor::block_on;

fn main() {
    let a = Box::new(TerminalActor {
        character: Character {
            id: CharacterId::new(0),
            name: "Person A".into(),
            race: CharacterRace::Human,
            base_attack: 1,
            health: 2,
        },
    });
    let b = Box::new(DumbActor {
        character: Character {
            id: CharacterId::new(1),
            name: "Person B".into(),
            race: CharacterRace::Human,
            base_attack: 2,
            health: 5,
        },
    });
    let mut battle = Battle {
        actors: vec![(0, a), (1, b)],
        teams: vec![
            Team {
                name: "Player".into(),
                id: 0,
            },
            Team {
                name: "Computer".into(),
                id: 1,
            },
        ],
    };
    loop {
        block_on(battle.advance());
    }
}
