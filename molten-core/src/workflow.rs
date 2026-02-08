//! This module defines the core structures for managing workflow definitions,
//! which dictate the lifecycle and state transitions of documents within the
//! Molten system.
//!
//! It includes `Phase` and `Transition` to model the states and movements
//! within a workflow, `WorkflowDefinition` to represent a complete state machine,
//! and `WorkflowBuilder` for programmatic construction and validation of workflows.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;
use validator::{Validate, ValidationError};

// -----------------------------------------------------------------------------
// Enums & Sub-Structs
// -----------------------------------------------------------------------------

/// Defines the behavior of a specific phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseType {
    /// The entry point of the workflow. There should typically be only one.
    Start,
    /// A standard working state (e.g., "Draft", "Under Review").
    Normal,
    /// A terminal state (e.g., "Approved", "Rejected", "Void").
    End,
}

/// A single state within the workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Phase {
    /// Unique identifier for this phase (e.g., "draft").
    #[validate(length(min = 1, max = 64))]
    pub id: String,

    /// Human-readable name (e.g., "Draft Mode").
    #[validate(length(min = 1, max = 100))]
    pub label: String,

    /// The behavior type of this phase.
    #[serde(rename = "type")]
    pub phase_type: PhaseType,
}

impl Phase {
    /// Creates a new `Phase` instance.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the phase.
    /// * `label` - The human-readable name for the phase.
    /// * `phase_type` - The type of the phase (e.g., Start, Normal, End).
    pub fn new(id: &str, label: &str, phase_type: PhaseType) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            phase_type,
        }
    }
}

/// A directed edge between two Phases.
/// Represents a valid movement from `from_phase` to `to_phase`.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Transition {
    /// The name of the action (e.g., "Submit for Review").
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// The ID of the source phase.
    #[validate(length(min = 1, max = 64))]
    pub from: String,

    /// The ID of the target phase.
    #[validate(length(min = 1, max = 64))]
    pub to: String,
    // Future expansion: We will add "guards" or "permissions" here later.
    // e.g., pub required_role: Option<String>

    // TODO: Add required_fields for a transition to be valid.
    // This is different from the global is_required in Field definition
    // #[serde(default)]
    // pub required_fields: Vec<String>,
}

