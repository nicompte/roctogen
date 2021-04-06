//! Method, error and parameter types for the Licenses endpoint.
#![allow(
    unused_imports,
)]
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct Licenses<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> Licenses {
    Licenses { auth }
}

/// Errors for the [Get a license](Licenses::get_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum LicensesGetError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Forbidden")]
    Status403(BasicError),
    #[error("Resource Not Found")]
    Status404(BasicError),
    #[error("Not Modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get all commonly used licenses](Licenses::get_all_commonly_used_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum LicensesGetAllCommonlyUsedError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Not Modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get the license for a repository](Licenses::get_for_repo_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum LicensesGetForRepoError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}


/// Query parameters for the [Get all commonly used licenses](Licenses::get_all_commonly_used_async()) endpoint.
#[derive(Default, Serialize)]
pub struct LicensesGetAllCommonlyUsedParams {
    
    featured: Option<bool>, 
    /// Results per page (max 100).
    per_page: Option<u16>, 
    /// Page number of the results to fetch.
    page: Option<u16>
}

impl LicensesGetAllCommonlyUsedParams {
    pub fn new() -> Self {
        Self::default()
    }

    
    pub fn featured(self, featured: bool) -> Self {
        Self { 
            featured: Some(featured),
            per_page: self.per_page, 
            page: self.page, 
        }
    }

    /// Results per page (max 100).
    pub fn per_page(self, per_page: u16) -> Self {
        Self { 
            featured: self.featured, 
            per_page: Some(per_page),
            page: self.page, 
        }
    }

    /// Page number of the results to fetch.
    pub fn page(self, page: u16) -> Self {
        Self { 
            featured: self.featured, 
            per_page: self.per_page, 
            page: Some(page),
        }
    }
}

impl<'enc> From<&'enc PerPage> for LicensesGetAllCommonlyUsedParams {
    fn from(per_page: &'enc PerPage) -> Self {
        Self {
            per_page: Some(per_page.per_page),
            page: Some(per_page.page),
            ..Default::default()
        }
    }
}

impl<'api> Licenses<'api> {
    /// ---
    ///
    /// # Get a license
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/licenses/#get-a-license)
    ///
    /// ---
    pub async fn get_async(&self, license: &str) -> Result<License, LicensesGetError> {

        let request_uri = format!("{}/licenses/{}", super::GITHUB_BASE_API_URL, license);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                403 => Err(LicensesGetError::Status403(crate::adapters::to_json_async(github_response).await?)),
                404 => Err(LicensesGetError::Status404(crate::adapters::to_json_async(github_response).await?)),
                304 => Err(LicensesGetError::Status304),
                code => Err(LicensesGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get a license
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/licenses/#get-a-license)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&self, license: &str) -> Result<License, LicensesGetError> {

        let request_uri = format!("{}/licenses/{}", super::GITHUB_BASE_API_URL, license);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                403 => Err(LicensesGetError::Status403(crate::adapters::to_json(github_response)?)),
                404 => Err(LicensesGetError::Status404(crate::adapters::to_json(github_response)?)),
                304 => Err(LicensesGetError::Status304),
                code => Err(LicensesGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get all commonly used licenses
    /// 
    /// [GitHub API docs for get_all_commonly_used](https://docs.github.com/rest/reference/licenses/#get-all-commonly-used-licenses)
    ///
    /// ---
    pub async fn get_all_commonly_used_async(&self, query_params: Option<impl Into<LicensesGetAllCommonlyUsedParams>>) -> Result<Vec<LicenseSimple>, LicensesGetAllCommonlyUsedError> {

        let mut request_uri = format!("{}/licenses", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                304 => Err(LicensesGetAllCommonlyUsedError::Status304),
                code => Err(LicensesGetAllCommonlyUsedError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get all commonly used licenses
    /// 
    /// [GitHub API docs for get_all_commonly_used](https://docs.github.com/rest/reference/licenses/#get-all-commonly-used-licenses)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_all_commonly_used(&self, query_params: Option<impl Into<LicensesGetAllCommonlyUsedParams>>) -> Result<Vec<LicenseSimple>, LicensesGetAllCommonlyUsedError> {

        let mut request_uri = format!("{}/licenses", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: LicensesGetAllCommonlyUsedParams = params.into();
            request_uri.push_str(&serde_urlencoded::to_string(qp)?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                304 => Err(LicensesGetAllCommonlyUsedError::Status304),
                code => Err(LicensesGetAllCommonlyUsedError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the license for a repository
    ///
    /// This method returns the contents of the repository's license file, if one is detected.
    /// 
    /// Similar to [Get repository content](https://docs.github.com/rest/reference/repos#get-repository-content), this method also supports [custom media types](https://docs.github.com/rest/overview/media-types) for retrieving the raw license content or rendered license HTML.
    /// 
    /// [GitHub API docs for get_for_repo](https://docs.github.com/rest/reference/licenses/#get-the-license-for-a-repository)
    ///
    /// ---
    pub async fn get_for_repo_async(&self, owner: &str, repo: &str) -> Result<LicenseContent, LicensesGetForRepoError> {

        let request_uri = format!("{}/repos/{}/{}/license", super::GITHUB_BASE_API_URL, owner, repo);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(LicensesGetForRepoError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the license for a repository
    ///
    /// This method returns the contents of the repository's license file, if one is detected.
    /// 
    /// Similar to [Get repository content](https://docs.github.com/rest/reference/repos#get-repository-content), this method also supports [custom media types](https://docs.github.com/rest/overview/media-types) for retrieving the raw license content or rendered license HTML.
    /// 
    /// [GitHub API docs for get_for_repo](https://docs.github.com/rest/reference/licenses/#get-the-license-for-a-repository)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_for_repo(&self, owner: &str, repo: &str) -> Result<LicenseContent, LicensesGetForRepoError> {

        let request_uri = format!("{}/repos/{}/{}/license", super::GITHUB_BASE_API_URL, owner, repo);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(LicensesGetForRepoError::Generic { code }),
            }
        }
    }

}
