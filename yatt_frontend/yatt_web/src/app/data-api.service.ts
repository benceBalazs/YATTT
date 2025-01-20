import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class DataApiService {
  private baseUrl = 'http://localhost:8080/api/v1/';
  constructor(private http: HttpClient) {}

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

  getAuth(): Observable<any> {
    return this.http.get<Observable<any>>(this.baseUrl + 'auth');
  }

  register(body: any): Observable<any> {
    return this.http.post<Observable<any>>(this.baseUrl + 'register', body);
  }
}
