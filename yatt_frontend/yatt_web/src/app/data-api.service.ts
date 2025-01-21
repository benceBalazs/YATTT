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

  refreshToken(): Observable<any> {
    const token = localStorage.getItem('access_token');
    const headers = new HttpHeaders().set('Authorization', `Bearer ${token}`);
    return this.http.post(`${this.baseUrl}/`, {}, { headers });
  }

  getHeaders(): HttpHeaders {
    const token = localStorage.getItem('access_token');
    return new HttpHeaders().set('Authorization', `Bearer ${token}`);
  }

  getAttendances(): Observable<any> {
    const headers = {
      Authorization:
        'Bearer ' +
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mzc1NTY4NzAsImlhdCI6MTczNzQ3MDQ3MCwidXNlcl9pZCI6ImRwY2l3azVhcmR0OHpxeWV1aG9tIn0.Pv-XUn9sPsmngbLW0ltxqqXUe1q8XopgxLvV5emqPLs',
    };
    return this.http.get<Observable<any>>(this.baseUrl + '/attendances', {
      headers,
    });
  }

  getCards(): Observable<GetCardResponse> {
    const headers = {
      Authorization:
        'Bearer ' +
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mzc1NTY4NzAsImlhdCI6MTczNzQ3MDQ3MCwidXNlcl9pZCI6ImRwY2l3azVhcmR0OHpxeWV1aG9tIn0.Pv-XUn9sPsmngbLW0ltxqqXUe1q8XopgxLvV5emqPLs',
    };
    return this.http.get<GetCardResponse>(this.baseUrl + '/cards', { headers });
  }
  addCard(card: Card): Observable<any> {
    const headers = {
      Authorization:
        'Bearer ' +
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mzc1NTY4NzAsImlhdCI6MTczNzQ3MDQ3MCwidXNlcl9pZCI6ImRwY2l3azVhcmR0OHpxeWV1aG9tIn0.Pv-XUn9sPsmngbLW0ltxqqXUe1q8XopgxLvV5emqPLs',
    };
    return this.http.post(this.baseUrl + '/cards', card, { headers });
  }

  updateCard(card_id: string, card: Card): Observable<any> {
    const headers = {
      Authorization:
        'Bearer ' +
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mzc1NTY4NzAsImlhdCI6MTczNzQ3MDQ3MCwidXNlcl9pZCI6ImRwY2l3azVhcmR0OHpxeWV1aG9tIn0.Pv-XUn9sPsmngbLW0ltxqqXUe1q8XopgxLvV5emqPLs',
    };
    return this.http.put(`${this.baseUrl}/cards/${card_id}`, card, {headers});
  }

  deleteCard(card_id: string): Observable<any> {
    const headers = {
      Authorization:
        'Bearer ' +
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mzc1NTY4NzAsImlhdCI6MTczNzQ3MDQ3MCwidXNlcl9pZCI6ImRwY2l3azVhcmR0OHpxeWV1aG9tIn0.Pv-XUn9sPsmngbLW0ltxqqXUe1q8XopgxLvV5emqPLs',
    };
    return this.http.delete(`${this.baseUrl}/cards/${card_id}`, {headers});
  }
}
