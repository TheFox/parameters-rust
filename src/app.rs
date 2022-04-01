
#[derive(Debug)]
pub struct App {
    pub input_file_path: Option<String>,
    pub output_file_path: Option<String>,
    pub regexp: Option<String>,
    pub env_name: Option<String>,
    pub instance: Option<String>,
    pub search: String,
    pub is_quiet: bool,
    pub no_header: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_file_path: None,
            output_file_path: None,
            regexp: Some(String::from("^")),
            env_name: None,
            instance: None,
            search: String::from("@"),
            is_quiet: false,
            no_header: false,
        }
    }
}
