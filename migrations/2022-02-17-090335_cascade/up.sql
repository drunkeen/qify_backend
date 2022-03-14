-- Your SQL goes here
ALTER TABLE "spotify"
    ADD FOREIGN KEY ("spotify_id")
        REFERENCES "room"("spotify_id")
            ON DELETE CASCADE;

ALTER TABLE "song"
    ADD FOREIGN KEY ("room_id")
        REFERENCES "room"("room_id")
            ON DELETE CASCADE;
