//Copyright (c) 2017 Rio Advancement Inc

use super::super::super::APPLICABLE_TO;

/// The service rules supported as events
pub enum ServiceRule {
    ReAssemble,
    Assemble,
}

pub struct ServiceTallyer {}

impl ServiceTallyer {
    /// Returns
    /// true: If the plan category is an application or container
    pub fn tally(b: String) -> bool {
        Self::not_applicable(&b)
    }

    /// Returns
    /// true: if the service rule is reasemble -  sent from reassembler.
    pub fn tally_rule(rule: &ServiceRule) -> bool {
        match rule {
            &ServiceRule::ReAssemble => true,
            _ => false,
        }
    }

    fn not_applicable(b: &str) -> bool {
        APPLICABLE_TO.contains(&b)
    }
}
