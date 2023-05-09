use crate::sim::engine::{*, interactable::{InteractParameter, InteractAction}, position::Position, entity::MADEntity};

/// Silo Unit
/// - Represents a silo that can be used to launch nukes
/// - Can be built by the player
/// 
/// 
/// # Unit Actions
/// - Launch Nuke
/// - Drill
/// 
/// # Unit Parameters
/// - Current Target
/// - Current Yield
/// - Number of Nukes
/// 
pub struct SiloUnit{
    pub id: String,
    pub actions: Vec<InteractAction<SiloUnitActions>>,
    pub parameters: Vec<InteractParameter>,
    pub positiuon: Position,

    // Internal states
    state: SiloUnitState,
    has_missle: bool
    
}

pub enum SiloUnitState{
    Disabled,
    Idle,
    Fueling(u32),
    Countdown(u32),
    Launching,
    LaunchingDrill,
    Empty,
}

/// Silo Unit Actions
/// 
/// # Actions
/// - Launch Nuke
/// - Drill
pub enum SiloUnitActions{
    LaunchNuke,
    Drill,
}


impl SiloUnit{
    pub fn new() -> SiloUnit{
        SiloUnit{
            id: String::from("silo"),
            actions: vec![],
            parameters: vec![],
            positiuon: Position::zero(),

            state: SiloUnitState::Disabled,
            has_missle: false,
        }
    }
}

impl MADEntity for SiloUnit{
    fn update(&mut self, delta_time: &time::MADTime) {
        todo!()
    }

    fn init(&self) {
        todo!()
    }
}
