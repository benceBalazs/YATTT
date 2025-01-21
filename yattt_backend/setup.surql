-- Create the namespace
DEFINE NAMESPACE yattt_backend;
USE NAMESPACE yattt_backend;

-- Create the database
DEFINE DATABASE yattt_backend;
USE DATABASE yattt_backend;

-- Create the User table
DEFINE TABLE User SCHEMAFULL;
DEFINE FIELD username ON User TYPE string ASSERT $value != NONE;
DEFINE FIELD password ON User TYPE string ASSERT $value != NONE;
DEFINE INDEX idx_unique_username ON User FIELDS username UNIQUE; -- Enforce unique username

-- Create the Attendance table
DEFINE TABLE Attendance SCHEMAFULL;
DEFINE FIELD user_id ON Attendance TYPE record<User>; -- Reference to User
DEFINE FIELD device_id ON Attendance TYPE string;
DEFINE FIELD tag_id ON Attendance TYPE string;
DEFINE FIELD check_in_time ON Attendance TYPE datetime;
DEFINE FIELD check_out_time ON Attendance TYPE datetime;
DEFINE FIELD duration ON Attendance TYPE float;

-- Create the Card table
DEFINE TABLE Card SCHEMAFULL;
DEFINE FIELD user_id ON Card TYPE record<User>; -- Reference to User
DEFINE FIELD tag_id ON Card TYPE string;
DEFINE FIELD card_name ON Card TYPE string;
DEFINE INDEX idx_unique_tag_id ON Card FIELDS tag_id UNIQUE;

-- Create the Lecture table
DEFINE TABLE Lecture SCHEMAFULL;
DEFINE FIELD lv_name ON Lecture TYPE string;
DEFINE FIELD start_time ON Lecture TYPE datetime;
DEFINE FIELD end_time ON Lecture TYPE datetime;
DEFINE FIELD duration ON Lecture TYPE float;
DEFINE FIELD device_id ON Lecture TYPE string;


-- create a test user
LET $new_user = (INSERT INTO User (username, password) VALUES ('testUser', '$2y$13$Ya/BG6UTw/A4SyC/uIVL9OYK4WyaMlnEyAUBpA7oXfdEKYJFvWYJe'));

-- insert test card
INSERT INTO Card (user_id, card_name, tag_id) 
VALUES ($new_user.id[0], 'My AAU Card', 'cardTagId5');


-- insert example lectures
INSERT INTO Lecture (lv_name, start_time, end_time, duration, device_id) VALUES
    ('Scientific Writing about Scientific Writing', d'2024-12-11T08:15:00Z', d'2024-12-11T09:45:00Z', 1.5, 'S2.69-Bitmovin-1'),
    ('Tutorium zu Scientific about Scientific Writing', d'2024-12-11T08:00:00Z', d'2024-12-11T09:00:00Z', 1.0, 'online-1'),
    ('Seminar in Business, Money Printing, and the mindless consumer model', d'2024-12-11T08:15:00Z', d'2024-12-11T09:45:00Z', 1.5, 'S1.42-1'),
    ('Scientific introduction to Science', d'2024-12-11T13:30:00Z', d'2024-12-11T15:00:00Z', 1.5, 'S2.69-Bitmovin-2'),
    ('Process Engineering', d'2024-12-11T13:30:00Z', d'2024-12-11T15:00:00Z', 1.5, 'S2.42-1'),
    ('Current Topics in Interactive Systems, Design another prototype', d'2024-12-11T11:45:00Z', d'2024-12-11T13:15:00Z', 1.5, 'S2.69-Bitmovin-3'),
    ('Data Engineering: SQL introduction Part 11', d'2024-12-11T15:15:00Z', d'2024-12-11T16:45:00Z', 1.5, 'HS8-1'),
    ('Distributed Computing Infrastructures: Vs of BigData', d'2024-12-11T08:15:00Z', d'2024-12-11T09:45:00Z', 1.5, 'V1.01-1'),
    ('Advanced Advanced Software Engineering', d'2024-12-11T13:30:00Z', d'2024-12-11T15:00:00Z', 1.5, 'S2.42-2'),
    ('Tutorial Advanced Advanced Software Engineering: Hands-On Debugging', d'2024-12-11T13:30:00Z', d'2024-12-11T15:00:00Z', 1.5, 'S2.42-3'),
    ('Current Topics in Current Topics', d'2024-12-11T11:45:00Z', d'2024-12-11T13:15:00Z', 1.5, 'S2.69-Bitmovin-4'),
    ('Serverless Distributed Systems: Papers and kubectl', d'2024-12-11T10:00:00Z', d'2024-12-11T11:30:00Z', 1.5, 'S1.42-2'),
    ('Current Topics in Software Engineering', d'2024-12-11T10:00:00Z', d'2024-12-11T11:30:00Z', 1.5, 'S2.69-Bitmovin-5'),
    ('Selected Topics in Software Engineering', d'2024-12-11T15:15:00Z', d'2024-12-11T16:45:00Z', 1.5, 'S2.69-Bitmovin-6'),
    ('Fundamental Topics of Java', d'2024-12-11T13:30:00Z', d'2024-12-11T15:00:00Z', 1.5, 'N1.71-1');