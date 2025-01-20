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
  name: string;
}
