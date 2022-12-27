use strum_macros::{EnumString};

pub fn generate_rule_elements() -> Vec<RuleElement> {
    vec![
        RuleElement::FlatModifier,
        RuleElement::Immunity
    ] 
}


#[derive(Debug, Clone, PartialEq, Eq, EnumString, Hash)]
pub enum RuleElement {
    FlatModifier,
    Immunity
}
impl RuleElement {
    pub fn fields(&self) -> Vec<Field> {
        match self {
            RuleElement::FlatModifier => vec![Field::Label],
            RuleElement::Immunity => vec![Field::Value],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Hash)]
pub enum Field {
    Label,
    Value
}