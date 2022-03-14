-- Your SQL goes here

CREATE TABLE "actions" (
    "id" SERIAL PRIMARY KEY,

    "room_id" VARCHAR(64) NOT NULL,

    "action" VARCHAR(16) NOT NULL,
    "timestamp" TIMESTAMP NOT NULL,

    FOREIGN KEY ("room_id")
        REFERENCES "room"("room_id")
            ON DELETE CASCADE
);

ALTER TABLE "room"
    ADD COLUMN "latest_track" VARCHAR(36) NOT NULL UNIQUE default '';
