import React, { useEffect, useState } from "react";
import { Battle, BattleState } from "./battle";
import * as messages from "./messages.js";
import Card from "./Card.js";
import Character from "./Character.js";
import BattleHistory from "./BattleHistory.js";

messages.init(async () => {});

export default function App() {
    const [battleState, setBattleState] = useState<BattleState>();

    useEffect(() => {
        // Throwaway
        fetch("/info");

        const onBattleState = (e) => {
            setBattleState(JSON.parse(e.data));
        };

        messages.addEventListener("battle_state", onBattleState);
        return () => {
            messages.removeEventListener("battle_state", onBattleState);
        };
    }, [setBattleState]);

    if (!battleState) {
        return <div>Loading...</div>;
    }

    const { character_id: characterId, battle } = battleState;

    return (
        <div style={{ display: "flex", maxWidth: "1500px" }}>
            <div style={{ flexGrow: 5 }}>
                <h2>Characters</h2>
                <div
                    style={{ display: "flex", justifyContent: "space-around" }}
                >
                    {Object.values(battle.characters).map((character) => (
                        <Character key={character.id} character={character} />
                    ))}
                </div>
                <ul
                    style={{
                        listStyle: "none",
                        display: "flex",
                        flexDirection: "column",
                        gap: "1em",
                    }}
                >
                    {battle.characters[characterId].hand.map((cardId) => {
                        const card = battle.cards[cardId];
                        return (
                            <li key={cardId}>
                                <Card card={card} />
                            </li>
                        );
                    })}
                </ul>
            </div>
            <div style={{ flexGrow: 2 }}>
                <BattleHistory history={battle.history} />
            </div>
        </div>
    );
}
