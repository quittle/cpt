use cpd::*;
use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    let a = Box::new(DumbActor {
        character: Character {
            id: 0,
            name: "Person A".into(),
            race: CharacterRace::Human,
            base_attack: 1,
        },
    });
    let b = Box::new(DumbActor {
        character: Character {
            id: 1,
            name: "Person B".into(),
            race: CharacterRace::Human,
            base_attack: 2,
        },
    });
    let mut battle = Battle {
        actors: vec![(0, a), (1, b)],
        teams: vec![Team { id: 0 }, Team { id: 1 }],
    };
    block_on(battle.advance());
}
