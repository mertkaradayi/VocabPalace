use std::path::PathBuf;

pub struct IBooksPaths {
    pub library_path: PathBuf,
    pub highlights_path: PathBuf,
}

pub fn get_ibooks_paths() -> Option<IBooksPaths> {
    let home = dirs::home_dir()?;
    let container_path = home.join("Library/Containers/com.apple.iBooksX/Data/Documents");

    let library_path = container_path
        .join("BKLibrary")
        .join("BKLibrary-1-091020131601.sqlite");

    let highlights_path = container_path
        .join("AEAnnotation")
        .join("AEAnnotation_v10312011_1727_local.sqlite");

    Some(IBooksPaths {
        library_path,
        highlights_path,
    })
}
