-- This file should undo anything in `up.sql`

ALTER TABLE "room"
    DROP COLUMN "creation_date";
