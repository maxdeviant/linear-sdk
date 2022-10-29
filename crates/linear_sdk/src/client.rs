use graphql_client::GraphQLQuery;
use url::{ParseError, Url};

use crate::ApiKey;

/// The Linear client.
pub struct LinearClient {
    base_url: Url,
    key: ApiKey,
    client: reqwest::Client,
}

impl LinearClient {
    /// Returns a new instance of the Linear client using the provided API key.
    pub fn new(key: &ApiKey) -> Self {
        LinearClientBuilder::new(key).build()
    }

    /// Returns a [`LinearClientBuilder`] that may be used to construct a Linear client.
    pub fn builder(key: &ApiKey) -> LinearClientBuilder {
        LinearClientBuilder::new(key)
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn key(&self) -> &ApiKey {
        &self.key
    }

    pub(crate) async fn post_graphql<Q: GraphQLQuery>(
        &self,
        variables: Q::Variables,
    ) -> Result<graphql_client::Response<Q::ResponseData>, reqwest::Error> {
        let body = Q::build_query(variables);

        let response = self
            .client
            .post(self.base_url().clone())
            .bearer_auth(self.key())
            .json(&body)
            .send()
            .await?;

        response.json().await
    }

    pub async fn issue(
        &self,
        id: &str,
    ) -> Result<crate::graphql::issue::issue::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::issue::Issue>(crate::graphql::issue::issue::Variables {
                id: id.to_string(),
            })
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

/// A builder for a Linear client.
pub struct LinearClientBuilder<'a> {
    base_url: Url,
    key: &'a ApiKey,
}

impl<'a> LinearClientBuilder<'a> {
    /// Returns a new [`LinearClientBuilder`] using the provided API key.
    pub fn new(key: &'a ApiKey) -> Self {
        Self {
            base_url: Url::parse("https://api.linear.app/graphql").unwrap(),
            key,
        }
    }

    /// Sets the base URL of the Linear API that the client should point to.
    pub fn base_url(mut self, base_url: &'a str) -> Result<LinearClientBuilder, ParseError> {
        self.base_url = Url::parse(base_url)?;
        Ok(self)
    }

    /// Sets the API key that the client will use.
    pub fn key(mut self, key: &'a ApiKey) -> Self {
        self.key = key;
        self
    }

    /// Consumes the builder and returns the constructed client.
    pub fn build(self) -> LinearClient {
        let client = reqwest::Client::builder()
            .user_agent(concat!("linear_sdk/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap();

        LinearClient {
            base_url: self.base_url,
            key: self.key.to_owned(),
            client,
        }
    }
}
