import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import { Router } from '@angular/router';
import { DataApiService } from '../../services/data-api.service';
import { AuthService } from '../../services/auth.service';
import {NgIf} from '@angular/common';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css'],
  imports: [
    ReactiveFormsModule,
    NgIf
  ]
})
export class LoginComponent implements OnInit {
  loginForm: FormGroup;
  errorMessageSignIn: string = '';
  errorMessageRegister: string = '';

  constructor(
    private fb: FormBuilder,
    private apiService: DataApiService,
    private router: Router,
    private authService: AuthService // Inject AuthService here
  ) {
    this.loginForm = this.fb.group({
      username: ['', Validators.required],
      password: ['', [Validators.required, Validators.minLength(8)]],
    });
  }

  ngOnInit(): void {}

  signIn(): void {
    const { username, password } = this.loginForm.value;

    this.apiService.login(username, password).subscribe({
      next: (response) => {
        // Store token and update AuthService
        localStorage.setItem('access_token', response.access_token);
        this.authService.setUser(username); // Update the user state
        this.router.navigate(['/home']); // Redirect to home
      },
      error: (err) => {
        this.errorMessageSignIn =
          err.status === 400 || err.status === 401
            ? 'Invalid username or password'
            : 'An unexpected error occurred. Please try again.';
      },
    });
  }

  register(): void {
    const { username, password } = this.loginForm.value;

    this.apiService.register(username, password).subscribe({
      next: (response) => {
        localStorage.setItem('access_token', response.access_token);
        this.authService.setUser(username); // Update the user state
        this.router.navigate(['/home']); // Redirect to home
      },
      error: (err) => {
        this.errorMessageRegister =
          err.status === 400
            ? 'Registration failed. Please try again.'
            : 'An unexpected error occurred. Please try again.';
      },
    });
  }
}
