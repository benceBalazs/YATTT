import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Card, CardWithId, GetCardResponse, RecordId } from './project.model';

@Injectable({
  providedIn: 'root'
})

export class DataApiService {
  private baseUrl = 'http://localhost:8080/api/v1';

  constructor(private http: HttpClient) {}

  login(username: string, password: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/auth/login`, { username, password });
  }

  register(username: string, password: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/auth/register`, { username, password });
  }

  getAttendances(): Observable<any> {
    return this.http.get<Observable<any>>(this.baseUrl + '/attendances');
  }

  getCards(): Observable<GetCardResponse> {

    return this.http.get<GetCardResponse>(this.baseUrl + '/cards');
  }
  addCard(card: Card): Observable<any> {
    return this.http.post(this.baseUrl + '/cards', card,);
  }

  updateCard(card_id: string, card: Card): Observable<any> {

    return this.http.put(`${this.baseUrl}/cards/${card_id}`, card);
  }

  deleteCard(card_id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/cards/${card_id}`);
  }
}
