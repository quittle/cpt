import React, { useState } from "react";
import { BattleState, CardId, Character } from "./battle";
import { assetPath, Coordinate, isAdjacent } from "./utils";
import { move, takeAction } from "./state";
import { isCardEligible } from "./Card";

export function GameBoard(props: {
  battleState: BattleState;
  draggedCard: CardId | undefined;
}) {
  const battle = props.battleState.battle;
  const [selectedSquare, setSelectedSquare] = useState<Coordinate>();

  return (
    <table
      style={{
        background: "#cddc39",
        fontSize: "100px",
      }}
    >
      <tbody>
        {battle.board.grid.members.map((row, y) => (
          <tr key={y}>
            {row.map((col, x) => {
              let image: string | undefined;
              let character: Character | undefined;
              if (col?.Character !== undefined) {
                character = battle.characters[col.Character];
                if (character.image !== null) {
                  image = `url(${assetPath(character.image)})`;
                }
                if (character.health == 0) {
                  image = `url(${assetPath("skull.png")})`;
                }
              }
              const curLocation: Coordinate = { x, y };
              const isSelectedSquare =
                selectedSquare &&
                selectedSquare.x === x &&
                selectedSquare.y === y;
              const isPlayer =
                props.battleState.character_id === col?.Character;

              // Only ineligible if there is actively a card being dragged and that card isn't eligible.
              const isIneligible =
                props.draggedCard !== undefined &&
                (character?.health == 0 ||
                  !isCardEligible(isPlayer, props.draggedCard, battle));

              return (
                <td
                  key={x}
                  style={{
                    border: `1px solid ${isSelectedSquare ? "red" : "black"}`,
                    width: "1em",
                    height: "1em",
                    textAlign: "center",
                    backgroundImage: image,
                    backgroundSize: "contain",
                    opacity: isIneligible ? 0.5 : 1,
                  }}
                  onDragOver={(e) => {
                    if (props.draggedCard === undefined) {
                      return;
                    }

                    e.preventDefault();
                    e.dataTransfer.dropEffect = isIneligible ? "none" : "move";
                  }}
                  onDrop={async (_e) => {
                    if (
                      props.draggedCard === undefined ||
                      character === undefined
                    ) {
                      return;
                    }

                    await takeAction(props.draggedCard, character.id);
                  }}
                  onClick={async () => {
                    if (isPlayer) {
                      if (isSelectedSquare) {
                        setSelectedSquare(undefined);
                      } else {
                        setSelectedSquare(curLocation);
                      }
                    } else {
                      if (
                        selectedSquare !== undefined &&
                        isAdjacent(selectedSquare, curLocation)
                      ) {
                        const item =
                          battle.board.grid.members[selectedSquare.y][
                            selectedSquare.x
                          ];
                        if (item?.Character !== undefined) {
                          setSelectedSquare(undefined);
                          await move(item?.Character, curLocation);
                        }
                      }
                    }
                  }}
                  title={character?.name}
                ></td>
              );
            })}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
