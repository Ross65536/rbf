use std::collections::HashMap;

fn parse_flags(args: impl Iterator<Item = String>) -> HashMap<String, String> {
    args
      .filter(|arg| arg.starts_with("-") && arg.len() > 1)
      .map(|s| s.to_string())
      .map(|arg| arg.chars().skip(1).collect())
      .map(|arg: String| {
        let mut split = arg.split("=");
        let key = split.next().unwrap();
        let value = match split.next() {
          Some(v) => v,
          None => "",
        };

        (key.to_string(), value.to_string())
      }).collect()
}

pub fn parse(args: impl Iterator<Item = String>) -> (Vec<String>, HashMap<String, String>) {
  let args: Vec<String> = args.collect();
  let commands: Vec<String> = args.iter()
      .filter(|arg| !arg.starts_with("-"))
      .map(|arg| arg.to_string())
      .skip(1)
      .collect();

  let flags = parse_flags(args.into_iter());

  (commands, flags)
}