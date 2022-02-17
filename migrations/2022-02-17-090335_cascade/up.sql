-- Your SQL goes here
ALTER TABLE "spotify"
    ADD CONSTRAINT "fk_room"
        FOREIGN KEY ("spotify_id")
            REFERENCES "room"("spotify_id")
                ON DELETE CASCADE;

ALTER TABLE "song"
    ADD CONSTRAINT "fk_room"
        FOREIGN KEY ("room_id")
            REFERENCES "room"("room_id")
                ON DELETE CASCADE;
