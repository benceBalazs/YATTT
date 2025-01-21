import { Component } from '@angular/core';
import {MatIcon} from "@angular/material/icon";
import {ThemeService} from '../theme.service';
import {DataApiService} from '../data-api.service';
import {Router, RouterOutlet} from '@angular/router';
import {HttpClient} from '@angular/common/http';
import {NgIf} from '@angular/common';
import {MatButton, MatIconButton} from '@angular/material/button';

@Component({
  selector: 'app-navbar',
  imports: [
    MatIcon,
    NgIf,
    MatButton,
    MatIconButton,
  ],
  templateUrl: './navbar.component.html',
  styleUrl: './navbar.component.css'
})
export class NavbarComponent {
  isDarkMode: boolean;
  username: string | null = '';

  constructor(private themeService: ThemeService, private dataApiService: DataApiService, private router: Router, private http: HttpClient) {
    this.isDarkMode = this.themeService.isDarkMode();
  }

  toggleTheme() {
    this.isDarkMode = !this.isDarkMode;
    this.themeService.setDarkMode(this.isDarkMode);
  }

  ngOnInit(): void {
    if (typeof window !== 'undefined' && localStorage) {
      this.username = localStorage.getItem('username');
    }
  }

  logout(): void {
    if (typeof window !== 'undefined' && localStorage) {
      localStorage.clear(); // Clear all user data
      this.router.navigate(['/login']);
    }
  }

}
