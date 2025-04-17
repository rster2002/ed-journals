use std::path::PathBuf;
use std::time::Duration;
use async_fs::File;
use futures::io::BufWriter;
// use tokio::fs;
// use tokio::fs::File;
use crate::modules::tests::FileSimulator;
use crate::tests::{test_dir, test_root};
// const FILES: &[(&str, &str)] = &[
//     ("Journal.2000-01-01T100000.01.log", include_str!("../fixtures/Journal.2000-01-01T100000.01.log")),
//     ("Journal.2022-11-01T182557.01.log", include_str!("../fixtures/Journal.2022-11-01T182557.01.log")),
//     ("Journal.2023-01-10T133420.01.log", include_str!("../fixtures/Journal.2023-01-10T133420.01.log")),
// ];

pub async fn simulate_log_file(name: &str) -> (PathBuf, FileSimulator<BufWriter<File>>) {
    let dir_root = test_root().join(name).to_path_buf();
    async_fs::create_dir_all(&dir_root).await.unwrap();



    todo!()

    // for (filename, contents) in FILES {
    //     #[cfg(debug_assertions)]
    //     println!("Starting {}", filename);
    //
    //
    // }
    //
    // sleep(Duration::from_millis(10)).await;
}
