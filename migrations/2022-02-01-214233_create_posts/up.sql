-- Your SQL goes here

CREATE TABLE "spotify" (
    "id" SERIAL PRIMARY KEY,

    "spotify_id" VARCHAR(64) NOT NULL,

    "access_token" TEXT NOT NULL,
    "refresh_token" TEXT NOT NULL,
    "refresh_date" TIMESTAMP NOT NULL
);

CREATE TABLE "song" (
    "id" SERIAL PRIMARY KEY,

    "uri" TEXT NOT NULL,
    "artist" TEXT NOT NULL,
    "title" TEXT NOT NULL,

    "room_id" VARCHAR(64) NOT NULL
);

CREATE TABLE "room" (
    "room_id" VARCHAR(64) NOT NULL PRIMARY KEY,
    "spotify_id" VARCHAR(64) NOT NULL
);
