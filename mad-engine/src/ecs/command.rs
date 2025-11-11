use crate::component::ECSComponent;

#[derive(Debug, Clone)]
pub enum ECSCommandType {
    /// Spawns the list of components with the same entity index, grouping them together
    SpawnEntity(Vec<ECSComponent>),
    AppendToEntity(u32, Vec<ECSComponent>),
    RemoveFromEntity(u32, Vec<ECSComponent>),
}
