use syn::{Ident, AttributeArgs};
use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct MethodMacroArgs {
    #[darling(default)]
    pub action: Option<Ident>,
    pub route: String,
    pub controller_name: Ident
}

impl MethodMacroArgs {
    pub fn set_action(&mut self, action_name: &str) {
        self.action = Some(Ident::new(action_name, self.controller_name.span()));
    }   
}

pub fn parse_args(args: &AttributeArgs) -> MethodMacroArgs {
    match MethodMacroArgs::from_list(args) {
        Ok(v) => v,
        Err(e) => { panic!("Cannot parse arguments! \nArguemtns: {:?} \nError: {}", args, e)}
    }
}