use crate::azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use crate::azure::core::lease::LeaseId;
use crate::azure::core::{
    ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired, ContainerNameSupport, LeaseIdOption,
    LeaseIdSupport, TimeoutOption, TimeoutSupport,
};
use crate::azure::core::{No, ToAssign, Yes};
use crate::azure::storage::client::Client;
use crate::azure::storage::container::responses::GetPropertiesResponse;
use futures::future::{done, Future};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    p_container_name: PhantomData<ContainerNameSet>,
    client: &'a Client,
    container_name: Option<&'a str>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a, ContainerNameSet> ClientRequired<'a> for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a> GetPropertiesBuilder<'a, No> {
    pub(crate) fn new(client: &'a Client) -> GetPropertiesBuilder<'a, No> {
        GetPropertiesBuilder {
            p_container_name: PhantomData {},
            client,
            container_name: None,
            timeout: None,
            client_request_id: None,
            lease_id: None,
        }
    }
}

impl<'a> ContainerNameRequired<'a> for GetPropertiesBuilder<'a, Yes> {
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> ContainerNameSupport<'a> for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = GetPropertiesBuilder<'a, Yes>;

    fn with_container_name(self, container_name: &'a str) -> Self::O {
        GetPropertiesBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: Some(container_name),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
        }
    }
}

impl<'a> GetPropertiesBuilder<'a, Yes> {
    pub fn finalize(self) -> impl Future<Item = GetPropertiesResponse, Error = AzureError> {
        let mut uri = format!("{}/{}?restype=container", self.client().blob_uri(), self.container_name());

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let req = self.client().perform_request(
            &uri,
            &Method::HEAD,
            |ref mut request| {
                ClientRequestIdOption::add_header(&self, request);
                LeaseIdOption::add_header(&self, request);
            },
            None,
        );

        let container_name = self.container_name().to_owned();

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::OK))
            .and_then(|(headers, _body)| done(GetPropertiesResponse::from_response(container_name, &headers)))
    }
}

impl<'a, ContainerNameSet> TimeoutOption for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet> TimeoutSupport for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = GetPropertiesBuilder<'a, ContainerNameSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        GetPropertiesBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> ClientRequestIdOption<'a> for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet> ClientRequestIdSupport<'a> for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = GetPropertiesBuilder<'a, ContainerNameSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetPropertiesBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> LeaseIdOption<'a> for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet> LeaseIdSupport<'a> for GetPropertiesBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = GetPropertiesBuilder<'a, ContainerNameSet>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        GetPropertiesBuilder {
            p_container_name: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: Some(lease_id),
        }
    }
}
