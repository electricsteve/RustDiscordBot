use crate::component::Component;

pub mod moderation;
pub mod todo;

pub fn get_components() -> Vec<Component> {
    vec![*moderation::component(), *todo::component()]
}
