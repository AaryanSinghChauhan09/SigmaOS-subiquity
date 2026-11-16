// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-subiquity: subiquity component model.
//!
//! Real implementation: subiquity_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubiquityItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubiquityError {
    EmptySubiquityItemName,
    DuplicateSubiquityItem(String),
    EmptyEventSubject,
}

impl fmt::Display for SubiquityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySubiquityItemName => write!(f, "subiquity_item name must not be empty"),
            Self::DuplicateSubiquityItem(id) => write!(f, "duplicate subiquity_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SubiquityError {}

pub struct Subiquity {
    pub name: String,
    subiquity_items: BTreeMap<String, SubiquityItem>,
    events: BTreeMap<String, Event>,
}

impl Subiquity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            subiquity_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_subiquity_item(&mut self, item: SubiquityItem) -> Result<(), SubiquityError> {
        if item.name.trim().is_empty() {
            return Err(SubiquityError::EmptySubiquityItemName);
        }
        let id = item.id.clone();
        if self.subiquity_items.contains_key(&id) {
            return Err(SubiquityError::DuplicateSubiquityItem(id.clone()));
        }
        self.subiquity_items.insert(id, item);
        Ok(())
    }

    pub fn subiquity_item_count(&self) -> usize {
        self.subiquity_items.len()
    }

    pub fn get_subiquity_item(&self, id: &str) -> Option<&SubiquityItem> {
        self.subiquity_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SubiquityError> {
        if event.subject.trim().is_empty() {
            return Err(SubiquityError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_subiquity_item(id: &str, name: &str) -> SubiquityItem {
        SubiquityItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Subiquity::new("Subiquity");
        assert_eq!(m.name, "Subiquity");
        assert_eq!(m.subiquity_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_subiquity_item() {
        let mut m = Subiquity::new("Subiquity");
        m.add_subiquity_item(sample_subiquity_item("a1", "Alpha")).unwrap();
        assert_eq!(m.subiquity_item_count(), 1);
        assert!(m.get_subiquity_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Subiquity::new("Subiquity");
        assert!(m.add_subiquity_item(sample_subiquity_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Subiquity::new("Subiquity");
        m.add_subiquity_item(sample_subiquity_item("x", "One")).unwrap();
        assert!(m.add_subiquity_item(sample_subiquity_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Subiquity::new("Subiquity");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
