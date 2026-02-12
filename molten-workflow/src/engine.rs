//! This module provides the functions which implement the core workflow engine logic.

use crate::error::WorkflowError;
use molten_core::document::Document;
use molten_core::workflow::{WorkflowDefinition, WorkflowGraph};

/// Attempts to transition a document from its current phase to a specified target phase
/// according to the rules defined in the provided workflow.
///
/// If the transition is valid, the `document.current_phase` field is updated in place
/// to the `target_phase_id`.
///
/// This function performs several checks:
/// 1. Verifies that the document's `workflow_id` matches the provided `workflow.id()`.
/// 2. Ensures the `target_phase_id` actually exists within the `workflow` definition.
/// 3. Handles initial transitions for new documents (those with an empty `current_phase`),
///    only allowing them to transition to the workflow's designated "Start" phase.
/// 4. Validates that a direct transition path exists from the document's `current_phase`
///    to the `target_phase_id` within the `workflow` graph.
///
/// # Arguments
/// * `doc` - A mutable reference to the `Document` to be transitioned.
/// * `workflow` - The `WorkflowDefinition` that defines the valid phases and transitions
///   for this document.
/// * `target_phase_id` - The `id` of the phase to which the document should transition.
///
/// # Returns
/// * `Ok(())` if the transition was successful and the document's phase was updated.
/// * `Err(WorkflowError)` if any of the validation checks fail (e.g., workflow mismatch,
///   unknown phase, invalid transition, or no current phase on a non-new document).
pub fn transition(
    doc: &mut Document,
    workflow: &WorkflowDefinition,
    target_phase_id: &str,
) -> Result<(), WorkflowError> {
    // 1. Sanity Check: Does the document belong to this workflow?
    if doc.workflow_id != workflow.id() {
        return Err(WorkflowError::WorkflowMismatch {
            doc_wf: doc.workflow_id.clone(),
            provided_wf: workflow.id().to_string(),
        });
    }

    // 2. Validate Target Phase Existence
    if workflow.get_phase(target_phase_id).is_none() {
        return Err(WorkflowError::UnknownPhase(target_phase_id.to_string()));
    }

    // 3. Handle "New" Documents (Empty Phase)
    // If the document has no phase, we only allow transitioning to the "Start" phase.
    if doc.current_phase.is_empty() {
        if let Some(start_phase) = workflow.get_start_phase() {
            if start_phase.id == target_phase_id {
                doc.current_phase = target_phase_id.to_string();
                return Ok(());
            } else {
                return Err(WorkflowError::InvalidTransition {
                    current: "WAITING_TO_START".to_string(),
                    target: target_phase_id.to_string(),
                });
            }
        } else {
            // Should verify workflow has start phase, but for runtime safety:
            return Err(WorkflowError::UnknownPhase(
                "No start phase defined".to_string(),
            ));
        }
    }

    // 4. Validate the Edge (The Transition Rule)
    // We delegate this check to the WorkflowGraph trait we defined in Core.
    if !workflow.can_transition(&doc.current_phase, target_phase_id) {
        return Err(WorkflowError::InvalidTransition {
            current: doc.current_phase.clone(),
            target: target_phase_id.to_string(),
        });
    }

    // 5. Apply the Change
    doc.current_phase = target_phase_id.to_string();

    // Note: In a real system, you might trigger "Side Effects" here
    // (e.g., sending emails), but that belongs in `molten-service`.

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use molten_core::workflow::{Phase, PhaseType, Transition, WorkflowBuilder};

    fn create_simple_workflow() -> WorkflowDefinition {
        WorkflowBuilder::new("wf_ticket", "Ticket Workflow")
            .add_phase(Phase::new("draft", "Draft", PhaseType::Start))
            .add_phase(Phase::new("review", "Review", PhaseType::Normal))
            .add_phase(Phase::new("closed", "Closed", PhaseType::End))
            // Define paths: Draft -> Review -> Closed
            .add_transition(Transition::new("submit", "draft", "review"))
            .add_transition(Transition::new("approve", "review", "closed"))
            // Also allow "Reject": Review -> Draft
            .add_transition(Transition::new("reject", "review", "draft"))
            .build()
            .unwrap()
    }

    #[test]
    fn test_valid_transitions() {
        let wf = create_simple_workflow();
        let mut doc = Document::new("doc1", "form_ticket", "wf_ticket");

        // 1. Initialize (Empty -> Start)
        assert!(transition(&mut doc, &wf, "draft").is_ok());
        assert_eq!(doc.current_phase, "draft");

        // 2. Draft -> Review
        assert!(transition(&mut doc, &wf, "review").is_ok());
        assert_eq!(doc.current_phase, "review");

        // 3. Review -> Closed
        assert!(transition(&mut doc, &wf, "closed").is_ok());
        assert_eq!(doc.current_phase, "closed");
    }

    #[test]
    fn test_invalid_jump() {
        let wf = create_simple_workflow();
        let mut doc = Document::new("doc1", "doc_ticket", "wf_ticket");

        // Initialize
        let _ = transition(&mut doc, &wf, "draft");

        // Try to skip Review (Draft -> Closed)
        let res = transition(&mut doc, &wf, "closed");
        assert!(res.is_err());
        assert!(matches!(
            res.unwrap_err(),
            WorkflowError::InvalidTransition { .. }
        ));

        // Ensure state didn't change
        assert_eq!(doc.current_phase, "draft");
    }

    #[test]
    fn test_workflow_mismatch() {
        let wf = create_simple_workflow(); // ID: wf_ticket
        let mut doc = Document::new("doc1", "doc_ticket", "other_workflow_id");

        let res = transition(&mut doc, &wf, "draft");
        assert!(matches!(
            res.unwrap_err(),
            WorkflowError::WorkflowMismatch { .. }
        ));
    }
}
