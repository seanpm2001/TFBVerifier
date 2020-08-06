use crate::benchmark::BenchmarkCommands;
use crate::error::VerifierResult;
use crate::request::{get_response_body, get_response_headers, ContentType};
use crate::test_type::Executor;
use crate::verification::Messages;

pub struct Plaintext {
    pub concurrency_levels: Vec<i64>,
    pub pipeline_concurrency_levels: Vec<i64>,
}
impl Executor for Plaintext {
    fn retrieve_benchmark_commands(&self, _url: &str) -> VerifierResult<BenchmarkCommands> {
        // todo

        Ok(BenchmarkCommands {
            primer_command: vec![],
            warmup_command: vec![],
            benchmark_commands: vec![],
        })
    }

    fn verify(&self, url: &str) -> VerifierResult<Messages> {
        let mut messages = Messages::new(url);

        let response_headers = get_response_headers(&url)?;
        messages.headers(&response_headers);
        self.verify_headers(
            &response_headers,
            &url,
            ContentType::Plaintext,
            &mut messages,
        );
        let response_body = get_response_body(&url, &mut messages);
        messages.body(&response_body);

        self.verify_plaintext(&response_body, &mut messages);

        Ok(messages)
    }
}
impl Plaintext {
    fn verify_plaintext(&self, response_body: &str, messages: &mut Messages) {
        let body = response_body.to_lowercase();
        let expected = "hello, world!";
        let extra_bytes = body.len() - expected.len();

        if !body.contains(expected) {
            messages.error(
                format!("Could not find 'Hello, World!' in response: '{}'", body),
                "Invalid response body",
            );
        }

        if extra_bytes > 0 {
            messages.warning(
                format!("Server is returning {} more bytes than are required. This may negatively affect benchmark performance.", extra_bytes),
                "Additional response byte(s)"
            );
        }
    }
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use crate::test_type::plaintext::Plaintext;
    use crate::verification::Messages;

    #[test]
    fn it_should_succeed_on_correct_body() {
        let plaintext = Plaintext {
            concurrency_levels: vec![16, 32, 64, 128, 256, 512],
            pipeline_concurrency_levels: vec![16, 32, 64, 128, 256, 512],
        };
        let mut messages = Messages::default();
        plaintext.verify_plaintext("Hello, World!", &mut messages);
        assert!(messages.errors.is_empty());
        assert!(messages.warnings.is_empty());
    }

    #[test]
    fn it_should_fail_on_incorrect_message() {
        let plaintext = Plaintext {
            concurrency_levels: vec![16, 32, 64, 128, 256, 512],
            pipeline_concurrency_levels: vec![16, 32, 64, 128, 256, 512],
        };
        let mut messages = Messages::default();
        plaintext.verify_plaintext("World, Hello!", &mut messages);
        let mut found = false;
        for error in messages.errors {
            if error
                .message
                .contains("Could not find 'Hello, World!' in response")
            {
                found = true;
                break;
            }
        }
        assert!(found);
    }
}
