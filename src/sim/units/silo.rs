use crate::sim::engine::{*, interactable::{InteractParameter, InteractAction, InteractParameterValue}, position::Position, entity::MADEntity};

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
    Fueling(u32), // Fueling for x seconds
    Countdown(u32), // Countdown for x seconds
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

        // Create the actions
        let launch_nuke_action = InteractAction{
            name: String::from("Launch Nuke"),
            description: String::from("Launch a nuke at the current target"),
            action: SiloUnitActions::LaunchNuke,
            render: || {
                todo!()
            }
        };

        let drill_action = InteractAction{
            name: String::from("Drill"),
            description: String::from("Drill for more fuel"),
            action: SiloUnitActions::Drill,
            render: || {
                todo!()
            }
        };

        // Create the parameters
        let current_target_parameter = InteractParameter{
            name: String::from("Current Target"),
            description: String::from("The current target of the silo"),
            value: InteractParameterValue::String(String::from("")),
            render: || {
                todo!()
            }
        };

        let current_yield_parameter = InteractParameter{
            name: String::from("Current Yield"),
            description: String::from("The current yield of the silo"),
            value: InteractParameterValue::Number(0.0),
            render: || {
                todo!()
            }
        };
        SiloUnit{
            id: String::from("silo"),
            actions: vec![launch_nuke_action, drill_action],
            parameters: vec![current_target_parameter, current_yield_parameter],
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
        self.state = SiloUnitState::Idle;
    }

    fn set_position(&mut self, position: Position) {
        self.positiuon = position;
    }

    fn get_position(&self) -> &Position {
        &self.positiuon
    }

    fn get_bounds(&self) -> (f32, f32) {
        (10.0, 10.0)
    }

    fn get_sprite_id(&self) -> String {
        todo!()
    }
}
