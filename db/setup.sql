-- Database
CREATE DATABASE twilio;
CREATE USER twilio_admin WITH ENCRYPTED PASSWORD 'twilio@123';
ALTER DATABASE twilio OWNER TO twilio_admin;
GRANT ALL PRIVILEGES ON DATABASE twilio TO twilio_admin;
GRANT USAGE, CREATE ON SCHEMA public TO twilio_admin;
ALTER DATABASE twilio SET TIMEZONE TO 'Asia/Kolkata';
ALTER USER twilio_admin CREATEDB CREATEROLE LOGIN;
-- GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA public TO twilio_admin;
-- GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO twilio_admin;

/*
To open in postgres database
psql -U postgres -h localhost
*/
