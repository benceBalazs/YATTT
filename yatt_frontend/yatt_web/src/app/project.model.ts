export class Project {
  //access_token: string;
  card_name?: string;
  lecture_name?: string;
  check_in_time?: number;
  check_out_time?: number;
  duration?: number;
}

export interface Card {
  tag_id: string;
  card_name: string;
}

export type RecordId = {
  id?: {
    tb: string;
    id: {
      "String": string;
    }
  }
}

export type CardWithId = Card & RecordId;

export type GetCardResponse = { cards: CardWithId[]};
