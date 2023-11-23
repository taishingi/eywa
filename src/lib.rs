pub mod eywa {
    use std::fs::ReadDir;
    use std::io::BufRead;
    use std::path::Path;
    use std::{fs::File, io::BufReader};
    pub mod print;
    pub const FILES: &i32 = &0;
    pub const DIRECTORIES: &i32 = &1;

    ///
    /// # Create a new file
    ///
    /// Create a new file only if not exist
    ///
    /// - `name`    The name of the filename to create
    ///
    #[macro_export]
    macro_rules! touch {
        ($name:expr) => {
            if std::path::Path::new($name).is_file() {
                panic!("{}", format!("The {} file already exist", $name));
            } else {
                std::fs::File::create($name)
                    .expect(format!("Failed to create the {} file", $name).as_str());
            }
        };
    }

    #[macro_export]
    macro_rules! touch_if_not_exists {
        ($name:expr) => {
            !if std::path::Path::new($name).is_file() {
                std::fs::File::create($name)
                    .expect(format!("Failed to create the {} file", $name).as_str());
            }
        };
    }
    #[macro_export]
    macro_rules! mkdir_if_not_exists {
        ($name:expr) => {
            !if std::path::Path::new($name).is_file() {
                std::fs::create_dir($name)
                    .expect(format!("Failed to create the {} directory", $name).as_str());
            }
        };
    }

    ///
    /// # Create a new file
    ///
    /// Create a new file only if not exist
    ///
    /// - `name`    The name of the filename to create
    ///
    #[macro_export]
    macro_rules! display_help {
        ($instance:expr) => {
            $instance.help();
        };
    }

    ///
    /// # Create a new file with content
    ///
    /// Create new file with content with content
    ///
    /// - `name`        The name of the filename to create
    /// - `content`     The file content to create
    ///
    #[macro_export]
    macro_rules! touch_with_content {
        ($name:expr,$content:expr) => {
            if std::path::Path::new($name).is_file() {
                panic!("{}", format!("The {} file already exist", $name));
            } else {
                use std::io::Write;
                let mut x = std::fs::File::create($name)
                    .expect(format!("Failed to create the {} file", $name).as_str());
                x.write_all($content.as_bytes())
                    .expect(format!("Failed add the content in the {} file", $name).as_str());
            }
        };
    }

    ///
    /// # Print a file
    ///
    /// - `file`                    The name of the filename to print
    /// - `success_callback`        The callaback to call on success
    /// - `failure_callback`        The callaback to call on failure
    ///
    #[macro_export]
    macro_rules! print {
        ($file:expr,$success_callback:expr,$failure_callback:expr) => {
            match Impress::default().print($file).status {
                JobStatus::SUCCESS => $success_callback($file),
                JobStatus::FAILED => failure_callback($file),
            }
        };
    }

    ///
    /// # Print a file to at printer
    ///
    /// - `file`                    The name of the filename to print
    /// - `printer`                 The printer to use
    /// - `success_callback`        The callaback to call on success
    /// - `failure_callback`        The callaback to call on failure
    ///
    #[macro_export]
    macro_rules! print_at {
        ($file:expr,$printer:expr,$success_callback:expr,$failure_callback:expr) => {
            match Impress::default().print_at($printer, $file).status {
                JobStatus::SUCCESS => $success_callback($file),
                JobStatus::FAILED => failure_callback($file),
            }
        };
    }
    ///
    /// # Remove an exiting file
    ///
    /// - `filename`    The path of the filename to remove
    ///
    #[macro_export]
    macro_rules! rm {
        ($filename:expr) => {
            if std::path::Path::new($filename).is_file() {
                std::fs::remove_file($filename)
                    .expect(format!("Failed to remove the {} file", $filename).as_str());
            } else {
                panic!("{}", format!("The {} file not already exist", $filename));
            }
        };
    }

    ///
    /// # Get the file contents
    ///
    /// - `name`    The name of the filename
    ///
    #[macro_export]
    macro_rules! get_file_contents {
        ($name:expr) => {
            use std::io::Read;
            if std::path::Path::new($name).is_file() {
                std::fs::read_to_string($name)
                    .expect(format!("Failed to get the {} file content", $name).as_str())
            } else {
                panic!("{}", format!("The {} file don't exist", $name));
            }
        };
    }

    ///
    /// # Get the directory contents
    ///
    /// Get all contents inside the directory
    /// will matches the pattern
    ///
    /// - `name`    The name of the directory
    ///
    #[macro_export]
    macro_rules! directory_contents {
        ($directory:expr,$pattern:expr) => {
            use glob::glob;
            use std::ops::Add;
            let mut x = Vec::new();
            let p = String::from($directory).add("/").add($pattern);
            if std::path::Path::new($directory).is_dir() {
                for path in glob(p.as_str()).unwrap().filter_map(Result::ok) {
                    x.push(path.display().to_string());
                }
                x
            } else {
                panic!("{}", format!("The {} directory don't exist", $directory));
            }
        };
    }

    ///
    /// # Get all directories and filenames in the directory
    ///
    /// Parse a directory and separate directories to files
    ///
    /// - `directory`       The name of the directory
    /// - `length`          The length of the subdirectory
    ///
    #[macro_export]
    macro_rules! get_contents {
        ($directory:expr,$length:expr) => {{
            use glob::glob;
            use std::collections::HashMap;
            use std::ops::Add;
            let mut files: Vec<String> = Vec::new();
            let mut directories: Vec<String> = Vec::new();
            let mut data: HashMap<i32, Vec<String>> = HashMap::new();
            let p = String::from($directory).add("/*".repeat($length).as_str());
            for path in glob(p.as_str()).unwrap().filter_map(Result::ok) {
                if std::path::Path::new(path.display().to_string().as_str()).is_dir() {
                    directories.push(path.display().to_string());
                } else {
                    files.push(path.display().to_string());
                }
            }
            data.insert(0, directories);
            data.insert(1, files);
            data
        }};
    }

    ///
    /// # Create a new directory
    ///
    /// Create a new directory if not exist
    ///
    /// - `name`    The name of the directory to create
    ///
    #[macro_export]
    macro_rules! mkdir {
        ($name:expr) => {
            if std::path::Path::new($name).is_dir() {
                panic!("{}", format!("The {} directory already exist", $name));
            } else {
                std::fs::create_dir($name)
                    .expect(format!("Failed to create the {} directory", $name).as_str());
            }
        };
    }

    ///
    /// # Remove an existing directory
    ///
    /// Remove a directory only if exist
    ///
    /// - `name`    The name of the directory to remove
    ///
    ///
    #[macro_export]
    macro_rules! rmdir {
        ($name:expr) => {
            if std::path::Path::new($name).is_dir() {
                std::fs::remove_dir_all($name)
                    .expect(format!("Failed to remove the {} directory", $name).as_str())
            } else {
                panic!("{}", format!("The {} directory don't exist", $name));
            }
        };
    }

    ///
    /// # Get all directories in a directory
    ///
    /// - `name`    The name of the directory to parse
    ///
    #[macro_export]
    macro_rules! get_dirs {
        ($path:expr) => {
            if !std::path::Path::new($path).is_dir() {
                panic!("{}", format!("The {} directory don't exist", $path));
            } else {
                use glob::glob;
                use std::ops::Add;
                let mut x = Vec::new();
                let d = String::from($path);
                if std::path::Path::new($path).is_dir() {
                    for p in glob(d.add("*").as_str()).unwrap().filter_map(Result::ok) {
                        if std::path::Path::new(p.display().to_string().as_str()).is_dir() {
                            let binding = p.display().to_string();
                            x.push(binding);
                        }
                    }
                    x
                } else {
                    panic!("{}", format!("The {} directory don't exist", $path));
                }
            }
        };
    }

    ///
    /// # Get all files in a directory
    ///
    /// - `name`    The name of the directory to parse
    ///
    #[macro_export]
    macro_rules! get_files {
        ($path:expr) => {
            if !std::path::Path::new($path).is_dir() {
                panic!("{}", format!("The {} directory don't exist", $path));
            } else {
                use glob::glob;
                use std::ops::Add;
                let mut x: Vec<String> = Vec::new();
                let pattern = String::from($path).add("/").add("*");
                for p in glob(pattern.as_str()).unwrap().filter_map(Result::ok) {
                    if std::path::Path::new(p.display().to_string().as_str()).is_file() {
                        x.push(p.display().to_string());
                    }
                }
                x
            }
        };
    }

    ///
    /// # Check if a file exist in a directory
    ///
    /// - `path`    The path to parse
    /// - `file`    The file to find    
    ///
    ///
    #[macro_export]
    macro_rules! file_exists {
        ($path:expr,$file:expr) => {{
            get_files!($path).contains(&$file.to_string())
        }};
    }

    ///
    /// # Check if a directory exist in directories list
    ///
    /// - `path`            The path to parse
    /// - `directory`       The directory to find
    ///
    #[macro_export]
    macro_rules! directory_exists {
        ($path:expr,$directory:expr) => {
            get_dirs!($path).contains(&$directory.to_string())
        };
    }

    ///
    /// # Execute a command in the directory
    ///
    /// - `directory`       The command working directory
    /// - `program`         The program to run
    /// - `args`            The program arguments
    ///  
    #[macro_export]
    macro_rules! shell {
        ($directory: expr,$program:expr,$args:expr,$message_success:expr,$message_failure:expr,$file:expr) => {{
            use colored_truecolor::Colorize;
            use std::fs::remove_file;
            use std::fs::File;
            use std::io;
            use std::io::Write;
            use std::path::Path;
            use std::process::Command;
            println!();
            let output = Command::new($program)
                .current_dir($directory)
                .args($args)
                .spawn()
                .expect("failed to execute the program")
                .wait()
                .expect("there was an error");
            println!();
            match output.success() {
                true => {
                    println!(
                        "[  {}  ] {}",
                        "OK".green().bold(),
                        $message_success.blue().bold()
                    );
                }
                false => {
                    println!(
                        "[  {}  ] {}",
                        "KO".red().bold(),
                        $message_failure.blue().bold()
                    );
                }
            }
            output
        }};
    }

    pub fn get_lines(filename: &str) -> Vec<String> {
        let file: File = File::open(filename).expect("failed to found filename");
        let reader: BufReader<File> = BufReader::new(file);
        let mut r: Vec<String> = Vec::new();
        for line in reader.lines() {
            r.push(line.unwrap());
        }
        r
    }

    pub fn read(directory: &str) -> ReadDir {
        Path::new(directory)
            .read_dir()
            .expect("Failed to read the directory")
    }

    pub fn dir_exist(directory: &str) -> bool {
        assert!(Path::new(directory).exists());
        true
    }

    pub fn file_exist(filename: &str) -> bool {
        assert!(Path::new(filename).is_file());
        true
    }

    pub fn symlink_exist(link: &str) -> bool {
        assert!(Path::new(link).is_symlink());
        true
    }
}

#[cfg(test)]
mod test {
    use crate::{
        directory_contents, directory_exists, file_exists, get_contents, get_dirs, get_files,
        helpful::{DIRECTORIES, FILES},
        mkdir, rm, rmdir, touch, touch_with_content,
    };

    #[test]
    pub fn test_touch() {
        touch!("Doc");
        rm!("Doc");
        touch_with_content!("READ", "readme");
        rm!("READ");
    }

    #[test]
    pub fn test_dir() {
        mkdir!("documentation");
        rmdir!("documentation");
    }

    #[test]
    pub fn test_get_dirs() {
        get_dirs!("/");
        directory_contents!("/usr/include", "*.h");
        assert!(directory_exists!("/", "/home"));
    }

    #[test]
    pub fn test_files() {
        get_files!("/etc");
        assert!(file_exists!("/etc", "/etc/hosts"));
    }

    #[test]
    pub fn test_get_content() {
        get_contents!("/", 2).get(FILES);
        get_contents!("/", 2).get(DIRECTORIES);
    }
}
