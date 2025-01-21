import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class DataApiService {
  private baseUrl = 'http://localhost:8080/api/v1/';

  constructor(private http: HttpClient) {}

  login(username: string, password: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/login`, { username, password });
  }

  register(username: string, password: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/register`, { username, password });
  }

  refreshToken(): Observable<any> {
    const token = localStorage.getItem('access_token');
    const headers = new HttpHeaders().set('Authorization', `Bearer ${token}`);
    return this.http.post(`${this.baseUrl}`, {}, { headers });
  }

  getHeaders(): HttpHeaders {
    const token = localStorage.getItem('access_token');
    return new HttpHeaders().set('Authorization', `Bearer ${token}`);
  }

  getAttendances(): Observable<any> {
    const headers = {
      Authorization:
        'Bearer ' +
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Mzc0NjYyMzIsImlhdCI6MTczNzM3OTgzMiwidXNlcl9pZCI6InlhbTY5bjJsbm1ibzN2MWMxdjl5In0.9CJ2PDGma--zt3AuchIFeh9NJUI43D3IXRw1_5ibSqU',
    };
    return this.http.get<Observable<any>>(this.baseUrl + 'attendances', {
      headers,
    });
  }

  getCards(): Observable<any> {
    return this.http.get<Observable<any>>(this.baseUrl + 'cards');
  }
  addCard(card: { tag_id: string; name: string }): Observable<any> {
    return this.http.post(this.baseUrl + 'cards', card);
  }

  updateCard(tag_id: string, card: { tag_id: string; name: string }): Observable<any> {
    return this.http.put(this.baseUrl + 'cards', card);
  }

  deleteCard(tag_id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/cards/${tag_id}`);
  }
}
