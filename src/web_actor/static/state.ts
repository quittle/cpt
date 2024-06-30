import * as messages from "./messages.js";
import { Battle } from "./battle.ts";

const ul = document.getElementsByTagName("ul")[0];

let battle: Battle;

// messages.addEventListener("battle_state", (e) => {
//     const li = document.createElement("li");
//     li.textContent = `Received Chat: ${e.data}`;
//     ul.appendChild(li);

//     battle = JSON.parse(e.data);
// });

// document
//     .getElementsByTagName("button")[0]
//     .addEventListener("click", async () => {
//         await fetch("/act", {
//             method: "POST",
//             headers: {
//                 "Content-Type": "application/json",
//             },
//             body: JSON.stringify({
//                 card_id: battle?.card_id,
//                 target_id: battle?.character_id,
//             }),
//         });
//     });
