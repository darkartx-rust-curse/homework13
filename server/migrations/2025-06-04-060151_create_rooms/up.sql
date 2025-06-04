-- Your SQL goes here

CREATE TABLE rooms (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	name varchar NOT NULL,
	created_at timestamp DEFAULT now() NOT NULL,
	updated_at timestamp DEFAULT now() NOT NULL,
	CONSTRAINT room_pk PRIMARY KEY (id),
	CONSTRAINT room_name_unique UNIQUE (name)
)
