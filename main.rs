use std::io::Write;
use ldtstore_codegen::codegen::{codegen, CodegenResult};

fn main() {
    let CodegenResult { home, tools, tools_plain } = codegen(r"D:\root\repo\public\ldtstore-homepage");
    let mut f = std::fs::File::create(r"C:\swap\codegen-test").unwrap();

    macro_rules! out {
        ($s:expr) => {
            writeln!(f, stringify!($s)).unwrap();
            for (k, v) in $s {
                writeln!(f, "{}", k).unwrap();
                writeln!(f, "{}", v).unwrap();
            }
        };
    }
    
    out!(home);
    out!(tools);
    out!(tools_plain);
}
