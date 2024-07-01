import React from "react";
import { BattleHistoryEntry, TypedText } from "./battle";

function convert(typedText: TypedText): React.ReactNode {
    if ("Text" in typedText) {
        return typedText["Text"];
    } else if ("Typed" in typedText) {
        const [battleType, text] = typedText["Typed"];
        switch (battleType) {
            case "Id":
                return <b>{text}</b>;
            case "Attack":
                return <b>{text}</b>;
            case "Damage":
                return <b>{text}</b>;
        }
    } else {
        throw new Error(
            `Invalid TypedText encountered: ${JSON.stringify(typedText)}`
        );
    }
}

export default function BattleHistory(props: {
    history: BattleHistoryEntry[];
}) {
    return (
        <ol>
            {props.history.map((entry, index) => (
                <li key={index}>{entry.map(convert)}</li>
            ))}
        </ol>
    );
}
