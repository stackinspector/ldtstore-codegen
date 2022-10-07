use std::{str::FromStr, fmt::Display, error::Error, fs::{self, OpenOptions}, path::{Path, PathBuf}, io::Write};
use concat_string::concat_string as cs;
use ldtstore_codegen::{codegen::{codegen, CodegenResult}, Inserts};

macro_rules! assert_none {
    ($x:expr) => {
        assert!(matches!($x, None))
    };
}

macro_rules! load {
    ($p:expr) => {
        fs::read_to_string($p).unwrap()
    };
}

macro_rules! byte2str {
    ($b:expr) => {
        std::str::from_utf8($b).unwrap()
    };
}

#[allow(non_snake_case)]
struct TemplatePaths<'a> {
    IMAGE: &'a str,
    MIRROR: &'a str,
}

#[derive(Clone, Copy)]
enum Config {
    Default,
    Intl,
    Test,
}

// region: impl FromStr

impl FromStr for Config {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Config::Default,
            "intl" => Config::Intl,
            "test" => Config::Test,
            _ => return Err(ParseEnumError)
        })
    }
}

#[derive(Debug)]
struct ParseEnumError;

impl Display for ParseEnumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing enum")
    }
}

impl Error for ParseEnumError {}

// endregion

#[derive(Clone, Copy)]
enum FileType {
    Html,
    Css,
    Script,
}

impl FileType {
    fn as_src(&self) -> &'static str {
        match self {
            FileType::Html => "html",
            FileType::Css => "css",
            FileType::Script => "ts",
        }
    }

    fn as_dest(&self) -> &'static str {
        match self {
            FileType::Html => "html",
            FileType::Css => "css",
            FileType::Script => "js",
        }
    }

    fn comment(&self) -> (&'static str, &'static str) {
        match self {
            FileType::Html => ("<!--", "-->"),
            FileType::Css | FileType::Script => ("/*", "*/"),
        }
    }
}

const fn template_paths(config: Config) -> TemplatePaths<'static> {
    match config {
        Config::Default => TemplatePaths {
            IMAGE: "//s0.ldtstore.com.cn",
            MIRROR: "//d1.ldtstore.com.cn",
        },
        Config::Intl => TemplatePaths {
            IMAGE: "//fastly.jsdelivr.net/gh/stackinspector/ldtstore-assert@latest/image",
            MIRROR: "//d1.ldtstore.net",
        },
        Config::Test => TemplatePaths {
            IMAGE: "/image",
            MIRROR: "//d1.ldtstore.net",
        },
    }
}

const COPYRIGHT: &str = "
  Copyright (c) 2021-2022 CarrotGeball and stackinspector. All rights reserved. MIT license.
  Source code: https://github.com/stackinspector/ldtstore-homepage
  Commit: ";

fn read_commit<P: AsRef<Path>>(base_path: P) -> [u8; 7] {
    let base_path = base_path.as_ref();
    let head = load!(base_path.join(".git/HEAD"));
    let head = head.split('\n').next().unwrap();
    let head = head.split("ref: ").nth(1).unwrap();
    let commit = fs::read(base_path.join(".git").join(head)).unwrap();
    commit[0..7].try_into().unwrap()
}

