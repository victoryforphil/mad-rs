// Component to store basic unit information

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct NameComponent {
    pub name: &'static str,
    pub desc: &'static str,
}