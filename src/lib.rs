// /home/bjorn/.local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous
#[macro_use]
mod models;
mod macros;

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::models::journal_dir::EDLogDir;
    use std::path::PathBuf;

    #[test]
    fn sandbox() {
        // let dir_path = PathBuf::from("/home/bjorn/GitHub/ed-logs/journals");
        let dir_path = current_dir()
            .unwrap()
            .join("journals");

        let log_dir = EDLogDir::try_from(dir_path).unwrap();

        for journal in log_dir.journal_logs().unwrap() {
            let mut reader = journal.create_reader().unwrap();

            for entry in reader {
                // dbg!(&entry);
                entry.unwrap();

                // if !entry.is_err() {
                //     dbg!(&entry);
                // }
                //
                // assert!(entry.is_ok());
            }
        }

        // let mut reader = EDLogDir::try_from(dir_path)
        //     .unwrap()
        //     .journal_logs()
        //     .unwrap()
        //     .get(0)
        //     .unwrap()
        //     .create_reader()
        //     .unwrap();
        //
        // while let Some(entry) = reader.next() {
        //     dbg!(entry);
        // }
    }
}
