use crate::journal::JournalEvent;
use crate::state::models::resolvers::journal_state_resolver::JournalStateResolver;
use crate::state::resolvers::live_state_resolver::live_state_entry_owned::LiveStateEntryOwned;
use crate::state::StateContainer;
use std::collections::HashMap;

/// Journal state tracks both log and live events from JSON files. It's important to note, however,
/// that the state built from the JSON files is volatile and is only kept in memory. To save that
/// part of the state, you need to call [JournalStateResolver::all_live_state] and save the contents
/// somewhere. Then, when you create your journal state again, use the [From] trait to create your
/// instance, after which you can feed state like normal again.
///
/// Below is a small example on how to manage this state.
///
/// ```rust,no_run
/// use std::collections::HashMap;
/// use std::fs;
/// use std::path::Path;
/// use ed_journals::io::auto_detect_journal_path;
/// use ed_journals::journal::blocking::LiveJournalDirReader;
/// use ed_journals::state::{JournalState, LiveStateEntryOwned};
///
/// // In this example we will save the live state to a file called 'live_state.json'.
/// let path = Path::new("live_state.json");
///
/// // Check if the file exists and create either create a default state or read the file and use
/// // that instead.
/// let mut state = if path.exists() {
///     // You need to create a HashMap with strings as keys and live states as values like shown
///     // below.
///     let string_contents = fs::read_to_string(&path).unwrap();
///     let parsed: HashMap<String, LiveStateEntryOwned> = serde_json::from_str(&string_contents).unwrap();
///
///     JournalState::from(parsed)
/// } else {
///     // If the path does not exist, the default state is used.
///     JournalState::default()
/// };
///
/// // From here you can use the state how you would normally. In this example a live reader is
/// // used to read and observe changes in the journal directory.
///
/// let journal_path = auto_detect_journal_path().unwrap();
/// let mut journal_reader = LiveJournalDirReader::open(&journal_path).unwrap();
///
/// for entry in journal_reader {
///     let entry = entry.unwrap();
///
///     state.feed(&entry);
///
///     if !entry.is_live {
///         continue;
///     }
///
///     state.flush();
///
///     // In this case we save the state to a file. In this example it's done every 'live' event.
///     // Then on later launches of the application, it will read the contents and use that.
///     let state_contents = serde_json::to_string(&state.all_live_state()).unwrap();
///     fs::write(&path, state_contents).unwrap();
/// }
/// ```
pub type JournalState = StateContainer<JournalStateResolver, JournalEvent>;

impl From<HashMap<String, LiveStateEntryOwned>> for JournalState {
    fn from(value: HashMap<String, LiveStateEntryOwned>) -> Self {
        StateContainer::from(JournalStateResolver::from(value))
    }
}
