-- Your SQL goes here
ALTER TABLE "song"
    ADD COLUMN "album" TEXT NOT NULL default '';

ALTER TABLE "song"
    ADD COLUMN "duration_ms" INTEGER NOT NULL default 0;

ALTER TABLE "song"
    ADD COLUMN "image" TEXT NOT NULL default 'https://via.placeholder.com/256';

ALTER TABLE "song"
    DROP COLUMN "artist";
