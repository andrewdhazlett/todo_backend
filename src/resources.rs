use actix_web::{Path, State};
use app::TodoState;

#[derive(Deserialize)]
pub struct GreetingParams {
    name: String,
}

pub fn put_name(
    (state, params): (State<TodoState>, Path<GreetingParams>),
) -> String {
    state.name.replace(params.name.clone());
    "success".to_string()
}

pub fn get_greeting(state: State<TodoState>) -> String {
    let name = state.name.try_borrow().unwrap();
    format!("Hello, {}!", &name)
}
