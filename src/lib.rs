// /home/bjorn/.local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous
mod models;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::models::journal_dir::EDLogDir;

    #[test]
    fn sandbox() {
        let dir_path = PathBuf::from("/home/bjorn/.local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous");
        let ed_log_dir = EDLogDir::try_from(dir_path)
            .unwrap();
    }
}
