-- This file should undo anything in `up.sql`

ALTER TABLE "room"
    DROP COLUMN "room_id_short";
