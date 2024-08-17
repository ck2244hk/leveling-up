pub trait ManualHandledError {}

#[derive(Debug)]
pub struct UserInputError(pub String);

impl ManualHandledError for UserInputError {}

#[derive(Debug)]
pub struct InvalidUnitError(pub String);
impl ManualHandledError for InvalidUnitError {}
