import { CardId, CharacterId } from "./battle.ts";

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
