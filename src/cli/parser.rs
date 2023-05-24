pub fn parse_args<T: Iterator<Item = String>>(
    mut args: T,
) -> Result<(String, String), &'static str> {
    args.next();
    let query = match args.next() {
        Some(query) => query,
        None => return Err("Couldn't get query from args"),
    };
    let filename = match args.next() {
        Some(filename) => filename,
        None => return Err("Couldn't get filename from args"),
    };
    Ok((query, filename))
}
