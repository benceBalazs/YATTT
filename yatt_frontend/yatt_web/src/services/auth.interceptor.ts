import { inject, Injectable } from '@angular/core';
import { HttpInterceptor, HttpRequest, HttpHandler, HttpEvent, HttpHandlerFn, HttpEventType } from '@angular/common/http';
import { Observable, tap } from 'rxjs';
import { DataApiService } from './data-api.service';

@Injectable({
  providedIn: 'root',
})
export class AuthInterceptor implements HttpInterceptor {
  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    const token = localStorage.getItem('access_token'); // Retrieve token from local storage
    console.log('token', token);
    if (token) {
      req = req.clone({
        setHeaders: {
          Authorization: `Bearer ${token}`,
        },
      });
    }
    return next.handle(req);
  }
}

// export function intercept(req: HttpRequest<any>, next: Next): Observable<HttpEvent<any>> {
//   const token = localStorage.getItem('access_token'); // Retrieve token from local storage
//   console.log('token', token);
//   if (token) {
//     req = req.clone({
//       setHeaders: {
//         Authorization: `Bearer ${token}`,
//       },
//     });
//   }
//   return next.handle(req);
// }

export function authInterceptor(req: HttpRequest<unknown>, next: HttpHandlerFn) {
  // Inject the current `AuthService` and use it to get an authentication token:
  const token = localStorage.getItem('access_token')!;
  console.log('token', token);
  // Clone the request to add the authentication header.
  const newReq = req.clone({
    headers: req.headers.append('Authorization', `Bearer ${token}`),
  });

  return next(newReq);
}
