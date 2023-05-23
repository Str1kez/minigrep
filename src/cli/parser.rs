pub fn parse_args(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() < 3 {
        return Err("Args not enough");
    }
    Ok((&args[1], &args[2]))
}
