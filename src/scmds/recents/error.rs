use std::io;
use std::path::PathBuf;
use git2;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ReactorInit(err: io::Error) {
            description("Could not initialize tokio event loop in worker thread")
            cause(err)
        }
        Timeout(err: io::Error) {
            description("A timeout could not be created or failed while being invoked")
            cause(err)
        }
        RepositoryDirectory(err: io::Error, path: PathBuf) {
            display("Could not create directory to contain crates.io repository at '{}'",
                     path.display())
            cause(err)
        }
        Git2(err: git2::Error) {
            description("A git operation failed")
            from()
            cause(err)
        }
    }
}
