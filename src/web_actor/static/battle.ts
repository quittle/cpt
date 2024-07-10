export enum CharacterRace {
    Human = "Human",
}

export type CardId = number;
export type CharacterId = number;

export interface Character {
    id: CharacterId;
    name: string;
    race: CharacterRace;
    hand: CardId[];
    deck: CardId[];
    health: number;
    hand_size: number;
}

export interface Team {
    id: number;
    name: string;
}

export type BattleType = "Id" | "Attack" | "Damage";

export type TypedText =
    | {
          Text: string;
      }
    | {
          Typed: [BattleType, string];
      };

export type BattleHistoryEntry = TypedText[];

export enum ActionTarget {
    Me = "Me",
    Others = "Others",
    Any = "Any",
}

export type CardAction =
    | {
          Damage: {
              target: ActionTarget;
              amount: number;
          };
      }
    | {
          Heal: {
              target: ActionTarget;
              amount: number;
          };
      };

export interface Card {
    id: CardId;
    name: string;
    description: string;
    flavor?: string;
    actions: CardAction[];
}

export interface Battle {
    characters: Record<string, Character>;
    teams: Team[];
    history: BattleHistoryEntry[];
    round: number;
    cards: Record<string, Card>;
}

export interface BattleState {
    character_id: number;
    battle: Battle;
}
