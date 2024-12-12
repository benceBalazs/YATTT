import { Component, OnInit } from '@angular/core';
import {FormGroup, FormBuilder, Validators, ReactiveFormsModule} from '@angular/forms';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
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
  public loginForm!: FormGroup;
  public errorMessage: string = '';

  constructor(
    private formBuilder: FormBuilder,
    private http: HttpClient,
    private router: Router
  ) { }

  ngOnInit(): void {
    // Initialize form with validation rules
    this.loginForm = this.formBuilder.group({
      username: ['', [Validators.required]],
      password: ['', [Validators.required]]
    });
  }

  signIn() {
    if (this.loginForm.invalid) {
      this.errorMessage = 'Please fill all fields correctly!';
      return;
    }

    const credentials = {
      username: this.loginForm.value.username,
      password: this.loginForm.value.password
    };

    this.http.post<any>('http://localhost:3000/auth/login', credentials)
      .subscribe({
        next: (response) => {
          alert('Login Successful!');
          localStorage.setItem('token', response.token); // Store the token locally
          this.router.navigate(['/home']);
        },
        error: (err) => {
          this.errorMessage = 'Invalid username or password';
          console.error('Login error', err);
        }
      });
  }

  signUp(){

  }
}
