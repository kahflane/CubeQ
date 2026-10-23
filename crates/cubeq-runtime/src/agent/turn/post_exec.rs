//! Post-execution recording: result log line, the `after_tool_call` hook, the
//! completion Status, and filling the executed calls' `ordered_results` slots.

use super::call_prep::StreamToolCall;
use super::context::TurnCtx;
use super::events::{ProgressEvent, StreamDelta, send_progress};
use super::redact::scrub_credentials;
use crate::agent::tool_execution::ToolExecutionOutcome;
use cubeq_tool_call_parser::ParsedToolCall;

/// Record each executed tool call's outcome (upstream loop body,
/// post-execution section): one `tool_call_result` log line, the
/// `after_tool_call` hook, a completion Status to the draft, and the
/// call's slot in `ordered_results`.
pub(crate) async fn record_executed_outcomes(
    ctx: &TurnCtx<'_>,
    executable_indices: &[usize],
    executable_calls: &[ParsedToolCall],
    stream_calls: &[Option<StreamToolCall>],
    executed_outcomes: Vec<ToolExecutionOutcome>,
    ordered_results: &mut [Option<(String, Option<String>, ToolExecutionOutcome)>],
    iteration: usize,
) {
    for (((idx, call), stream_call), outcome) in executable_indices
        .iter()
        .zip(executable_calls.iter())
        .zip(stream_calls.iter())
        .zip(executed_outcomes)
    {
        // The pending ToolCall and terminal ToolResult are emitted by the
        // executor (execute_one_tool) at dispatch and completion time so serial
        // batches interleave call->result per tool. Post-exec only records the
        // outcome to history, logs, hooks, and ordered_results.

        let error_reason_for_audit = if outcome.audit_output.is_some() {
            outcome
                .error_reason
                .as_ref()
                .map(|_| "sensitive tool error omitted")
        } else {
            outcome.error_reason.as_deref()
        };

        ::cubeq_log::record!(
            INFO,
            ::cubeq_log::Event::new(module_path!(), ::cubeq_log::Action::Complete)
                .with_category(::cubeq_log::EventCategory::Tool)
                .with_outcome(if outcome.success {
                    ::cubeq_log::EventOutcome::Success
                } else {
                    ::cubeq_log::EventOutcome::Failure
                })
                .with_duration(u64::try_from(outcome.duration.as_millis()).unwrap_or(u64::MAX))
                .with_attrs(::serde_json::json!({
                    "model": ctx.model,
                    "iteration": iteration + 1,
                    "tool": call.name.clone(),
                    "error_reason": error_reason_for_audit.map(scrub_credentials),
                    "output": scrub_credentials(outcome.output_for_audit()),
                    "trace_id": ctx.turn_id,
                })),
            "tool_call_result"
        );

        // ── Hook: after_tool_call (void) ─────────────────
        if let Some(hooks) = ctx.hooks {
            let hook_context = crate::hooks::tool_call_hook_context(ctx.turn_id, iteration, *idx);
            let tool_result_obj = crate::tools::ToolResult {
                success: outcome.success,
                output: outcome.output_for_audit().into(),
                error: None,
            };
            // The prepared arguments travel with the completion call so
            // argument-auditing hooks export what was actually dispatched and
            // never need to retain arguments between the hook phases.
            hooks
                .fire_after_tool_call_with_context_and_args(
                    &hook_context,
                    &call.name,
                    &call.arguments,
                    &tool_result_obj,
                    outcome.duration,
                )
                .await;
        }

        // ── Progress: tool completion ───────────────────────
        send_progress(ctx.on_delta, ProgressEvent::Planning).await;
        if let (Some(tx), Some(stream_call)) = (ctx.on_delta, stream_call) {
            let secs = outcome.duration.as_secs();
            ::cubeq_log::record!(
                DEBUG,
                ::cubeq_log::Event::new(module_path!(), ::cubeq_log::Action::Note)
                    .with_category(::cubeq_log::EventCategory::Tool)
                    .with_attrs(::serde_json::json!({"tool": call.name, "secs": secs})),
                "Sending progress complete to draft"
            );
            let _ = tx
                .send(StreamDelta::ToolComplete {
                    tool: call.name.clone(),
                    arguments: std::sync::Arc::clone(&stream_call.arguments),
                    tool_provenance: stream_call.tool_provenance,
                    secs,
                    success: outcome.success,
                    error: outcome.error_reason.as_deref().map(scrub_credentials),
                })
                .await;
        }

        // Capture into the innermost live SOP step scope (no-op otherwise).
        if crate::sop::executor::step_capture_active() {
            crate::sop::executor::record_step_tool_call(
                &call.name,
                &call.arguments,
                outcome.success,
                outcome.output_for_audit().to_string(),
                if outcome.audit_output.is_some() {
                    None
                } else {
                    outcome.output_data.clone()
                },
                error_reason_for_audit,
                u64::try_from(outcome.duration.as_millis()).unwrap_or(u64::MAX),
            );
        }

        ordered_results[*idx] = Some((call.name.clone(), call.tool_call_id.clone(), outcome));
    }
}
