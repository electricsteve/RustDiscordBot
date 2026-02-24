use crate::component::Component;

pub mod moderation;

pub fn get_components() -> Vec<Component> {
    vec![
        *moderation::component(),
    ]
}