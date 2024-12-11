-- Create the namespace
DEFINE NAMESPACE yattt_backend;
USE NAMESPACE yattt_backend;

-- Create the database
DEFINE DATABASE yattt_backend;
USE DATABASE yattt_backend;

-- Create the User table
DEFINE TABLE User SCHEMALESS;
DEFINE FIELD username ON User TYPE string;
DEFINE FIELD password ON User TYPE string;

-- Create the Attendance table
DEFINE TABLE Attendance SCHEMALESS;
DEFINE FIELD User ON Attendance TYPE record<User>; -- Reference to User
DEFINE FIELD deviceId ON Attendance TYPE string;
DEFINE FIELD check_in_time ON Attendance TYPE datetime;
DEFINE FIELD check_out_time ON Attendance TYPE datetime;
DEFINE FIELD duration ON Attendance TYPE float;

-- Create the Card table
DEFINE TABLE Card SCHEMALESS;
DEFINE FIELD User ON Card TYPE record<User>; -- Reference to User
DEFINE FIELD tagId ON Card TYPE string;
DEFINE FIELD name ON Card TYPE string;

-- Create the Lecture table
DEFINE TABLE Lecture SCHEMALESS;
DEFINE FIELD LV_name ON Lecture TYPE string;
DEFINE FIELD start_time ON Lecture TYPE datetime;
DEFINE FIELD end_time ON Lecture TYPE datetime;
DEFINE FIELD duration ON Lecture TYPE float;
DEFINE FIELD deviceId ON Lecture TYPE string;