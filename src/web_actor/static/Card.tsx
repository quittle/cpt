import React from "react";
import { Card } from "./battle";

export default function Card(props: {
    card: Card;
    onDragStart: () => void;
    onDragEnd: () => void;
    onClick: () => void;
    hasDefaultAction: boolean;
}) {
    return (
        <button
            draggable={true}
            onDragStart={(e) => {
                e.dataTransfer.setData("text/plain", String(props.card.id));
                props.onDragStart();
            }}
            onClick={props.onClick}
            onDragEnd={props.onDragEnd}
            style={{
                padding: 0,
                borderWidth: "0.3em",
                width: "15em",
                cursor: props.hasDefaultAction ? "pointer" : "grab",
            }}
        >
            <b
                style={{
                    background: "rgba(10, 10, 10, 0.3)",
                    display: "block",
                    padding: "0.3em",
                    borderBottom: "0.1em solid black",
                }}
            >
                {props.card.name}
            </b>
            <div style={{ padding: "0.5em" }}>
                <p>{props.card.description}</p>
                <p>
                    <i>{props.card.flavor}</i>
                </p>
            </div>
        </button>
    );
}
