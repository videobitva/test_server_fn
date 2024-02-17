use leptos::*;
use some_lib::{associated_type, just_type};

#[server(impl_into = false)]
pub async fn with_assoc_type(data: associated_type::ActiveModel) -> Result<(), ServerFnError> {
    Ok(())
}

#[server()]
pub async fn with_just_type(data: just_type::ActiveModel) -> Result<(), ServerFnError> {
    Ok(())
}
