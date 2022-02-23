-- Your SQL goes here
ALTER TABLE "song"
    ADD COLUMN "album" TEXT NOT NULL UNIQUE default '';

ALTER TABLE "song"
    ADD COLUMN "duration_ms" INTEGER NOT NULL UNIQUE default 0;

ALTER TABLE "song"
    ADD COLUMN "image" TEXT NOT NULL UNIQUE default 'https://via.placeholder.com/256';

ALTER TABLE "song"
    DROP COLUMN "artist";
