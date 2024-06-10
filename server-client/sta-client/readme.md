# sta-client

This crate handles the client side actions for server-client app

## File handling functions:

dir_exists(path: &str) -> io::Result<bool>
is_writable(path: &str) -> io::Result<bool>
create_dir(path: &str) -> io::Result<()>
is_valid_file(path: &str) -> io::Result<bool>

## Client handling functions:

strip_to_second_space(cow: Cow<str>) -> Cow<str>
parse_command() -> Option<(String, String)>
