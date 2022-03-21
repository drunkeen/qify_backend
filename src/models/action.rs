use crate::models::ServiceResult;
use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::schema::actions;

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct RoomAction {
    pub id: i32,

    pub room_id: String,
    pub action: String,
    pub timestamp: std::time::SystemTime,
}

#[allow(dead_code)]
#[derive(Insertable, Serialize, Deserialize, Debug, Eq, PartialEq, Hash, AsChangeset)]
#[table_name = "actions"]
pub struct NewRoomAction {
    pub room_id: String,
    pub action: String,
    pub timestamp: std::time::SystemTime,
}

pub fn _get_actions(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    room_id_full: String,
) -> ServiceResult<Vec<RoomAction>> {
    use crate::schema::actions::dsl;

    let connection = pool.get().expect("Could not create connection");
    let res = actions::table
        .filter(dsl::room_id.eq(room_id_full))
        .load::<RoomAction>(&connection)?;

    Ok(res)
}

pub fn _get_last_action(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    room_id_full: String,
) -> ServiceResult<RoomAction> {
    use crate::schema::actions::dsl;

    let connection = pool.get().expect("Could not create connection");
    let res = actions::table
        .filter(dsl::room_id.eq(room_id_full))
        .order_by(dsl::timestamp.desc())
        .first::<RoomAction>(&connection)?;

    Ok(res)
}

pub fn push_actions(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    room_id_full: String,
    action: String,
) -> ServiceResult<Vec<RoomAction>> {
    use crate::schema::actions::dsl;

    let val = NewRoomAction {
        action,
        timestamp: std::time::SystemTime::now(),
        room_id: room_id_full,
    };

    let connection = pool.get().expect("Could not create connection");
    let res = diesel::insert_into(dsl::actions)
        .values(&val)
        .on_conflict(dsl::room_id)
        .do_update()
        .set(&val)
        .get_results::<RoomAction>(&connection)?;

    Ok(res)
}
