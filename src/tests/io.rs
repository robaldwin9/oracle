use std::path::Path;
use crate::{load_list, Error};

#[test]
fn test_load_lists() -> Result<(), Error> {
     let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data");

    for filename in ["compliments.txt", "insults.txt", "answers.txt"] {
        let entries = load_list(&directory.join(filename))?;

        assert!(!entries.is_empty());
        assert!(entries.iter().all(|entry| {
            !entry.is_empty() && entry == entry.trim()
        }));
    }

    Ok(())
}


#[test]
fn test_local_dotenv_loading() -> Result<(), Error> {
      let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");

    let entries = dotenvy::from_path_iter(path)?;

    for entry in entries {
        entry?;
    }

    Ok(())
}

 
