// Component to store basic unit information

use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct NameComponent {
    pub name: String,
    pub desc: String,
}