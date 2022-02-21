table! {
    room (room_id) {
        room_id -> Varchar,
        spotify_id -> Varchar,
        room_id_short -> Varchar,
        creation_date -> Timestamp,
    }
}

table! {
    song (id) {
        id -> Int4,
        uri -> Text,
        artist -> Text,
        title -> Text,
        room_id -> Varchar,
    }
}

table! {
    spotify (id) {
        id -> Int4,
        spotify_id -> Varchar,
        access_token -> Varchar,
        refresh_token -> Varchar,
        expire_date -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(room, song, spotify,);
