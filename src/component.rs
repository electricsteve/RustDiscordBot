use poise::Command;

pub struct Component {
    commands: Vec<Command<crate::Data, crate::Error>>
}