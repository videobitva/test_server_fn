use leptos::*;
use some_lib::{associated_type, just_type};

// does not compile
#[server()]
pub async fn with_assoc_type(data: associated_type::ActiveModel) -> Result<(), ServerFnError> {
    Ok(())
}

// does compile
#[server()]
pub async fn with_just_type(data: just_type::ActiveModel) -> Result<(), ServerFnError> {
    Ok(())
}
