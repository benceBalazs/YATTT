import { Component, OnInit } from '@angular/core';
import {Router, NavigationEnd, RouterOutlet} from '@angular/router';
import {NavbarComponent} from './navbar/navbar.component';
import {NgIf} from '@angular/common';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  imports: [
    NavbarComponent,
    RouterOutlet,
    NgIf
  ],
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  isLoginRoute = false;

  constructor(private router: Router) {}

  ngOnInit() {
    this.router.events.subscribe((event) => {
      if (event instanceof NavigationEnd) {
        console.log('Current URL:', event.url);
        this.isLoginRoute = event.url === '/login';
        console.log('isLoginRoute:', this.isLoginRoute);
      }
    });
  }
}
