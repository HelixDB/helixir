use std::path::Path;

pub fn check_helix_init() -> bool {
    Path::new("helixdb-cfg").exists()
}
