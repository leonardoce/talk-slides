#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use diesel::PgConnection;
use rocket_contrib::json::Json;
use serde::Serialize;

use dotenv;
use std::env;

#[derive(Serialize)]
struct Note {
    pub name: String,
    pub frequency: f64,
}

#[derive(Serialize)]
struct ScaleResponse {
    pub notes: Vec<Note>,
}

#[get("/notes")]
fn get_notes() -> Option<Json<ScaleResponse>> {
    use diesel::prelude::*;
    use rtunerbis::schema::notes;

    let conn = create_connection()
        .expect("Can't connect to database");

    notes::table
        .select((notes::note_name, notes::frequency))
        .load::<(String, f64)>(&conn)
        .ok()
        .map(|data| Json(ScaleResponse {
            notes: data.into_iter().map(|(name, frequency)| Note {
                name: name,
                frequency: frequency
            }).collect()
        }))
}

#[derive(Serialize)]
struct NoteResponse {
    pub name: String,
    pub expected_frequency: f64,
    pub diff_cents: f64
}

#[get("/notes/<frequency>")]
fn get_from_frequency(frequency: f64) -> Option<Json<NoteResponse>> {
    use diesel::prelude::*;
    use rtunerbis::schema::notes;
    use rtunerbis::abs;

    let conn = create_connection()
        .expect("Can't connect to database");

    notes::table
        .select((notes::note_name, notes::frequency, abs(notes::frequency - frequency)))
        .order_by(abs(notes::frequency - frequency))
        .first::<(String, f64, f64)>(&conn)
        .ok()
        .map(|(name, db_frequency, _freq_diff)| Json(NoteResponse {
                name: name,
                expected_frequency: db_frequency,
                diff_cents: rtunerbis::notes::diff_cents(db_frequency, frequency)
        }))
}

/// Create a connection to PostgreSQL
fn create_connection() -> diesel::ConnectionResult<PgConnection> {
    use diesel::prelude::*;
    PgConnection::establish(&env::var("DATABASE_URL").unwrap())
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite()
        .mount("/", rocket::routes!(get_notes, get_from_frequency))
        .launch();
}
