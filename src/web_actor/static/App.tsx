import React, { useEffect, useState } from "react";
import { Battle, BattleState } from "./battle";
import * as messages from "./messages.js";

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
        <div>
            <h2>Characters</h2>
            {Object.values(battle.characters).map((character) => {
                return (
                    <div>
                        {character.name} - <b>{character.health}</b>
                    </div>
                );
            })}
            <ul>
                {battle.characters[characterId].hand.map((cardId) => {
                    const card = battle.cards[cardId];
                    return (
                        <li key={cardId}>
                            <button
                                onClick={async () => {
                                    await fetch("/act", {
                                        method: "POST",
                                        headers: {
                                            "Content-Type": "application/json",
                                        },
                                        body: JSON.stringify({
                                            card_id: cardId,
                                            target_id: battle.characters[1].id,
                                        }),
                                    });
                                }}
                            >
                                <b>{card.name}</b>
                                <p>{card.description}</p>
                                <p>
                                    <i>{card.flavor}</i>
                                </p>
                            </button>
                        </li>
                    );
                })}
            </ul>
        </div>
    );

    return <pre>{JSON.stringify(battle)}</pre>;
}
