use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv;
use std::env;
use rtunerbis;

fn main() {
    dotenv::dotenv().ok();

    let conn = PgConnection::establish(
        &env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable")
    ).expect("Can't connect to PostgreSQL");

    delete_from_notes(&conn)
        .expect("Can't delete from notes table");
    insert_notes(&conn)
        .expect("Can't insert notes");
}

fn delete_from_notes(conn: &PgConnection) -> diesel::QueryResult<usize> {
    use rtunerbis::schema::notes;
    diesel::delete(notes::table).execute(conn)
}

fn insert_notes(conn: &PgConnection) -> Result<(), diesel::result::Error> {
    use rtunerbis::schema::notes;

    for (note_name, frequency) in rtunerbis::notes::EqualTemperamentScale::new() {
        diesel::insert_into(notes::table)
            .values((notes::note_name.eq(note_name), notes::frequency.eq(frequency)))
            .execute(conn)?;
    }

    Ok(())
}