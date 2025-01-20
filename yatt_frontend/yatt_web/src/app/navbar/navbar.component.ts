import { Component } from '@angular/core';
import {MatIcon} from "@angular/material/icon";
import {ThemeService} from '../theme.service';
import {DataApiService} from '../data-api.service';
import {Router, RouterOutlet} from '@angular/router';
import {HttpClient} from '@angular/common/http';

@Component({
  selector: 'app-navbar',
    imports: [
        MatIcon,
    ],
  templateUrl: './navbar.component.html',
  styleUrl: './navbar.component.css'
})
export class NavbarComponent {

  isDarkMode: boolean;

  constructor(private themeService: ThemeService, private dataApiService: DataApiService, private router: Router, private http: HttpClient) {
    this.isDarkMode = this.themeService.isDarkMode();
  }

  toggleTheme() {
    this.isDarkMode = !this.isDarkMode;
    this.themeService.setDarkMode(this.isDarkMode);
  }

}
