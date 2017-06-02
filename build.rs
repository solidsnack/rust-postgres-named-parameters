#[cfg(not(feature = "codegen"))]
mod codegen {
    pub fn gen() {
        println!("Skipping parser generation.");
    }
}

#[cfg(feature = "codegen")]
mod codegen {
    use std;
    use std::env;
    use std::io::Write;
    extern crate peg;

    pub fn gen() {
        gen_parser();
        gen_keywords();
    }

    fn gen_parser() {
        let generated = env::var("OUT_DIR").unwrap() + "/parser.rs";
        let dst = "src/peg.rs";
        peg::cargo_build("parser.rustpeg");
        std::fs::copy(&generated, &dst)
            .expect(&format!("Could not copy `{}` to `{}`", &generated, &dst));
    }

    fn gen_keywords() {
        let keywords: &str = include_str!("kwlist.h");
        let generated = env::var("OUT_DIR").unwrap() + "/kwlist.txt";
        let dst = "src/kwlist";
        let mut f = std::fs::File::create(&generated)
            .expect(&format!("Could not open `{}`.", &generated));
        for line in keywords.lines() {
            let words: Vec<&str> =
                line.splitn(8, |c| "\" ()".contains(c)).collect();
            if !(words.contains(&"PG_KEYWORD") && words[0] == "PG_KEYWORD") ||
               words.contains(&"UNRESERVED_KEYWORD") {
                continue;
            }
            f.write_all(words[2].as_bytes())
             .expect(&format!("Could not write to `{}`.", &generated));
            f.write_all("\n".as_bytes())
             .expect(&format!("Could not write to `{}`.", &generated));
            // f.write_all(format!("{:?}\n", words).as_bytes())
            //  .expect(&format!("Could not write to `{}`.", &generated));
        }
        f.sync_all()
         .expect(&format!("Could not write to `{}`.", &generated));
        std::fs::copy(&generated, &dst)
            .expect(&format!("Could not copy `{}` to `{}`", &generated, &dst));
    }
}


fn main() { codegen::gen(); }
