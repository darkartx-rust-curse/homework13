-- Your SQL goes here

CREATE TABLE devices (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
    room_id uuid NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
	name varchar NOT NULL,
	created_at timestamp DEFAULT now() NOT NULL,
	updated_at timestamp DEFAULT now() NOT NULL,
	CONSTRAINT device_pk PRIMARY KEY (id)
);

CREATE UNIQUE INDEX index_devices_on_room_id_and_name ON devices USING btree (room_id, name);
