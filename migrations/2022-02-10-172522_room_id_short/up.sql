-- Your SQL goes here
ALTER TABLE "room"
    ADD COLUMN "room_id_short" VARCHAR(6) NOT NULL UNIQUE default '' ;
