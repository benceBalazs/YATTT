import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  private userSubject = new BehaviorSubject<string | null>(null);
  public user$ = this.userSubject.asObservable();

  // Set the logged-in user
  setUser(username: string): void {
    this.userSubject.next(username);
    localStorage.setItem('username', username);
  }

  // Get the current user
  getUser(): any {
    if (typeof window !== 'undefined' && localStorage) { //check if localStorage is defined
      return localStorage.getItem('username');
    }
    return null;
  }

  // Clear user on logout
  clearUser(): void {
    this.userSubject.next(null);
    localStorage.removeItem('username');
    localStorage.removeItem('access_token'); // Clear token as well
  }
}
