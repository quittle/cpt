import React, { useState } from "react";
import { BattleState, Character } from "./battle";
import { assetPath, Coordinate, isAdjacent } from "./utils";
import { move } from "./state";

export function GameBoard(props: { battleState: BattleState }) {
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
              }
              const curLocation: Coordinate = { x, y };
              const isSelectedSquare =
                selectedSquare &&
                selectedSquare.x === x &&
                selectedSquare.y === y;
              const isPlayer =
                props.battleState.character_id === col?.Character;
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
