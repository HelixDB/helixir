use std::path::Path;

pub fn check_helix_init() -> bool {
    Path::new("db/schema.hx").exists() && Path::new("db/queries.hx").exists()
}
