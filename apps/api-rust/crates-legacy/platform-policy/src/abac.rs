use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub struct PolicyContext {
    pub subject_attributes: HashMap<String, String>,
    pub resource_attributes: HashMap<String, String>,
    pub environment_attributes: HashMap<String, String>,
}

impl PolicyContext {
    pub fn new() -> Self {
        Self {
            subject_attributes: HashMap::new(),
            resource_attributes: HashMap::new(),
            environment_attributes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub id: String,
    pub effect: Effect,
    pub subject_match: HashMap<String, String>,
    pub resource_match: HashMap<String, String>,
}

impl PolicyRule {
    pub fn matches(&self, ctx: &PolicyContext) -> bool {
        // Check subject matches
        for (k, v) in &self.subject_match {
            if ctx.subject_attributes.get(k) != Some(v) {
                return false;
            }
        }
        // Check resource matches
        for (k, v) in &self.resource_match {
            if ctx.resource_attributes.get(k) != Some(v) {
                return false;
            }
        }
        true
    }
}

pub struct AbacEngine {
    rules: Vec<PolicyRule>,
}

impl AbacEngine {
    pub fn new(rules: Vec<PolicyRule>) -> Self {
        Self { rules }
    }

    pub fn evaluate(&self, ctx: &PolicyContext) -> Effect {
        let mut final_effect = Effect::Deny; // Default deny
        
        for rule in &self.rules {
            if rule.matches(ctx) {
                match rule.effect {
                    Effect::Deny => return Effect::Deny, // Explicit deny overrides everything
                    Effect::Allow => final_effect = Effect::Allow,
                }
            }
        }
        final_effect
    }
}
