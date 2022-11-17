use chadscript::run;
use std::path::Path;

fn main() {
	let source_path = Path::new("index.ts");
	run(source_path).unwrap();
}
