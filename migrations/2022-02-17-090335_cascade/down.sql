-- This file should undo anything in `up.sql`
ALTER TABLE "spotify"
    DROP CONSTRAINT "fk_room";

ALTER TABLE "song"
    DROP CONSTRAINT "fk_room";
