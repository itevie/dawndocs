use std::{collections::HashMap, path::PathBuf, process::Command};

use walkdir::WalkDir;

fn main() {
    let root = std::env::current_dir().expect("Failed to get current directory");

    let mut gen_folder = root.clone();
    gen_folder.push("./gen");

    debug(format!(
        "Generation folder is {}",
        gen_folder.canonicalize().unwrap().display()
    ));

    if gen_folder.exists() {
        debug("Deleting gen folder");
        std::fs::remove_dir_all(gen_folder.clone()).expect("Failed to delete gen folder");
    }

    debug("Creating gen folder");
    std::fs::create_dir(gen_folder.clone()).expect("Failed to create gen folder");

    let places = HashMap::from([
        ("package.json", include_str!("./resources/package.json")),
        ("vite.config.js", include_str!("./resources/vite.config.js")),
        ("index.html", include_str!("./resources/index.html")),
        (
            "eslint.config.js",
            include_str!("./resources/eslint.config.js"),
        ),
        (".gitignore", include_str!("./resources/.gitignore")),
    ]);

    for (key, value) in places {
        let mut location = gen_folder.clone();
        location.push(format!("./{}", key));
        debug(format!("Writing {} into {}", key, location.display()));
        std::fs::write(location, value).expect("Failed to write!");
    }

    debug("Generating src folder");
    let mut src = gen_folder.clone();
    src.push("./src");
    std::fs::create_dir(src.clone()).expect("Failed to create src folder");

    debug("Creating pages folder");
    let mut pages = src.clone();
    pages.push("./pages");
    std::fs::create_dir(pages.clone()).expect("Failed to create pages folder");

    let mut md_files_path = root.clone();
    md_files_path.push("./docs/md");
    let md_files = load_md_files(md_files_path);
    debug(format!("Found {} markdown files", md_files.len()));

    let mut imports: HashMap<String, String> = HashMap::new();

    let mut id: u8 = 0;
    for (file_name, contents) in md_files {
        let name = format!("Gen_{}", id);
        let mut react_md_file_path = pages.clone();
        react_md_file_path.push(format!("./{}.jsx", name));
        debug(format!(
            "Generating {} as {} at {}",
            file_name,
            name,
            react_md_file_path.display()
        ));
        let finished_contents = make_react_md_page(file_name.clone(), contents, id);
        std::fs::write(react_md_file_path, finished_contents).expect("Failed to write");
        imports.insert(
            PathBuf::from(file_name)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
                .replace(".md", ""),
            name,
        );
        id += 1;
    }

    debug(format!("Creating main.jsx file"));
    let mut main_file = src.clone();
    main_file.push("./main.jsx");
    let main_jsx = include_str!("./resources/main.jsx")
        .replace(
            "$imports",
            &imports
                .iter()
                .map(|e| format!("import {} from \"./pages/{}.jsx\";", e.1, e.1))
                .collect::<String>(),
        )
        .replace(
            "$routes",
            &imports
                .iter()
                .map(|e| {
                    format!(
                        "{{path: \"{}\", element: <{} />}},",
                        get_actual_md_route_path(e.0),
                        e.1
                    )
                })
                .collect::<String>(),
        )
        .replace(
            "$sidebar_links",
            &imports
                .iter()
                .map(|e| {
                    format!(
                        "<Link noHighlight href=\"/{}\">{}</Link>",
                        get_actual_md_route_path(e.0),
                        get_actual_md_route_path(e.0)
                    )
                })
                .collect::<String>(),
        );
    std::fs::write(main_file, main_jsx).expect("Failed to create main file");

    debug("Cloning dawn-ui");
    let status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/itevie/dawn-ui")
        .current_dir(src.clone())
        .status()
        .expect("Failed to clone dawn-ui");
    if !status.success() {
        panic!("Failde to run git clone! Status: {}", status);
    }

    debug("Running pnpm install");
    let status = Command::new("pnpm")
        .arg("install")
        .current_dir(gen_folder.clone())
        .status()
        .expect("Failed to run pnpm install");
    if !status.success() {
        panic!("Failed to run pnpm install! Status: {}", status);
    }

    debug("Running pnpm run build");

    let status = Command::new("pnpm")
        .arg("run")
        .arg("dev")
        .current_dir(gen_folder)
        .status()
        .expect("Failed to run pnpm install");
    if !status.success() {
        panic!("Failed to run pnpm install! Status: {}", status);
    }
}

fn debug<T: Into<String>>(contents: T) {
    println!("{}", contents.into());
}

fn get_actual_md_route_path<T: Into<String>>(path: T) -> String {
    match path.into().as_str() {
        "_home" => "/".to_string(),
        x => x.to_string(),
    }
}

fn load_md_files(path: PathBuf) -> Vec<(String, String)> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter(|e| matches!(e, Result::Ok(_)))
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().extension().unwrap() == "md")
    {
        let path = entry.path();
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Some(name) = path.to_str() {
                files.push((name.to_string(), contents))
            }
        }
    }

    files
}

fn make_react_md_page<T: Into<String>>(file_name: T, contents: T, id: u8) -> String {
    return include_str!("./resources/pageBase.jsx")
        .replace("$title", &format!("Gen_{}", id))
        .replace("$file_name", &file_name.into())
        .replace(
            "$contents",
            &contents.into().replace("\n", "  \n").replace("`", "\\`"),
        );
}
