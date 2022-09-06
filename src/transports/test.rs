//! Test Transport
use crate::{error, helpers::json_rpc, rpc, Transport};
use core::future::Ready;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/// Test Transport
#[derive(Debug, Default, Clone)]
pub struct TestTransport {
    asserted: usize,
    requests: Rc<RefCell<Vec<(String, String)>>>,
    responses: Rc<RefCell<VecDeque<rpc::Value>>>,
}

impl Transport for TestTransport {
    type Out = Ready<error::Result<Vec<u8>>>;

    fn execute(&self, method: &'static str, params: Vec<crate::Value>) -> Self::Out {
        let request = json_rpc::encode_request(method, params);
        self.requests.borrow_mut().push((method.into(), request));
        let response = self.responses.borrow_mut().pop_front().unwrap();
        let returning = format!(r#"{{ "id": 0, "jsonrpc": "2.0", "result": {} }}"#, serde_json::to_string(&response).unwrap());
        core::future::ready(Ok(returning.as_bytes().to_vec()))
    }
}

impl TestTransport {
    /// Set response
    pub fn set_response(&mut self, value: rpc::Value) {
        *self.responses.borrow_mut() = vec![value].into();
    }

    /// Add response
    pub fn add_response(&mut self, value: rpc::Value) {
        self.responses.borrow_mut().push_back(value);
    }

    /// Assert request
    pub fn assert_request(&mut self, method: &str, params: &[String]) {
        let idx = self.asserted;
        self.asserted += 1;

        let (m, p) = self.requests.borrow().get(idx).expect("Expected result.").clone();
        assert_eq!(&m, method);
        let params = params.join(",");
        let payload = format!(r#"{{"id":0,"method":"{method}","params":[{params}]}}"#);
        let expected: serde_json::Value = serde_json::from_str(&payload).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&p).unwrap();
        assert_eq!(actual, expected);
    }

    /// Assert no more requests
    pub fn assert_no_more_requests(&self) {
        let requests = self.requests.borrow();
        assert_eq!(
            self.asserted,
            requests.len(),
            "Expected no more requests, got: {:?}",
            &requests[self.asserted..]
        );
    }
}
