table! {
    actions (id) {
        id -> Int4,
        room_id -> Varchar,
        action -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    room (room_id) {
        room_id -> Varchar,
        spotify_id -> Varchar,
        room_id_short -> Varchar,
        creation_date -> Timestamp,
        latest_track -> Varchar,
    }
}

table! {
    song (id) {
        id -> Int4,
        uri -> Text,
        title -> Text,
        room_id -> Varchar,
        album -> Text,
        duration_ms -> Int4,
        image -> Text,
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

joinable!(actions -> room (room_id));
joinable!(song -> room (room_id));

allow_tables_to_appear_in_same_query!(
    actions,
    room,
    song,
    spotify,
);
