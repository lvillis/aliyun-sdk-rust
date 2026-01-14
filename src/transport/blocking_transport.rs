use std::time::Duration;

use crate::transport::{BlockingTransport, BoxError, Request, Response};

pub(crate) struct UreqTransport {
    connect_timeout: Duration,
}

impl UreqTransport {
    pub(crate) fn new(connect_timeout: Duration) -> Result<Self, BoxError> {
        Ok(Self { connect_timeout })
    }
}

impl BlockingTransport for UreqTransport {
    fn send(&self, request: Request) -> Result<Response, BoxError> {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_connect(Some(self.connect_timeout))
            .timeout_global(Some(request.timeout))
            .build();
        let agent = ureq::Agent::new_with_config(config);

        let mut builder = http::Request::builder()
            .method(request.method)
            .uri(request.url.as_str());

        for (name, value) in request.headers.iter() {
            builder = builder.header(name, value);
        }

        let request = builder.body(())?;
        let mut response = agent.run(request)?;

        let status = response.status();
        let headers = response.headers().clone();

        let body = response.body_mut().read_to_vec()?;

        Ok(Response {
            status,
            headers,
            body,
        })
    }
}
