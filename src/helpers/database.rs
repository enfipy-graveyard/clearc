use crate::config;

use r2d2_postgres::{r2d2::Pool, PostgresConnectionManager, TlsMode};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

pub type Database = Pool<PostgresConnectionManager>;

pub fn init_pool(cnfg: &config::Config, retry_count: i16) -> Result<Database, Box<Error>> {
    let conn_string: &str = &cnfg.db_connection;

    let delay = Duration::from_secs(1);
    let mut try_num: i16 = 1;

    loop {
        match PostgresConnectionManager::new(conn_string, TlsMode::None) {
            Ok(conn) => {
                println!("Connected to database on {} try", try_num);
                let pool = Pool::new(conn)?;
                return Ok(pool);
            }
            Err(err) => {
                println!("Connection failed on {} retry with error: {}", try_num, err);
                try_num += 1;
                if try_num > retry_count {
                    return Err(err.into());
                }
                sleep(delay);
            }
        }
    }
}
