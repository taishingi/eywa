pub mod eywa {
    use printers::printer::{Job, Printer};

    pub struct Impress {
        printers: Vec<Printer>,
    }

    impl Default for Impress {
        fn default() -> Impress {
            Impress::new()
        }
    }

    impl Impress {
        pub fn new() -> Impress {
            Self {
                printers: printers::get_printers(),
            }
        }

        pub fn print(self, filename: &str) -> Job {
            let p = self.printers.first().expect("failed to find printer");
            printers::print_file(p, filename)
        }
    }
}
