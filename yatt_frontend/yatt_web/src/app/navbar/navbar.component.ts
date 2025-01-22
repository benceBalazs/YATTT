import { Component, OnInit } from '@angular/core';
import { AuthService } from '../../services/auth.service';
import { Router } from '@angular/router';
import { ThemeService } from '../../services/theme.service';
import {NgIf} from '@angular/common';
import {MatButton, MatIconButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.css'],
  imports: [
    NgIf,
    MatButton,
    MatIconButton,
    MatIcon
  ]
})
export class NavbarComponent implements OnInit {
  isDarkMode: boolean;
  username: string | null = null;

  constructor(
    private themeService: ThemeService,
    private authService: AuthService,
    private router: Router
  ) {
    this.isDarkMode = this.themeService.isDarkMode();
  }

  toggleTheme() {
    this.isDarkMode = !this.isDarkMode;
    this.themeService.setDarkMode(this.isDarkMode);
  }

  ngOnInit(): void {
    // Subscribe to user state
    this.authService.user$.subscribe((user) => {
      console.log("Subscribed to user state:", user);
      this.username = user; // Update dynamically
    });

    // Optionally initialize from localStorage (for page refreshes)
    this.username = this.authService.getUser();
    console.log("localstorage", this.authService.getUser());
  }

  logout(): void {
    this.authService.clearUser(); // Clear user state
    this.router.navigate(['/login']); // Redirect to login page
  }
}
