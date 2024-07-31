import {
  ActionTarget,
  Battle,
  Card,
  CardAction,
  Character,
  CharacterId,
} from "./battle";

export function getActionTarget(action: CardAction): ActionTarget {
  return action["Damage"]?.target ?? action["Heal"]?.target;
}

export function getCardTarget(card: Card): ActionTarget {
  let defaultTarget = ActionTarget.Me;
  for (const action of card.actions) {
    const target = getActionTarget(action);
    switch (target) {
      case ActionTarget.Any:
        // Any is any
        return ActionTarget.Any;
      case ActionTarget.Others:
        // If others found, then it should target others
        defaultTarget = ActionTarget.Others;
        break;
      case ActionTarget.Me:
        break;
    }
  }

  return defaultTarget;
}

export function getLivingCharacters(battle: Battle): Character[] {
  return Object.values(battle.characters).filter(
    (character) => character.health > 0,
  );
}

export function getLivingEnemies(
  battle: Battle,
  player: CharacterId,
): Character[] {
  return getLivingCharacters(battle).filter(
    (character) => character.id != player,
  );
}

export function assetPath(rawAssetPath: string): string {
  return `ref/${rawAssetPath}`;
}

export interface Coordinate {
  x: number;
  y: number;
}

export function isAdjacent(a?: Coordinate, b?: Coordinate): boolean {
  if (a === undefined || b === undefined) {
    return false;
  }
  return (
    (a.x === b.x && (a.y === b.y - 1 || a.y === b.y + 1)) ||
    (a.y === b.y && (a.x === b.x - 1 || a.x === b.x + 1))
  );
}
