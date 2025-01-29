import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LoginComponent } from './login.component';
import {By} from '@angular/platform-browser';

describe('LoginComponent', () => {
  let component: LoginComponent;
  let fixture: ComponentFixture<LoginComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LoginComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(LoginComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should disable the login button when the form is invalid', () => {
    component.loginForm.controls['username'].setValue('');
    component.loginForm.controls['password'].setValue('');
    fixture.detectChanges();

    const button = fixture.debugElement.query(By.css('button[type="submit"]'));
    expect(button.nativeElement.disabled).toBeTruthy();
  });

});
