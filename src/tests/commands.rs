use crate::{commands, Error};

#[test]
fn test_command_registration() {
    let registered = commands::all();

    let mut names: Vec<&str> = registered
    .iter()
    .map(|command| command.name.as_ref())
    .collect();

    names.sort_unstable();

    assert_eq!(names, vec!["bless", "cast", "consult", "curse", "divine"]);
    assert!(registered.iter().all(|command| {
        command.slash_action.is_some()
    }));
}

#[test]
fn picks_an_existing_entry() -> Result<(), Error> {
    let entries = vec![
        "Yes.".to_owned(),
        "No.".to_owned(),
        "Ask again later.".to_owned(),
    ];

    let response = commands::pick_random_response(&entries)?;

    assert!(entries.contains(&response));
    Ok(())
}

#[test]
fn rejects_an_empty_list() {
    assert!(commands::pick_random_response(&[]).is_err());
}
