use actix_web::web::Data;
use std::collections::HashMap;
use std::sync::Mutex;

pub type State<'a, T> = HashMap<&'a str, T>;
pub type AppState<'a, T> = Data<Mutex<State<'a, T>>>;

pub fn new_state<'a, T>() -> AppState<'a, T> {
    let state = State::<T>::new();
    Data::new(Mutex::new(state))
}
