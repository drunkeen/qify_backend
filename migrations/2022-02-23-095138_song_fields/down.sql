-- This file should undo anything in `up.sql`

ALTER TABLE "song"
    DROP COLUMN "album";

ALTER TABLE "song"
    DROP COLUMN "duration_ms";

ALTER TABLE "song"
    DROP COLUMN "image";

ALTER TABLE "song"
    ADD COLUMN "artist" TEXT NOT NULL DEFAULT '';
