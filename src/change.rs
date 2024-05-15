use std::collections::{HashMap};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum Change<T> {
    Added(T),
    Changed(T),
    Removed(T),
}

// Function to find changes and categorize them
pub fn categorize_changes<T: PartialEq + Clone, I: PartialEq + Hash + Eq, F:Fn(&T) -> I>(old_array: &[T], new_array: &[T], get_identifier: F) -> Vec<Change<T>> {
    let old_set: HashMap<I, T> = old_array.iter().cloned().map(|e| (get_identifier(&e), e)).collect();
    let new_set: HashMap<I, T> = new_array.iter().cloned().map(|e| (get_identifier(&e), e)).collect();

    let mut changes = Vec::new();

    // Check for added elements
    for (key, element) in &new_set {
        if !old_set.contains_key(key) {
            changes.push(Change::Added(element.clone()));
        }
    }

    // Check for changed elements
    for (key, element) in &new_set {
        if let Some(new_element) = old_set.get(key) {
            if new_element != element {
                changes.push(Change::Changed(element.clone()));
            }
        }
    }

    // Check for removed elements
    for (key, element) in &old_set {
        if !new_set.contains_key(key) {
            changes.push(Change::Removed(element.clone()));
        }
    }

    changes
}

