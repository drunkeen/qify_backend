-- Your SQL goes here
ALTER TABLE "room"
    ADD COLUMN "creation_date" TIMESTAMP NOT NULL UNIQUE default now();
