use bloom_error::BloomError;
use bloom_messages::{contacts, Message};
use bloom_models::contacts::db;
use rusqlite::{params, Connection, NO_PARAMS};

#[cfg(target_os = "android")]
fn home_dir() -> String {
    let contents = std::fs::read_to_string("/proc/self/cmdline").unwrap();
    let contents = contents.trim_end_matches('\x00');
    return format!("/data/data/{}", &contents);
}

#[cfg(not(target_os = "android"))]
fn home_dir() -> String {
    let home = dirs::home_dir();
    match home {
        Some(res) => format!("{}/.bloom", res.to_str().unwrap()),
        None => "".to_string(),
    }
}

fn db_dir() -> String {
    let path = format!("{}/db", home_dir());
    let _ = std::fs::create_dir_all(&path);
    return path;
}

fn db_path() -> String {
    return format!("{}/bloom42.db", db_dir());
}

pub fn list_contacts(_: contacts::GuiListContacts) -> Result<Message, BloomError> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id TEXT PRIMARY KEY NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            notes TEXT NOT NULL,
            addresses TEXT NOT NULL,
            birthday TEXT,
            organizations TEXT NOT NULL,
            emails TEXT NOT NULL,
            phones TEXT NOT NULL,
            websites TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare("SELECT * FROM contacts")?;
    let contacts_iter = stmt.query_map(params![], |row| {
        Ok(db::Conatct {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
            first_name: row.get(3)?,
            last_name: row.get(4)?,
            notes: row.get(5)?,
            addresses: row.get(6)?,
            birthday: row.get(7)?,
            organizations: row.get(8)?,
            emails: row.get(9)?,
            phones: row.get(10)?,
            websites: row.get(11)?,
        })
    })?;
    let contacts: Result<Vec<db::Conatct>, _> = contacts_iter.collect();
    let contacts = contacts?;

    let ret: Message = contacts::GuiContacts { contacts }.into();

    return Ok(ret);
}
