
/// Trait for objects that can be interacted with 
/// both via actiions or parameters

pub trait Interactable<T>{
    fn get_actions(&self) -> Vec<InteractAction<T>>;
    fn get_parameters(&self) -> Vec<InteractParameter>;
}

/// An action that can be performed on an interactable object
/// 
/// # Fields
/// 
/// * `name` - The name of the action
/// * `description` - A description of the action
/// * `action` - The action to perform (Enum)
/// * `render` - The render function for the action
/// 
#[derive(Clone, Debug)]
pub struct InteractAction<T>{
    pub name: String,
    pub description: String,
    pub action: T,
    pub render: fn() -> ()
}


#[derive(Clone, Debug)]
pub enum InteractParameterValue{
    String(String),
    Number(f32),
    Boolean(bool,),
    Select( Vec<String>),
}
/// A parameter that can be set on an interactable object
/// 
/// # Fields
/// 
/// * `name` - The name of the parameter
/// * `description` - A description of the parameter
/// * `value` - The value of the parameter
/// * `render` - The render function for the parameter
/// 

#[derive(Clone, Debug)]
pub struct InteractParameter{
    pub name: String,
    pub description: String,
    pub value: InteractParameterValue,
    pub render: fn() -> (),
}


// ---- TESTS ---- //

// Test Structs

#[derive(Clone, Debug)]
enum TestActionOptions {
    TestAction1,
    TestAction2,
    TestAction3,
}

#[derive(Clone, Debug)]
struct TestInteractable{
    pub actions: Vec<InteractAction<TestActionOptions>>,
    pub parameters: Vec<InteractParameter>,
}

impl TestInteractable{
    pub fn new() -> TestInteractable{
        TestInteractable{
            actions: vec![],
            parameters: vec![],
        }
    }
}

impl Interactable<TestActionOptions> for TestInteractable{
    fn get_actions(&self) -> Vec<InteractAction<TestActionOptions>>{
        self.actions.clone()
    }

    fn get_parameters(&self) -> Vec<InteractParameter>{
        self.parameters.clone()
    }
}


#[cfg(test)]
mod tests{
    use std::cell::Cell;

    use log::info;

    use super::*;

    #[test]
    fn test_interactable(){
        let interactable = TestInteractable::new();

        assert_eq!(interactable.get_actions().len(), 0);
        assert_eq!(interactable.get_parameters().len(), 0);
    }

    #[test]
    fn test_action(){
       
        
        let action_1 = InteractAction{
            name: String::from("Test Action 1 "),
            description: String::from("A test action 1"),
            action: TestActionOptions::TestAction1,
            render: ||{},
        };

        let action_2 = InteractAction{
            name: String::from("Test Action 2 "),
            description: String::from("A test action 2"),
            action: TestActionOptions::TestAction2,
            render: ||{},
        };

        let action_3 = InteractAction{
            name: String::from("Test Action 3 "),
            description: String::from("A test action 3"),
            action: TestActionOptions::TestAction3,
            render: ||{},
        };

        let mut interactable = TestInteractable::new();

        interactable.actions.push(action_1);
        interactable.actions.push(action_2);
        interactable.actions.push(action_3);

        assert_eq!(interactable.get_actions().len(), 3);
        assert_eq!(interactable.get_parameters().len(), 0);

    }

    #[test]
    fn test_parameter(){
        let parameter_1 = InteractParameter{
            name: String::from("Test Parameter 1 "),
            description: String::from("A test parameter 1"),
            value: InteractParameterValue::String(String::from("Test String")),
            render: ||{},
        };

        let parameter_2 = InteractParameter{
            name: String::from("Test Parameter 2 "),
            description: String::from("A test parameter 2"),
            value: InteractParameterValue::Number(1.0),
            render: ||{},
        };

        let parameter_3 = InteractParameter{
            name: String::from("Test Parameter 3 "),
            description: String::from("A test parameter 3"),
            value: InteractParameterValue::Boolean(true),
            render: ||{},
        };

        let mut interactable = TestInteractable::new();

        interactable.parameters.push(parameter_1);
        interactable.parameters.push(parameter_2);
        interactable.parameters.push(parameter_3);

        assert_eq!(interactable.get_actions().len(), 0);
        assert_eq!(interactable.get_parameters().len(), 3);
    }
}
