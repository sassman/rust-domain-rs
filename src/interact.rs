use crate::Result;

use dialoguer::{Input, Password};

pub fn user_name() -> Result<String> {
    Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .map_err(Into::<anyhow::Error>::into)
}

pub fn user_secret() -> Result<String> {
    Password::new()
        .with_prompt("Password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()
        .map_err(Into::<anyhow::Error>::into)
}
