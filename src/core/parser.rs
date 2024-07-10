use std::collections::HashMap;

// Parses the options from the command line arguments and returns a tuple with the options and the content.
pub fn parse_options<'a>(args: &'a [String]) -> (HashMap<String, String>, Vec<&'a str>) {
    let mut options = HashMap::new();
    let mut content = Vec::new();
    let mut i = 0;

    while i < args.len() {
        if args[i].starts_with("--") {
            let key = &args[i][2..];
            i += 1;
            if i < args.len() {
                options.insert(key.to_string(), args[i].to_string());
            }
        } else {
            content.push(args[i].as_str());
        }
        i += 1;
    }

    (options, content)
}
