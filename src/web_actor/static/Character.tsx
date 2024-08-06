import React from "react";
import { Battle, CardId, Character, CharacterId } from "./battle";
import { assetPath } from "./utils";
import { pass, takeAction } from "./state";
import { isCardEligible } from "./Card";

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
    (character.health == 0 || !isCardEligible(isPlayer, draggedCard, battle));

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
      onDrop={async (_e) => {
        if (draggedCard === undefined) {
          return;
        }

        await takeAction(draggedCard, characterId);
      }}
    >
      {character.image ? (
        <img
          src={assetPath(character.image)}
          style={{
            width: isPlayer ? "100px" : "100%",
          }}
        />
      ) : null}
      {isPlayer ? (
        <button
          style={{
            fontSize: "2em",
            padding: "1em",
            marginInlineStart: "1em",
            verticalAlign: "top",
          }}
          onClick={async () => {
            await pass();
          }}
        >
          End Turn
        </button>
      ) : null}
      <h3>{character.name}</h3>
      {isPlayer
        ? `Remaining actions: ${"🔵".repeat(character.remaining_actions)}`
        : null}
      <div>
        Movement: <b>{character.movement}</b>
      </div>
      <div>
        Health: <b>{character.health}</b>
      </div>
    </div>
  );
}
