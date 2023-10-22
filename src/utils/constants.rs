pub mod api {
    pub const APIROOT: &str = "http://localhost:8080";

    pub fn apiroot(path: &str)-> String {
        format!("{}/{}", APIROOT, path)
    }
}

pub mod ui {
    pub const INPUT_MAX_COUNT: usize = 8;
}