fn build_static_inserts<P: AsRef<Path>>(base_path: P, config: Config, commit: &str) -> Inserts {
    let base_path = base_path.as_ref();
    let mut res = Inserts::new();
    assert_none!(res.insert(
        r#"<script src="/main.js"></script>"#.to_owned(),
        cs!(r#"<script src="/main-"#, commit, r#".js"></script>"#),
    ));
    assert_none!(res.insert(
        r#"<link rel="stylesheet" href="/style.css">"#.to_owned(),
        cs!(r#"<link rel="stylesheet" href="/style-"#, commit, r#".css">"#),
    ));
    assert_none!(res.insert(
        "<a n ".to_owned(),
        r#"<a target="_blank" "#.to_owned(),
    ));
    for entry in fs::read_dir(base_path.join("fragment")).unwrap() {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            assert_none!(res.insert(
                cs!("<!--{{", entry.file_name().to_str().unwrap(), "}}-->"),
                load!(entry.path()),
            ));
        }
    }
    assert_none!(res.insert(
        "<!--{{footer}}-->".to_owned(),
        load!(base_path.join("fragment").join(if matches!(config, Config::Intl) { "footer-intl.html" } else { "footer.html" })),
    ));
    assert_none!(res.insert(
        "/*{{minified:plain.css}}*/".to_owned(),
        minify_css(base_path.join("fragment").join("plain.css")),
    ));
    res
}

fn insert(mut input: String, inserts: &Inserts) -> String {
    for (k, v) in inserts {
        input = input.replace(k, v);
    }
    input
}

fn minify_css<P: AsRef<Path>>(full_path: P) -> String {
    use parcel_css::stylesheet::{StyleSheet, ParserOptions, PrinterOptions};
    let full_path = full_path.as_ref();
    let code = load!(full_path);
    let mut parsed = StyleSheet::parse(&code, ParserOptions { ..Default::default() }).unwrap();
    parsed.minify(Default::default()).unwrap();
    let res = parsed.to_css(PrinterOptions { minify: true, ..Default::default() }).unwrap();
    res.code
}

fn compile_script<P: AsRef<Path>>(full_path: P) -> String {
    use std::process::{Command, Stdio, Output};
    let Output { status, mut stdout, .. } = Command::new("esbuild")
        .arg(full_path.as_ref())
        .arg("--minify-whitespace")
        .arg("--minify-syntax")
        .arg("--target=es6")
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .output().unwrap();
    assert!(status.success());
    assert_eq!(stdout.pop().unwrap(), b'\n');
    cs!("(function(){", byte2str!(&stdout), "})()\n")
}

struct GlobalStates {
    base_path: PathBuf,
    dest_path: PathBuf,
    template: TemplatePaths<'static>,
    commit: [u8; 7],
    static_inserts: Inserts,
    codegen_result: CodegenResult,
}

impl GlobalStates {
    fn init(Args { base_path, dest_path, config }: Args) -> GlobalStates {
        let template = template_paths(config);
        let commit = read_commit(&base_path);
        let codegen_result = codegen(&base_path);
        let static_inserts = build_static_inserts(&base_path, config, byte2str!(&commit));
        GlobalStates { base_path, dest_path, template, commit, static_inserts, codegen_result }
    }

    fn emit(&self, name: &str, ty: FileType) {
        let GlobalStates { base_path, dest_path, template, commit, static_inserts, codegen_result } = self;

        let src_path = base_path.join(cs!(name, ".", ty.as_src()));
        let content = match ty {
            FileType::Html => {
                let src = load!(src_path);
                let static_templated = insert(src, static_inserts);
                let dynamic_templated = insert(static_templated, match name {
                    "index" => &codegen_result.home,
                    "ldtools/index" => &codegen_result.tools,
                    "ldtools/plain" => &codegen_result.tools_plain,
                    _ => unreachable!(),
                });
                dynamic_templated
            }
            FileType::Css => {
                minify_css(src_path)
            }
            FileType::Script => {
                compile_script(src_path)
            },
        };

        // macro_rules! replace_impl {
        //     ($($kw:ident),+) => {$(
        //         let content = content.replace(concat!("{{", stringify!($kw), "}}"), template.$kw);
        //     )+};
        // }
        // replace_impl!(IMAGE, MIRROR);
        let content = content.replace("{{IMAGE}}", template.IMAGE);
        let content = content.replace("{{MIRROR}}", template.MIRROR);

        let dest_path = dest_path.join(if matches!(ty, FileType::Html) {
            cs!(name, ".", ty.as_dest())
        } else {
            cs!(name, "-", byte2str!(commit), ".", ty.as_dest())
        });
        let (comment_l, comment_r) = ty.comment();
        let mut file = OpenOptions::new().create_new(true).write(true).open(dest_path).unwrap();
        macro_rules! w {
            ($s:expr) => {
                file.write_all($s.as_bytes()).unwrap();
            };
        }
        macro_rules! wb {
            ($s:expr) => {
                file.write_all($s).unwrap();
            };
        }
        w!(comment_l);
        w!(COPYRIGHT);
        wb!(commit);
        w!("\n");
        w!(comment_r);
        w!("\n\n");
        w!(content);
    }
}

/// !
#[derive(argh::FromArgs)]
struct Args {
    /// dest wwwroot path
    #[argh(option, short = 'd')]
    dest_path: PathBuf,
    /// dest profile
    #[argh(option, short = 'c')]
    config: Config,
    /// source path (default .)
    #[argh(option, short = 's', default = "Default::default()")]
    base_path: PathBuf,
}

fn main() {
    let args: Args = argh::from_env();
    fs::create_dir_all(&args.dest_path).unwrap();
    fs::create_dir_all(args.dest_path.join("ldtools")).unwrap();
    fs::copy(args.base_path.join("robots.txt"), args.dest_path.join("robots.txt")).unwrap();
    fs::copy(args.base_path.join("error.html"), args.dest_path.join("error.html")).unwrap();

    let builder = GlobalStates::init(args);
    builder.emit("index", FileType::Html);
    builder.emit("ldtools/index", FileType::Html);
    builder.emit("ldtools/plain", FileType::Html);
    builder.emit("style", FileType::Css);
    builder.emit("main", FileType::Script);
}
