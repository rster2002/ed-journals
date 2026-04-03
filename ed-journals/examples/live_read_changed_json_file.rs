use ed_journals::fs::common::{ChangedJsonFile, JsonFile};
use ed_journals::fs::SyncBlocker;
use ed_journals::status::Status;

fn main() {
    let path = "./test-files/json/StatusNone.json";

    // Main blocker that will be used to block the thread until something changes.
    let mut sync_blocker = SyncBlocker::new();

    // Open the path using the `JsonFile` type and provide the blocker.
    let mut json_file = ChangedJsonFile::<Status>::new(path, &sync_blocker).unwrap();

    loop {
        let Some(contents) = json_file.content().unwrap() else {
            // `None` is returned if the file is empty, or if the contents of the file haven't
            // changed since the last time it was read.
            sync_blocker.wait().unwrap();
            continue;
        };

        /// This is guaranteed to be different from the last loop.
        println!("{:?}", contents);

        // Wait for new content.
        sync_blocker.wait().unwrap();

        break; // You'll need to handle breaking out of the loop
    }
}