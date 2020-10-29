use std::env;

enum Task {
    Help, 
    Edit,
    Sync,
    UsageError,
    Clone(String), 
}

const parse_args() -> Task {
    match env::args().nth(1) {
        Some(flag) => {
            if flag == "-s" || flag == "--sync" {
                return Task::Sync,
            } else if flag == "-c" || flag == "--clone" {
                return Task::Clone,
            } else if flag == "-h" || flag == "--help" {
                return Task::Help,
            } else {
                return Task::UsageError,
            }
        }
        None => Task::Edit,
    }
}
