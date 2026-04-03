use ed_journals::fs::common::JsonFile;
use ed_journals::fs::SyncBlocker;
use ed_journals::status::Status;

fn main() {
    let path = "./test-files/json/StatusNone.json";

    // Main blocker that will be used to block the thread until something changes.
    let mut sync_blocker = SyncBlocker::new();

    // Open the path using the `JsonFile` type and provide the blocker.
    let json_file = JsonFile::<Status>::new(path, &sync_blocker).unwrap();

    loop {
        let Some(contents) = json_file.content().unwrap() else {
            // Wait for any update.
            sync_blocker.wait().unwrap();
            continue;
        };

        /// The contents will always just be the current content and doesn't necessarily need to be
        /// different from the last loop. For that you can use the `ChangedJsonFile` type.
        println!("{:?}", contents);

        // Wait for new content.
        sync_blocker.wait().unwrap();

        break; // You'll need to handle breaking out of the loop
    }
}