use std::path::PathBuf;

use directories::UserDirs;

/// Attempts to automatically detect where the journal directory is located on your device depending
/// on the current platform. If no path was found, the function will return [None].
///
/// ## Windows
///
/// On Windows it will check if the path
/// `%homedrive%%homepath%\Saved Games\Frontierr Developments\Elite Dangerous` exists and returns
/// the path if is does exist.
///
/// ## Linux
///
/// On Linux it will check the following paths in order, returning the first path that exists:
/// * `$HOME/.local/share/Steam/compatibilitytools.d/Proton 3.16-8 Beta ED/dist/share/default_pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous`
/// * `$HOME/.local/share/Steam/steamapps/common/Elite Dangerous/Products/elite-dangerous-64/Logs/Saved Games/Frontier Developments/Elite Dangerous`
/// * `$HOME/.local/share/Steam/steamapps/common/Proton 4.2/dist/share/default_pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous`
/// * `$HOME/.local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous`
pub fn auto_detect_journal_path() -> Option<PathBuf> {
    #[cfg(target_family = "windows")]
    {
        let expected_path = UserDirs::new()?
            .home_dir()
            .join("Saved Games")
            .join("Frontier Developments")
            .join("Elite Dangerous");

        if !expected_path.exists() {
            return None;
        }

        return Some(expected_path);
    }

    #[cfg(target_family = "unix")]
    {
        let user_dir = UserDirs::new()?;

        return [
            user_dir.home_dir().join(".local/share/Steam/compatibilitytools.d/Proton 3.16-8 Beta ED/dist/share/default_pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous"),
            user_dir.home_dir().join(".local/share/Steam/steamapps/common/Elite Dangerous/Products/elite-dangerous-64/Logs/Saved Games/Frontier Developments/Elite Dangerous"),
            user_dir.home_dir().join(".local/share/Steam/steamapps/common/Proton 4.2/dist/share/default_pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous"),
            user_dir.home_dir().join(".local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous"),
        ]
            .into_iter()
            .find(|path| path.exists());
    }
}
