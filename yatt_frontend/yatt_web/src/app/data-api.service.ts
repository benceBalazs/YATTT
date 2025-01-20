import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class DataApiService {
private baseUrl = 'http://localhost:4200/';
  constructor(private http: HttpClient) { }

  getAttendances() : Observable<any> {
    return this.http.get<Observable<any>>(this.baseUrl + 'attendances');
  }

  getAuth() : Observable<any> {
    return this.http.get<Observable<any>>(this.baseUrl + 'auth');
  }

}