impl Transition {
    /// Creates a new `Transition` instance.
    ///
    /// # Arguments
    /// * `name` - The name of the action represented by this transition.
    /// * `from` - The ID of the source phase.
    /// * `to` - The ID of the target phase.
    pub fn new(name: &str, from: &str, to: &str) -> Self {
        Self {
            name: name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// Workflow Definition (The Graph)
// -----------------------------------------------------------------------------

/// Defines the complete state machine.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(try_from = "WorkflowBuilder")]
pub struct WorkflowDefinition {
    /// The unique identifier for this workflow.
    #[validate(length(min = 1, max = 64))]
    id: String,

    /// Human-readable name for the workflow.
    #[validate(length(min = 1, max = 100))]
    name: String,

    /// All available phases (states) within this workflow.
    #[validate(nested)]
    phases: Vec<Phase>,

    /// All allowed transitions (movements) between phases.
    #[validate(nested)]
    transitions: Vec<Transition>,
}

/// Trait for querying workflow capability.
pub trait WorkflowGraph {
    /// Returns true if a transition exists from `current_phase` to `target_phase`.
    fn can_transition(&self, current_phase: &str, target_phase: &str) -> bool;

    /// Returns the Phase definition for a given ID.
    fn get_phase(&self, phase_id: &str) -> Option<&Phase>;

    /// Returns the starting phase of the workflow.
    fn get_start_phase(&self) -> Option<&Phase>;
}

impl WorkflowGraph for WorkflowDefinition {
    fn can_transition(&self, current_phase: &str, target_phase: &str) -> bool {
        self.transitions
            .iter()
            .any(|t| t.from == current_phase && t.to == target_phase)
    }

    fn get_phase(&self, phase_id: &str) -> Option<&Phase> {
        self.phases.iter().find(|p| p.id == phase_id)
    }

    fn get_start_phase(&self) -> Option<&Phase> {
        self.phases
            .iter()
            .find(|p| p.phase_type == PhaseType::Start)
    }
}

impl WorkflowDefinition {
    /// Returns the ID of the workflow.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Returns the human-readable name of the workflow.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns a slice of all phases in the workflow.
    pub fn phases(&self) -> &[Phase] {
        &self.phases
    }
    /// Returns a slice of all transitions in the workflow.
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }
}

// -----------------------------------------------------------------------------
// Validation Logic
// -----------------------------------------------------------------------------

// TODO: Implement additional validations for transitions, such as rules for phase types
/// Ensures that all transitions in a `WorkflowDefinition` refer to valid, existing phases.
///
/// This validation prevents transitions from or to non-existent phases, ensuring the
/// integrity and consistency of the workflow graph.
///
/// # Arguments
/// * `definition` - A reference to the `WorkflowDefinition` to validate.
///
/// # Returns
/// A `Result` which is `Ok` if all transitions are valid, or `Err` with
/// `validator::ValidationErrors` if any invalid transitions are found.
fn validate_workflow_integrity(
    definition: &WorkflowDefinition,
) -> Result<(), validator::ValidationErrors> {
    let phase_ids: HashSet<&String> = definition.phases.iter().map(|p| &p.id).collect();

    let mut errors = validator::ValidationErrors::new();

    for transition in definition.transitions.iter() {
        if !phase_ids.contains(&transition.from) {
            let mut err = ValidationError::new("invalid_transition_source");
            err.add_param("phase_id".into(), &transition.from);
            errors.add("transitions", err);
        }

        if !phase_ids.contains(&transition.to) {
            let mut err = ValidationError::new("invalid_transition_target");
            err.add_param("phase_id".into(), &transition.to);
            errors.add("transitions", err);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// -----------------------------------------------------------------------------
// Builder & Deserialization
// -----------------------------------------------------------------------------

/// Builder for constructing validated [`WorkflowDefinition`] instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowBuilder {
    /// The unique identifier for the workflow.
    pub id: String,
    /// Human-readable name for the workflow.
    pub name: String,
    #[serde(default)]
    /// The phases that make up this workflow.
    pub phases: Vec<Phase>,
    #[serde(default)]
    /// The transitions between phases in this workflow.
    pub transitions: Vec<Transition>,
}

impl WorkflowBuilder {
    /// Creates a new `WorkflowBuilder` instance with the given ID and name,
    /// defaulting phases and transitions to empty lists.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            phases: Vec::new(),
            transitions: Vec::new(),
        }
    }

    /// Adds a `Phase` to the workflow.
    pub fn add_phase(mut self, phase: Phase) -> Self {
        self.phases.push(phase);
        self
    }

    /// Adds a `Transition` to the workflow.
    pub fn add_transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Builds a validated `WorkflowDefinition` from the `WorkflowBuilder` instance.
    ///
    /// # Returns
    /// A `Result` containing the `WorkflowDefinition` if valid, or a
    /// `validator::ValidationErrors` if validation fails.
    pub fn build(self) -> Result<WorkflowDefinition, validator::ValidationErrors> {
        WorkflowDefinition::try_from(self)
    }
}

impl TryFrom<WorkflowBuilder> for WorkflowDefinition {
    type Error = validator::ValidationErrors;

    fn try_from(builder: WorkflowBuilder) -> Result<Self, Self::Error> {
        let wf = WorkflowDefinition {
            id: builder.id,
            name: builder.name,
            phases: builder.phases,
            transitions: builder.transitions,
        };

        // 1. Standard Field Validation
        wf.validate()?;

        // 2. Graph Integrity Validation (Custom Logic)
        validate_workflow_integrity(&wf)?;

        Ok(wf)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_integrity() {
        let res = WorkflowBuilder::new("wf_1", "Simple Workflow")
            .add_phase(Phase::new("start", "Start", PhaseType::Start))
            .add_phase(Phase::new("end", "End", PhaseType::End))
            // Valid transition
            .add_transition(Transition::new("finish", "start", "end"))
            .build();

        assert!(res.is_ok());
        let wf = res.unwrap();
        assert!(wf.can_transition("start", "end"));
        assert!(!wf.can_transition("end", "start")); // One way!
    }

    #[test]
    fn test_broken_reference() {
        let res = WorkflowBuilder::new("wf_bad", "Broken Workflow")
            .add_phase(Phase::new("start", "Start", PhaseType::Start))
            // Transition to 'end', but 'end' phase is not added!
            .add_transition(Transition::new("finish", "start", "end"))
            .build();

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("invalid_transition_target"));
    }

    #[test]
    fn test_get_start_phase() {
        let wf = WorkflowBuilder::new("wf_1", "Test")
            .add_phase(Phase::new("draft", "Draft", PhaseType::Start))
            .build()
            .unwrap();

        let start = wf.get_start_phase().unwrap();
        assert_eq!(start.id, "draft");
        assert!(matches!(start.phase_type, PhaseType::Start));
    }
}
