-- Your SQL goes here
CREATE TABLE app_user (
	id INT PRIMARY KEY,
	intra VARCHAR(255) NOT NULL,
	alias VARCHAR(255) NOT NULL,
	avatar VARCHAR(255) NOT NULL DEFAULT 'https://i.pinimg.com/564x/bc/5d/17/bc5d173a3001839b5f4ec29efad072ae.jpg',
	password BYTEA NULL,
	status VARCHAR(255) NOT NULL DEFAULT 'online',
	wins INTEGER NOT NULL DEFAULT 0, 
	losses INTEGER NOT NULL DEFAULT 0
);