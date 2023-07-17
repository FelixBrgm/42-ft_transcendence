-- Your SQL goes here
CREATE TABLE clients (
	id serial NOT NULL,
	title text NOT NULL,
	is_online boolean NOT NULL,
	CONSTRAINT clients_pkey PRIMARY KEY (id)
);