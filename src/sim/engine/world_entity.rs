use log::{info, debug};

use super::{time::MADTime, position::Position, entity::MADTestEntity};

/// Struct to store useful data for rendering
/// 
/// # Fields
/// 
pub struct MADRenderContext{

}

///
/// World entity trait
/// 
/// A trait for entities that can be added to a world and rendered
/// 
pub trait MADWorldEntity{
    ///
    /// Renders the entity to the screen
    /// 
    /// # Arguments
    /// 
    /// * `render_context` - The render context
    fn render(&mut self, render_context: &MADRenderContext);

    ///
    /// Sets the position of the entity in world space
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position to set
    fn set_position(&self, position: &Position);

    ///
    /// Gets the position of the entity in world space
    /// 
    /// # Returns
    /// 
    /// * `&Position` - The position of the entity
    fn get_position(&self) -> &Position;

    ///
    /// Gets the bounds of the entity (width, height) for collision / selection
    /// 
    /// # Returns
    /// 
    /// * `(f32, f32)` - The bounds of the entity
    fn get_bounds(&self) -> (f32, f32);
    fn get_sprite_id(&self) -> String;
}



// ---- TEST ---- //

struct MADWorldTestEntity{
    pub pos: Position,
}
/// Implements the MADWorldEntity trait for MADTestEntity (located in entity.rs)
impl MADWorldEntity for MADTestEntity{
    fn render(&mut self, render_context: &MADRenderContext) {
        debug!("Rendering entity");
    }

    fn set_position(&self, position: &Position) {
        todo!()
    }

    fn get_position(&self) -> &Position {
        todo!()
    }

    fn get_bounds(&self) -> (f32, f32) {
        todo!()
    }

    fn get_sprite_id(&self) -> String {
        todo!()
    }
}



