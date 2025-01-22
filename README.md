
![image](https://github.com/user-attachments/assets/14ce9ad3-0db7-42cd-8db7-b89e2bd40807)

# YATTT
YATTT is yat another time tracking tool - very nice

## Project Structure

### `yattt_data/`
The python based data-service:
- Provides an admin interface.
- Communication with the yattt_backend.
- Communicates with scanner devices.
- Validates scanned data.

### `yattt_backend/`
The backend of the application, built with Rust, featuring:
- **SurrealDB integration** for data storage.
- RESTful APIs for card management, authentication, and retrieval of attendance information.
- Deployment tools like Docker for easy setup.

### `yatt_frontend/yatt_web/`
A web-based interface built with Angular:
- Frontend solution to enable user authentication, and card management.

### `yattt_app/` and `yattt_ios/` as Scanner platforms
The Android client, written in Kotlin:
- Implements card scanning functionalities.
