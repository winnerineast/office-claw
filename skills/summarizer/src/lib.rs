#[allow(dead_code)]
mod bindings;

use crate::bindings::exports::near::agent::tool::{Guest, Request, Response};

struct Summarizer;

impl Guest for Summarizer {
    fn execute(req: Request) -> Response {
        let input = req.params;
        let summary = if input.len() > 50 {
            format!("Summary: {}...", &input[0..50])
        } else {
            format!("Summary: {}", input)
        };

        Response {
            output: Some(serde_json::to_string(&summary).unwrap()),
            error: None,
        }
    }

    fn schema() -> String {
        r#"{"type": "string"}"#.to_string()
    }

    fn description() -> String {
        "Summarizes an email string.".to_string()
    }
}

crate::bindings::export!(Summarizer with_types_in crate::bindings);
