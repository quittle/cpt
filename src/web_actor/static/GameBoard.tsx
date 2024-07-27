import React from "react";
import { Battle } from "./battle";
import { assetPath } from "./utils";

export function GameBoard(props: { battle: Battle }) {
    return (
        <table
            style={{
                background: "#cddc39",
                fontSize: "100px",
            }}
        >
            <tbody>
                {props.battle.board.grid.members.map((row, y) => (
                    <tr key={y}>
                        {row.map((col, x) => {
                            let image: string | undefined;
                            let title: string | undefined;
                            if (col?.Character !== undefined) {
                                const character =
                                    props.battle.characters[col.Character];
                                if (character.image !== null) {
                                    image = `url(${assetPath(character.image)})`;
                                }
                                title = character.name;
                            }
                            return (
                                <td
                                    key={x}
                                    style={{
                                        border: "1px solid black",
                                        width: "1em",
                                        height: "1em",
                                        textAlign: "center",
                                        backgroundImage: image,
                                        backgroundSize: "contain",
                                    }}
                                    title={title}
                                ></td>
                            );
                        })}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
