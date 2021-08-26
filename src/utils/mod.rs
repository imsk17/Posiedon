use std::path::Path;

pub fn db_exists() -> bool {
    Path::new("./db/IDENTITY").exists()
}
