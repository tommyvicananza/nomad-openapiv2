/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadIdentity {
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<bool>,
    #[serde(rename = "File", skip_serializing_if = "Option::is_none")]
    pub file: Option<bool>,
}

impl WorkloadIdentity {
    pub fn new() -> WorkloadIdentity {
        WorkloadIdentity {
            env: None,
            file: None,
        }
    }
}


