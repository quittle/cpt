import React from "react";
import { Character } from "./battle";

export default function Character(props: { character: Character }) {
    return (
        <div
            style={{ background: "#ededed", padding: "1em" }}
            onDragOver={(e) => {
                e.preventDefault();
            }}
            onDrop={async (e) => {
                const cardId = parseInt(
                    e.dataTransfer.getData("text/plain"),
                    10
                );

                await fetch("/act", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        card_id: cardId,
                        target_id: props.character.id,
                    }),
                });
            }}
        >
            <h3>{props.character.name}</h3>
            <div>
                Health: <b>{props.character.health}</b>
            </div>
        </div>
    );
}
