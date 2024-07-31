import { CardId, CharacterId } from "./battle.ts";
import { Coordinate } from "./utils.ts";

export async function takeAction(cardId: CardId, targetId: CharacterId) {
    await fetch("/act", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            card_id: cardId,
            target_id: targetId,
        }),
    });
}

export async function move(targetId: CharacterId, to: Coordinate) {
    await fetch("/move", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            target_id: targetId,
            to: to,
        }),
    });
}

export async function pass() {
    await fetch("/pass", {
        method: "POST",
    });
}
