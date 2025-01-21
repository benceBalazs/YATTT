import { bootstrapApplication } from '@angular/platform-browser';
import { provideRouter } from '@angular/router';
import { provideHttpClient, withInterceptorsFromDi } from '@angular/common/http';
import { AppComponent } from './app/app.component';
import { routes} from './app/app.routes';
import {AuthService} from './services/auth.service';

bootstrapApplication(AppComponent, {
  providers: [
    provideRouter(routes), // Provide routes
    provideHttpClient(withInterceptorsFromDi()), // Provide HTTP client with interceptors
    AuthService,
  ],
}).catch((err) => console.error(err));
