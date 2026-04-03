use ed_journals::fs::{DirWatcher, LogDir, SyncBlocker};
use ed_journals::fs::common::{ChangedJsonFile, NewestFile};
use ed_journals::status::Status;

fn main() {
    let mut dir = LogDir::new("./test-files/journals");

    // Main blocker that will be used to block the thread until something changes.
    let mut sync_blocker = SyncBlocker::new();

    // Watch the directory for changes. Keep in mind that this will need to be kept alive to
    // actually watch the directory, even if it's not directly used.
    let _dir_watcher = DirWatcher::new(&dir, &sync_blocker).unwrap();

    // Container that holds the newest file.
    let mut newest_file = NewestFile::new(&sync_blocker);

    // Open the JSON file.
    let mut status_json = ChangedJsonFile::<Status>::new("../test-files/json/StatusNone.json", &sync_blocker).unwrap();

    loop {
        // Check if the JSON file has changed.
        if let Some(contents) = status_json.content().unwrap() {
            /// Print the contents of the JSON file when the contents of it has changed.
            println!("{:?}", contents);
        }

        // Read the last file in the directory.
        if let Some(last) = dir.skip_to_last() {
            // Calling the `maybe_next` method will check the provided path against the path that is
            // currently held by the newest file. If the path is newer than the current one, then
            // the provided path is set and opened.
            newest_file.maybe_next(&last.unwrap()).unwrap();

            // Read any events from the newest file. Calling `maybe_next` won't reset the inner
            // iterator, so you can keep reading events from the same file or immediately continue in
            // case all events have been read.
            for event in newest_file {
                println!("{:?}", event.unwrap());
            }
        }

        // Block after reading all currently available events from the newest file. This is
        // unblocked when either something changes in the directory or the content of the current
        // newest file changes.
        sync_blocker.wait().unwrap();

        break; // You'll need to handle breaking out of the loop
    }
}