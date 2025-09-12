use std::path::Path;

pub fn check_helix_init() -> bool {
    Path::new("schema.hx").exists() && Path::new("queries.hx").exists()
}
