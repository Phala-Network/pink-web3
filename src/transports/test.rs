//! Test Transport
use crate::{error, helpers::json_rpc, Transport};
use core::future::Ready;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/// Test Transport
#[derive(Debug, Default, Clone)]
pub struct TestTransport {
    asserted: usize,
    requests: Rc<RefCell<Vec<(String, String)>>>,
    responses: Rc<RefCell<VecDeque<Vec<u8>>>>,
}

impl Transport for &TestTransport {
    type Out = Ready<error::Result<Vec<u8>>>;
    fn execute(&self, method: &'static str, params: Vec<&dyn erased_serde::Serialize>) -> Self::Out {
        (*self).execute(method, params)
    }
}

impl Transport for TestTransport {
    type Out = Ready<error::Result<Vec<u8>>>;
    fn execute(&self, method: &'static str, params: Vec<&dyn erased_serde::Serialize>) -> Self::Out {
        let request = json_rpc::encode_request::<_, 512>(method, params);
        self.requests.borrow_mut().push((method.into(), request));
        core::future::ready(Ok(self.responses.borrow_mut().pop_front().unwrap()))
    }
}

impl TestTransport {
    /// Set response
    pub fn set_response(&mut self, value: &[u8]) {
        *self.responses.borrow_mut() = vec![value.into()].into();
    }

    /// Add response
    pub fn add_response(&mut self, value: &[u8]) {
        self.responses.borrow_mut().push_back(value.to_vec());
    }

    /// Assert request
    pub fn assert_request(&mut self, method: &str, params: &str) {
        let idx = self.asserted;
        self.asserted += 1;

        let (m, p) = self.requests.borrow().get(idx).expect("Expected result.").clone();
        assert_eq!(&m, method);
        let payload = format!(r#"{{"id":0,"method":"{method}","params":{params}}}"#);
        assert_eq!(p, payload);
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
