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
    extern crate peg;

    pub fn gen() {
        let generated = env::var("OUT_DIR").unwrap() + "/parser.rs";
        let dst = "src/peg.rs";
        peg::cargo_build("parser.rustpeg");
        std::fs::copy(&generated, &dst)
            .expect(&format!("Could not copy `{}` to `{}`", &generated, &dst));
    }
}


fn main() { codegen::gen(); }
