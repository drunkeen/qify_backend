-- This file should undo anything in `up.sql`

DROP TABLE "actions";

ALTER TABLE "room"
    DROP COLUMN "latest_track";
