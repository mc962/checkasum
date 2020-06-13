use orbtk::prelude::*;

mod gui {
    pub mod main {
        pub mod view;
        pub mod state;
    }
    // pub mod input_view;
    // pub mod output_view;
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Checkasum")
                .position((100.0, 100.0))
                .size(600.0, 700.0)
                .resizeable(true)
                .child(gui::main::view::MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}

// use std::process::exit;
// // StructOpt import is required here for using its from_args method with Options
// use structopt::StructOpt;
//
// pub mod hashing;
// use hashing::{hash_file, hash_matches};
//
// mod cli;
// use cli::{Options, algorithm_type};
//
// fn main() {
//     let args = Options::from_args();
//     let algorithm = match algorithm_type(&args.method) {
//         Ok(algorithm) => algorithm,
//         Err(reason) => {
//             println!("Problem determining hashing algorithm: {}", reason);
//             exit(1);
//         }
//     };
//
//     let hash_str = hash_file(algorithm, &args.path);
//     match hash_str {
//         Ok(hash) => {
//             println!("{}   {}", hash, args.path.display());
//             if hash_matches(&hash, &args.expected) {
//                 println!("SUCCESS: file hash matches expected checksum hash!")
//             } else {
//                 println!("WARNING: File hash does not match expected checksum hash!");
//                 println!("Expected: {} | Actual: {}", args.expected, hash);
//             }
//         },
//         Err(reason) => {
//             println!("Problem hashing file: {}", reason);
//             exit(1);
//         }
//     }
// }
