import React from "react";
import { ActionTarget, Battle, CardId, Character, CharacterId } from "./battle";
import { getCardTarget } from "./utils";

function isCardEligible(
    isPlayer: boolean,
    cardId: CardId,
    battle: Battle
): boolean {
    const card = battle.cards[cardId];
    const target = getCardTarget(card);
    switch (target) {
        case ActionTarget.Me:
            return isPlayer;
        case ActionTarget.Others:
            return !isPlayer;
        case ActionTarget.Any:
            return true;
    }
}

export default function Character(props: {
    isPlayer: boolean;
    characterId: CharacterId;
    draggedCard: CardId | undefined;
    battle: Battle;
}) {
    const { isPlayer, characterId, draggedCard, battle } = props;
    const character = battle.characters[characterId];

    // Only ineligible if there is actively a card being dragged and that card isn't eligible.
    const isIneligible =
        draggedCard !== undefined &&
        !isCardEligible(isPlayer, draggedCard, battle);

    return (
        <div
            style={{
                background: "#ededed",
                padding: "1em",
                opacity: isIneligible ? 0.5 : 1,
            }}
            onDragOver={(e) => {
                if (draggedCard === undefined) {
                    return;
                }

                e.preventDefault();
                e.dataTransfer.dropEffect = isIneligible ? "none" : "move";
            }}
            onDrop={async (e) => {
                await fetch("/act", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        card_id: draggedCard,
                        target_id: characterId,
                    }),
                });
            }}
        >
            <h3>{character.name}</h3>
            {isPlayer
                ? `Remaining actions: ${"🔵".repeat(
                      character.remaining_actions
                  )}`
                : null}
            <div>
                Health: <b>{character.health}</b>
            </div>
        </div>
    );
}
