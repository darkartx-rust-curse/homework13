-- Your SQL goes here

CREATE EXTENSION pgcrypto;

CREATE TABLE houses (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	name varchar NOT NULL,
	created_at timestamp DEFAULT now() NOT NULL,
	updated_at timestamp DEFAULT now() NOT NULL,
	CONSTRAINT houses_pk PRIMARY KEY (id),
	CONSTRAINT houses_name_unique UNIQUE (name)
)
