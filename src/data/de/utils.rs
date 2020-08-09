pub fn default_var_sep() -> String {
    if cfg!(windows) { ";" } else { ":" }.to_owned()
}
