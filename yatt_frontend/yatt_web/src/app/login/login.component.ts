import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import { Router } from '@angular/router';
import { DataApiService} from '../data-api.service';
import {NgIf} from '@angular/common';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  imports: [
    ReactiveFormsModule,
    NgIf
  ],
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {
  loginForm: FormGroup;
  errorMessage: string = '';

  constructor(
    private fb: FormBuilder,
    private apiService: DataApiService,
    private router: Router
  ) {
    this.loginForm = this.fb.group({
      username: ['', Validators.required],
      password: ['', [Validators.required, Validators.minLength(8)]]
    });
  }

  ngOnInit(): void {}

  signIn(): void {
    const { username, password } = this.loginForm.value;

    this.apiService.login(username, password).subscribe({
      next: (response) => {
        localStorage.setItem('access_token', response.access_token);
        localStorage.setItem('username', username); // Save username for the navbar
        this.router.navigate(['/home']); // Redirect to home
      },
      error: (err) => {
        if (err.status === 400 || err.status === 401) {
          this.errorMessage = 'Invalid username or password';
        } else {
          this.errorMessage = 'An unexpected error occurred. Please try again.';
        }
      }
    });
  }


  signUp(): void {
    const { username, password } = this.loginForm.value;

    if (password.length < 8) {
      this.errorMessage = 'Password must be at least 8 characters long'; // Display error for short password
      return;
    }

    this.apiService.register(username, password).subscribe({
      next: (response) => {
        localStorage.setItem('access_token', response.access_token);
        localStorage.setItem('username', username); // Save username for the navbar
        this.router.navigate(['/home']); // Redirect to home
      },
      error: (err) => {
        if (err.status === 400) {
          this.errorMessage = 'Registration failed. Please try again.';
        } else {
          this.errorMessage = 'An unexpected error occurred. Please try again.';
        }
      }
    });
  }


}
