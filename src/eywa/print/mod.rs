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

        ///
        /// # Print a file on a selected printer
        ///
        /// - `printer`     The printer to use
        /// - `filename`    The filename to print
        ///
        pub fn print_at(self, printer: &Printer, filename: &str) -> Job {
            printer.print_file(filename)
        }

        ///
        /// # print a file on the first printer
        ///
        /// - `filename` The filename to print
        ///
        pub fn print(self, filename: &str) -> Job {
            let p = self.printers.first().expect("failed to find printer");
            printers::print_file(p, filename)
        }
    }
}
