/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`AbstractPolicyRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "decisionStrategy": {
///      "$ref": "#/definitions/DecisionStrategy"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "logic": {
///      "$ref": "#/definitions/Logic"
///    },
///    "name": {
///      "type": "string"
///    },
///    "owner": {
///      "type": "string"
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "resourceType": {
///      "type": "string"
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "resourcesData": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ResourceRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "scopesData": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AbstractPolicyRepresentation {
    #[serde(
        rename = "decisionStrategy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub decision_strategy: ::std::option::Option<DecisionStrategy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logic: ::std::option::Option<Logic>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policies: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "resourcesData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resources_data: ::std::option::Option<Vec<ResourceRepresentation>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "scopesData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scopes_data: ::std::option::Option<Vec<ScopeRepresentation>>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AbstractPolicyRepresentation {
    fn default() -> Self {
        Self {
            decision_strategy: Default::default(),
            description: Default::default(),
            id: Default::default(),
            logic: Default::default(),
            name: Default::default(),
            owner: Default::default(),
            policies: Default::default(),
            resource_type: Default::default(),
            resources: Default::default(),
            resources_data: Default::default(),
            scopes: Default::default(),
            scopes_data: Default::default(),
            type_: Default::default(),
        }
    }
}
///`Access`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "roles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "verify_caller": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Access {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roles: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub verify_caller: ::std::option::Option<bool>,
}
impl ::std::default::Default for Access {
    fn default() -> Self {
        Self {
            roles: Default::default(),
            verify_caller: Default::default(),
        }
    }
}
///`AccessToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "acr": {
///      "type": "string"
///    },
///    "address": {
///      "$ref": "#/definitions/AddressClaimSet"
///    },
///    "allowed-origins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "at_hash": {
///      "type": "string"
///    },
///    "auth_time": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "authorization": {
///      "$ref": "#/definitions/Authorization"
///    },
///    "azp": {
///      "type": "string"
///    },
///    "birthdate": {
///      "type": "string"
///    },
///    "c_hash": {
///      "type": "string"
///    },
///    "claims_locales": {
///      "type": "string"
///    },
///    "cnf": {
///      "$ref": "#/definitions/Confirmation"
///    },
///    "email": {
///      "type": "string"
///    },
///    "email_verified": {
///      "type": "boolean"
///    },
///    "exp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "family_name": {
///      "type": "string"
///    },
///    "gender": {
///      "type": "string"
///    },
///    "given_name": {
///      "type": "string"
///    },
///    "iat": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "iss": {
///      "type": "string"
///    },
///    "jti": {
///      "type": "string"
///    },
///    "locale": {
///      "type": "string"
///    },
///    "middle_name": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "nbf": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "nickname": {
///      "type": "string"
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "otherClaims": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "phone_number": {
///      "type": "string"
///    },
///    "phone_number_verified": {
///      "type": "boolean"
///    },
///    "picture": {
///      "type": "string"
///    },
///    "preferred_username": {
///      "type": "string"
///    },
///    "profile": {
///      "type": "string"
///    },
///    "realm_access": {
///      "$ref": "#/definitions/Access"
///    },
///    "resource_access": {
///      "writeOnly": true,
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/Access"
///      }
///    },
///    "s_hash": {
///      "type": "string"
///    },
///    "scope": {
///      "type": "string"
///    },
///    "sid": {
///      "type": "string"
///    },
///    "sub": {
///      "type": "string"
///    },
///    "trusted-certs": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "typ": {
///      "type": "string"
///    },
///    "updated_at": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "website": {
///      "type": "string"
///    },
///    "zoneinfo": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessToken {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub acr: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub address: ::std::option::Option<AddressClaimSet>,
    #[serde(
        rename = "allowed-origins",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_origins: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub at_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub auth_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authorization: ::std::option::Option<Authorization>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub azp: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub birthdate: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub c_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub claims_locales: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cnf: ::std::option::Option<Confirmation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email_verified: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gender: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub given_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iat: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iss: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub jti: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub middle_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nbf: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nickname: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "otherClaims",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub other_claims: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone_number_verified: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub picture: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub preferred_username: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profile: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realm_access: ::std::option::Option<Access>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub resource_access: ::std::collections::BTreeMap<::std::string::String, Access>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub s_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sid: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "trusted-certs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trusted_certs: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub typ: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated_at: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub website: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub zoneinfo: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AccessToken {
    fn default() -> Self {
        Self {
            acr: Default::default(),
            address: Default::default(),
            allowed_origins: Default::default(),
            at_hash: Default::default(),
            auth_time: Default::default(),
            authorization: Default::default(),
            azp: Default::default(),
            birthdate: Default::default(),
            c_hash: Default::default(),
            claims_locales: Default::default(),
            cnf: Default::default(),
            email: Default::default(),
            email_verified: Default::default(),
            exp: Default::default(),
            family_name: Default::default(),
            gender: Default::default(),
            given_name: Default::default(),
            iat: Default::default(),
            iss: Default::default(),
            jti: Default::default(),
            locale: Default::default(),
            middle_name: Default::default(),
            name: Default::default(),
            nbf: Default::default(),
            nickname: Default::default(),
            nonce: Default::default(),
            other_claims: Default::default(),
            phone_number: Default::default(),
            phone_number_verified: Default::default(),
            picture: Default::default(),
            preferred_username: Default::default(),
            profile: Default::default(),
            realm_access: Default::default(),
            resource_access: Default::default(),
            s_hash: Default::default(),
            scope: Default::default(),
            sid: Default::default(),
            sub: Default::default(),
            trusted_certs: Default::default(),
            typ: Default::default(),
            updated_at: Default::default(),
            website: Default::default(),
            zoneinfo: Default::default(),
        }
    }
}
///`AddressClaimSet`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "country": {
///      "type": "string"
///    },
///    "formatted": {
///      "type": "string"
///    },
///    "locality": {
///      "type": "string"
///    },
///    "postal_code": {
///      "type": "string"
///    },
///    "region": {
///      "type": "string"
///    },
///    "street_address": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AddressClaimSet {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub formatted: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub street_address: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AddressClaimSet {
    fn default() -> Self {
        Self {
            country: Default::default(),
            formatted: Default::default(),
            locality: Default::default(),
            postal_code: Default::default(),
            region: Default::default(),
            street_address: Default::default(),
        }
    }
}
///`AdminEventRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "authDetails": {
///      "$ref": "#/definitions/AuthDetailsRepresentation"
///    },
///    "details": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "error": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "operationType": {
///      "type": "string"
///    },
///    "realmId": {
///      "type": "string"
///    },
///    "representation": {
///      "type": "string"
///    },
///    "resourcePath": {
///      "type": "string"
///    },
///    "resourceType": {
///      "type": "string"
///    },
///    "time": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AdminEventRepresentation {
    #[serde(
        rename = "authDetails",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub auth_details: ::std::option::Option<AuthDetailsRepresentation>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub details: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "operationType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub operation_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub representation: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "resourcePath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resource_path: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time: ::std::option::Option<i64>,
}
impl ::std::default::Default for AdminEventRepresentation {
    fn default() -> Self {
        Self {
            auth_details: Default::default(),
            details: Default::default(),
            error: Default::default(),
            id: Default::default(),
            operation_type: Default::default(),
            realm_id: Default::default(),
            representation: Default::default(),
            resource_path: Default::default(),
            resource_type: Default::default(),
            time: Default::default(),
        }
    }
}
///`ApplicationRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "deprecated": true,
///  "type": "object",
///  "properties": {
///    "access": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "boolean"
///      }
///    },
///    "adminUrl": {
///      "type": "string"
///    },
///    "alwaysDisplayInConsole": {
///      "type": "boolean"
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authenticationFlowBindingOverrides": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authorizationServicesEnabled": {
///      "type": "boolean"
///    },
///    "authorizationSettings": {
///      "$ref": "#/definitions/ResourceServerRepresentation"
///    },
///    "baseUrl": {
///      "type": "string"
///    },
///    "bearerOnly": {
///      "type": "boolean"
///    },
///    "claims": {
///      "deprecated": true,
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ClaimRepresentation"
///        }
///      ]
///    },
///    "clientAuthenticatorType": {
///      "type": "string"
///    },
///    "clientId": {
///      "type": "string"
///    },
///    "clientTemplate": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "consentRequired": {
///      "type": "boolean"
///    },
///    "defaultClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultRoles": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "directAccessGrantsEnabled": {
///      "type": "boolean"
///    },
///    "directGrantsOnly": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "frontchannelLogout": {
///      "type": "boolean"
///    },
///    "fullScopeAllowed": {
///      "type": "boolean"
///    },
///    "id": {
///      "type": "string"
///    },
///    "implicitFlowEnabled": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "nodeReRegistrationTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "optionalClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "origin": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProtocolMapperRepresentation"
///      }
///    },
///    "publicClient": {
///      "type": "boolean"
///    },
///    "redirectUris": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "registeredNodes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "int32"
///      }
///    },
///    "registrationAccessToken": {
///      "type": "string"
///    },
///    "rootUrl": {
///      "type": "string"
///    },
///    "secret": {
///      "type": "string"
///    },
///    "serviceAccountsEnabled": {
///      "type": "boolean"
///    },
///    "standardFlowEnabled": {
///      "type": "boolean"
///    },
///    "surrogateAuthRequired": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    },
///    "useTemplateConfig": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "useTemplateMappers": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "useTemplateScope": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "webOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ApplicationRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    #[serde(
        rename = "adminUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "alwaysDisplayInConsole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub always_display_in_console: ::std::option::Option<bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authenticationFlowBindingOverrides",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub authentication_flow_binding_overrides: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authorizationServicesEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_services_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "authorizationSettings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_settings: ::std::option::Option<ResourceServerRepresentation>,
    #[serde(
        rename = "baseUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "bearerOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub claims: ::std::option::Option<ClaimRepresentation>,
    #[serde(
        rename = "clientAuthenticatorType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_authenticator_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_template: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "consentRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub consent_required: ::std::option::Option<bool>,
    #[serde(
        rename = "defaultClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "directAccessGrantsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_access_grants_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "directGrantsOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_grants_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "frontchannelLogout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frontchannel_logout: ::std::option::Option<bool>,
    #[serde(
        rename = "fullScopeAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_scope_allowed: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "implicitFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub implicit_flow_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "nodeReRegistrationTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub node_re_registration_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "notBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub not_before: ::std::option::Option<i32>,
    #[serde(
        rename = "optionalClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub optional_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
    #[serde(
        rename = "publicClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_client: ::std::option::Option<bool>,
    #[serde(
        rename = "redirectUris",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub redirect_uris: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "registeredNodes",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub registered_nodes: ::std::collections::BTreeMap<::std::string::String, i32>,
    #[serde(
        rename = "registrationAccessToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_access_token: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rootUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub root_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "serviceAccountsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_accounts_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "standardFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub standard_flow_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "surrogateAuthRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surrogate_auth_required: ::std::option::Option<bool>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "useTemplateConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_config: ::std::option::Option<bool>,
    #[serde(
        rename = "useTemplateMappers",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_mappers: ::std::option::Option<bool>,
    #[serde(
        rename = "useTemplateScope",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_scope: ::std::option::Option<bool>,
    #[serde(
        rename = "webOrigins",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_origins: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for ApplicationRepresentation {
    fn default() -> Self {
        Self {
            access: Default::default(),
            admin_url: Default::default(),
            always_display_in_console: Default::default(),
            attributes: Default::default(),
            authentication_flow_binding_overrides: Default::default(),
            authorization_services_enabled: Default::default(),
            authorization_settings: Default::default(),
            base_url: Default::default(),
            bearer_only: Default::default(),
            claims: Default::default(),
            client_authenticator_type: Default::default(),
            client_id: Default::default(),
            client_template: Default::default(),
            consent_required: Default::default(),
            default_client_scopes: Default::default(),
            default_roles: Default::default(),
            description: Default::default(),
            direct_access_grants_enabled: Default::default(),
            direct_grants_only: Default::default(),
            enabled: Default::default(),
            frontchannel_logout: Default::default(),
            full_scope_allowed: Default::default(),
            id: Default::default(),
            implicit_flow_enabled: Default::default(),
            name: Default::default(),
            node_re_registration_timeout: Default::default(),
            not_before: Default::default(),
            optional_client_scopes: Default::default(),
            origin: Default::default(),
            protocol: Default::default(),
            protocol_mappers: Default::default(),
            public_client: Default::default(),
            redirect_uris: Default::default(),
            registered_nodes: Default::default(),
            registration_access_token: Default::default(),
            root_url: Default::default(),
            secret: Default::default(),
            service_accounts_enabled: Default::default(),
            standard_flow_enabled: Default::default(),
            surrogate_auth_required: Default::default(),
            type_: Default::default(),
            use_template_config: Default::default(),
            use_template_mappers: Default::default(),
            use_template_scope: Default::default(),
            web_origins: Default::default(),
        }
    }
}
///`AuthDetailsRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "clientId": {
///      "type": "string"
///    },
///    "ipAddress": {
///      "type": "string"
///    },
///    "realmId": {
///      "type": "string"
///    },
///    "userId": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthDetailsRepresentation {
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ipAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "userId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AuthDetailsRepresentation {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            ip_address: Default::default(),
            realm_id: Default::default(),
            user_id: Default::default(),
        }
    }
}
///`AuthenticationExecutionExportRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "authenticator": {
///      "type": "string"
///    },
///    "authenticatorConfig": {
///      "type": "string"
///    },
///    "authenticatorFlow": {
///      "type": "boolean"
///    },
///    "autheticatorFlow": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "flowAlias": {
///      "type": "string"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "requirement": {
///      "type": "string"
///    },
///    "userSetupAllowed": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthenticationExecutionExportRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authenticator: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticatorConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authenticator_config: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticatorFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authenticator_flow: ::std::option::Option<bool>,
    #[serde(
        rename = "autheticatorFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub autheticator_flow: ::std::option::Option<bool>,
    #[serde(
        rename = "flowAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub flow_alias: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub requirement: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "userSetupAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_setup_allowed: ::std::option::Option<bool>,
}
impl ::std::default::Default for AuthenticationExecutionExportRepresentation {
    fn default() -> Self {
        Self {
            authenticator: Default::default(),
            authenticator_config: Default::default(),
            authenticator_flow: Default::default(),
            autheticator_flow: Default::default(),
            flow_alias: Default::default(),
            priority: Default::default(),
            requirement: Default::default(),
            user_setup_allowed: Default::default(),
        }
    }
}
///`AuthenticationExecutionInfoRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alias": {
///      "type": "string"
///    },
///    "authenticationConfig": {
///      "type": "string"
///    },
///    "authenticationFlow": {
///      "type": "boolean"
///    },
///    "configurable": {
///      "type": "boolean"
///    },
///    "description": {
///      "type": "string"
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "flowId": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "index": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "level": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "requirement": {
///      "type": "string"
///    },
///    "requirementChoices": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthenticationExecutionInfoRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticationConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authentication_config: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticationFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authentication_flow: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub configurable: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "flowId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub flow_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<i32>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub requirement: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "requirementChoices",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub requirement_choices: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for AuthenticationExecutionInfoRepresentation {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            authentication_config: Default::default(),
            authentication_flow: Default::default(),
            configurable: Default::default(),
            description: Default::default(),
            display_name: Default::default(),
            flow_id: Default::default(),
            id: Default::default(),
            index: Default::default(),
            level: Default::default(),
            priority: Default::default(),
            provider_id: Default::default(),
            requirement: Default::default(),
            requirement_choices: Default::default(),
        }
    }
}
///`AuthenticationExecutionRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "authenticator": {
///      "type": "string"
///    },
///    "authenticatorConfig": {
///      "type": "string"
///    },
///    "authenticatorFlow": {
///      "type": "boolean"
///    },
///    "autheticatorFlow": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "flowId": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "parentFlow": {
///      "type": "string"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "requirement": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthenticationExecutionRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authenticator: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticatorConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authenticator_config: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticatorFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authenticator_flow: ::std::option::Option<bool>,
    #[serde(
        rename = "autheticatorFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub autheticator_flow: ::std::option::Option<bool>,
    #[serde(
        rename = "flowId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub flow_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "parentFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_flow: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub requirement: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AuthenticationExecutionRepresentation {
    fn default() -> Self {
        Self {
            authenticator: Default::default(),
            authenticator_config: Default::default(),
            authenticator_flow: Default::default(),
            autheticator_flow: Default::default(),
            flow_id: Default::default(),
            id: Default::default(),
            parent_flow: Default::default(),
            priority: Default::default(),
            requirement: Default::default(),
        }
    }
}
///`AuthenticationFlowRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alias": {
///      "type": "string"
///    },
///    "authenticationExecutions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AuthenticationExecutionExportRepresentation"
///      }
///    },
///    "builtIn": {
///      "type": "boolean"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "topLevel": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct AuthenticationFlowRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticationExecutions",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub authentication_executions: ::std::vec::Vec<
        AuthenticationExecutionExportRepresentation,
    >,
    #[serde(
        rename = "builtIn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub built_in: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "topLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub top_level: ::std::option::Option<bool>,
}
impl ::std::default::Default for AuthenticationFlowRepresentation {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            authentication_executions: Default::default(),
            built_in: Default::default(),
            description: Default::default(),
            id: Default::default(),
            provider_id: Default::default(),
            top_level: Default::default(),
        }
    }
}
///`AuthenticatorConfigInfoRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "helpText": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "properties": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ConfigPropertyRepresentation"
///      }
///    },
///    "providerId": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthenticatorConfigInfoRepresentation {
    #[serde(
        rename = "helpText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub help_text: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub properties: ::std::vec::Vec<ConfigPropertyRepresentation>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AuthenticatorConfigInfoRepresentation {
    fn default() -> Self {
        Self {
            help_text: Default::default(),
            name: Default::default(),
            properties: Default::default(),
            provider_id: Default::default(),
        }
    }
}
///`AuthenticatorConfigRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alias": {
///      "type": "string"
///    },
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct AuthenticatorConfigRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AuthenticatorConfigRepresentation {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            config: Default::default(),
            id: Default::default(),
        }
    }
}
///`Authorization`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "permissions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Permission"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Authorization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: ::std::vec::Vec<Permission>,
}
impl ::std::default::Default for Authorization {
    fn default() -> Self {
        Self {
            permissions: Default::default(),
        }
    }
}
///`AuthorizationSchema`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "resourceTypes": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/ResourceType"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthorizationSchema {
    #[serde(
        rename = "resourceTypes",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub resource_types: ::std::collections::BTreeMap<
        ::std::string::String,
        ResourceType,
    >,
}
impl ::std::default::Default for AuthorizationSchema {
    fn default() -> Self {
        Self {
            resource_types: Default::default(),
        }
    }
}
///`BruteForceStrategy`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "LINEAR",
///    "MULTIPLE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum BruteForceStrategy {
    #[serde(rename = "LINEAR")]
    Linear,
    #[serde(rename = "MULTIPLE")]
    Multiple,
}
impl ::std::fmt::Display for BruteForceStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Linear => f.write_str("LINEAR"),
            Self::Multiple => f.write_str("MULTIPLE"),
        }
    }
}
impl ::std::str::FromStr for BruteForceStrategy {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LINEAR" => Ok(Self::Linear),
            "MULTIPLE" => Ok(Self::Multiple),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BruteForceStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BruteForceStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BruteForceStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CertificateRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "certificate": {
///      "type": "string"
///    },
///    "kid": {
///      "type": "string"
///    },
///    "privateKey": {
///      "type": "string"
///    },
///    "publicKey": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CertificateRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kid: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "privateKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub private_key: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "publicKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_key: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CertificateRepresentation {
    fn default() -> Self {
        Self {
            certificate: Default::default(),
            kid: Default::default(),
            private_key: Default::default(),
            public_key: Default::default(),
        }
    }
}
///`ClaimRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "address": {
///      "type": "boolean"
///    },
///    "email": {
///      "type": "boolean"
///    },
///    "gender": {
///      "type": "boolean"
///    },
///    "locale": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "boolean"
///    },
///    "phone": {
///      "type": "boolean"
///    },
///    "picture": {
///      "type": "boolean"
///    },
///    "profile": {
///      "type": "boolean"
///    },
///    "username": {
///      "type": "boolean"
///    },
///    "website": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClaimRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub address: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gender: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub locale: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub picture: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profile: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub website: ::std::option::Option<bool>,
}
impl ::std::default::Default for ClaimRepresentation {
    fn default() -> Self {
        Self {
            address: Default::default(),
            email: Default::default(),
            gender: Default::default(),
            locale: Default::default(),
            name: Default::default(),
            phone: Default::default(),
            picture: Default::default(),
            profile: Default::default(),
            username: Default::default(),
            website: Default::default(),
        }
    }
}
///`ClientInitialAccessCreatePresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "count": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "expiration": {
///      "type": "integer",
///      "format": "int32"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientInitialAccessCreatePresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expiration: ::std::option::Option<i32>,
}
impl ::std::default::Default for ClientInitialAccessCreatePresentation {
    fn default() -> Self {
        Self {
            count: Default::default(),
            expiration: Default::default(),
        }
    }
}
///`ClientInitialAccessPresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "count": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "expiration": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "id": {
///      "type": "string"
///    },
///    "remainingCount": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "token": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientInitialAccessPresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expiration: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "remainingCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remaining_count: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ClientInitialAccessPresentation {
    fn default() -> Self {
        Self {
            count: Default::default(),
            expiration: Default::default(),
            id: Default::default(),
            remaining_count: Default::default(),
            timestamp: Default::default(),
            token: Default::default(),
        }
    }
}
///`ClientMappingsRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "client": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "mappings": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RoleRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientMappingsRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mappings: ::std::vec::Vec<RoleRepresentation>,
}
impl ::std::default::Default for ClientMappingsRepresentation {
    fn default() -> Self {
        Self {
            client: Default::default(),
            id: Default::default(),
            mappings: Default::default(),
        }
    }
}
///`ClientPoliciesRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "globalPolicies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientPolicyRepresentation"
///      }
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientPolicyRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientPoliciesRepresentation {
    #[serde(
        rename = "globalPolicies",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_policies: ::std::vec::Vec<ClientPolicyRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub policies: ::std::vec::Vec<ClientPolicyRepresentation>,
}
impl ::std::default::Default for ClientPoliciesRepresentation {
    fn default() -> Self {
        Self {
            global_policies: Default::default(),
            policies: Default::default(),
        }
    }
}
///`ClientPolicyConditionRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "condition": {
///      "type": "string"
///    },
///    "configuration": {
///      "type": "array"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientPolicyConditionRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configuration: ::std::vec::Vec<::serde_json::Value>,
}
impl ::std::default::Default for ClientPolicyConditionRepresentation {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            configuration: Default::default(),
        }
    }
}
///`ClientPolicyExecutorRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "configuration": {
///      "type": "array"
///    },
///    "executor": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientPolicyExecutorRepresentation {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configuration: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub executor: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ClientPolicyExecutorRepresentation {
    fn default() -> Self {
        Self {
            configuration: Default::default(),
            executor: Default::default(),
        }
    }
}
///`ClientPolicyRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "conditions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientPolicyConditionRepresentation"
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "profiles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientPolicyRepresentation {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub conditions: ::std::vec::Vec<ClientPolicyConditionRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub profiles: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for ClientPolicyRepresentation {
    fn default() -> Self {
        Self {
            conditions: Default::default(),
            description: Default::default(),
            enabled: Default::default(),
            name: Default::default(),
            profiles: Default::default(),
        }
    }
}
///`ClientProfileRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "executors": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientPolicyExecutorRepresentation"
///      }
///    },
///    "name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientProfileRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub executors: ::std::vec::Vec<ClientPolicyExecutorRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ClientProfileRepresentation {
    fn default() -> Self {
        Self {
            description: Default::default(),
            executors: Default::default(),
            name: Default::default(),
        }
    }
}
///`ClientProfilesRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "globalProfiles": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientProfileRepresentation"
///      }
///    },
///    "profiles": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientProfileRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientProfilesRepresentation {
    #[serde(
        rename = "globalProfiles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_profiles: ::std::vec::Vec<ClientProfileRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub profiles: ::std::vec::Vec<ClientProfileRepresentation>,
}
impl ::std::default::Default for ClientProfilesRepresentation {
    fn default() -> Self {
        Self {
            global_profiles: Default::default(),
            profiles: Default::default(),
        }
    }
}
///`ClientRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "access": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "boolean"
///      }
///    },
///    "adminUrl": {
///      "type": "string"
///    },
///    "alwaysDisplayInConsole": {
///      "type": "boolean"
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authenticationFlowBindingOverrides": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authorizationServicesEnabled": {
///      "type": "boolean"
///    },
///    "authorizationSettings": {
///      "$ref": "#/definitions/ResourceServerRepresentation"
///    },
///    "baseUrl": {
///      "type": "string"
///    },
///    "bearerOnly": {
///      "type": "boolean"
///    },
///    "clientAuthenticatorType": {
///      "type": "string"
///    },
///    "clientId": {
///      "type": "string"
///    },
///    "clientTemplate": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "consentRequired": {
///      "type": "boolean"
///    },
///    "defaultClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultRoles": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "directAccessGrantsEnabled": {
///      "type": "boolean"
///    },
///    "directGrantsOnly": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "frontchannelLogout": {
///      "type": "boolean"
///    },
///    "fullScopeAllowed": {
///      "type": "boolean"
///    },
///    "id": {
///      "type": "string"
///    },
///    "implicitFlowEnabled": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "nodeReRegistrationTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "optionalClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "origin": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProtocolMapperRepresentation"
///      }
///    },
///    "publicClient": {
///      "type": "boolean"
///    },
///    "redirectUris": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "registeredNodes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "int32"
///      }
///    },
///    "registrationAccessToken": {
///      "type": "string"
///    },
///    "rootUrl": {
///      "type": "string"
///    },
///    "secret": {
///      "type": "string"
///    },
///    "serviceAccountsEnabled": {
///      "type": "boolean"
///    },
///    "standardFlowEnabled": {
///      "type": "boolean"
///    },
///    "surrogateAuthRequired": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    },
///    "useTemplateConfig": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "useTemplateMappers": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "useTemplateScope": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "webOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ClientRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    #[serde(
        rename = "adminUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "alwaysDisplayInConsole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub always_display_in_console: ::std::option::Option<bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authenticationFlowBindingOverrides",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub authentication_flow_binding_overrides: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authorizationServicesEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_services_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "authorizationSettings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_settings: ::std::option::Option<ResourceServerRepresentation>,
    #[serde(
        rename = "baseUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "bearerOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_only: ::std::option::Option<bool>,
    #[serde(
        rename = "clientAuthenticatorType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_authenticator_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_template: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "consentRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub consent_required: ::std::option::Option<bool>,
    #[serde(
        rename = "defaultClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "directAccessGrantsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_access_grants_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "directGrantsOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_grants_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "frontchannelLogout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frontchannel_logout: ::std::option::Option<bool>,
    #[serde(
        rename = "fullScopeAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_scope_allowed: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "implicitFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub implicit_flow_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "nodeReRegistrationTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub node_re_registration_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "notBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub not_before: ::std::option::Option<i32>,
    #[serde(
        rename = "optionalClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub optional_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
    #[serde(
        rename = "publicClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_client: ::std::option::Option<bool>,
    #[serde(
        rename = "redirectUris",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub redirect_uris: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "registeredNodes",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub registered_nodes: ::std::collections::BTreeMap<::std::string::String, i32>,
    #[serde(
        rename = "registrationAccessToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_access_token: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rootUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub root_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "serviceAccountsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_accounts_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "standardFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub standard_flow_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "surrogateAuthRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surrogate_auth_required: ::std::option::Option<bool>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "useTemplateConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_config: ::std::option::Option<bool>,
    #[serde(
        rename = "useTemplateMappers",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_mappers: ::std::option::Option<bool>,
    #[serde(
        rename = "useTemplateScope",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_scope: ::std::option::Option<bool>,
    #[serde(
        rename = "webOrigins",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_origins: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for ClientRepresentation {
    fn default() -> Self {
        Self {
            access: Default::default(),
            admin_url: Default::default(),
            always_display_in_console: Default::default(),
            attributes: Default::default(),
            authentication_flow_binding_overrides: Default::default(),
            authorization_services_enabled: Default::default(),
            authorization_settings: Default::default(),
            base_url: Default::default(),
            bearer_only: Default::default(),
            client_authenticator_type: Default::default(),
            client_id: Default::default(),
            client_template: Default::default(),
            consent_required: Default::default(),
            default_client_scopes: Default::default(),
            default_roles: Default::default(),
            description: Default::default(),
            direct_access_grants_enabled: Default::default(),
            direct_grants_only: Default::default(),
            enabled: Default::default(),
            frontchannel_logout: Default::default(),
            full_scope_allowed: Default::default(),
            id: Default::default(),
            implicit_flow_enabled: Default::default(),
            name: Default::default(),
            node_re_registration_timeout: Default::default(),
            not_before: Default::default(),
            optional_client_scopes: Default::default(),
            origin: Default::default(),
            protocol: Default::default(),
            protocol_mappers: Default::default(),
            public_client: Default::default(),
            redirect_uris: Default::default(),
            registered_nodes: Default::default(),
            registration_access_token: Default::default(),
            root_url: Default::default(),
            secret: Default::default(),
            service_accounts_enabled: Default::default(),
            standard_flow_enabled: Default::default(),
            surrogate_auth_required: Default::default(),
            type_: Default::default(),
            use_template_config: Default::default(),
            use_template_mappers: Default::default(),
            use_template_scope: Default::default(),
            web_origins: Default::default(),
        }
    }
}
///`ClientScopeRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProtocolMapperRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ClientScopeRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
}
impl ::std::default::Default for ClientScopeRepresentation {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            description: Default::default(),
            id: Default::default(),
            name: Default::default(),
            protocol: Default::default(),
            protocol_mappers: Default::default(),
        }
    }
}
///`ClientTemplateRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "deprecated": true,
///  "type": "object",
///  "properties": {
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "bearerOnly": {
///      "type": "boolean"
///    },
///    "consentRequired": {
///      "type": "boolean"
///    },
///    "description": {
///      "type": "string"
///    },
///    "directAccessGrantsEnabled": {
///      "type": "boolean"
///    },
///    "frontchannelLogout": {
///      "type": "boolean"
///    },
///    "fullScopeAllowed": {
///      "type": "boolean"
///    },
///    "id": {
///      "type": "string"
///    },
///    "implicitFlowEnabled": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProtocolMapperRepresentation"
///      }
///    },
///    "publicClient": {
///      "type": "boolean"
///    },
///    "serviceAccountsEnabled": {
///      "type": "boolean"
///    },
///    "standardFlowEnabled": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientTemplateRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "bearerOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_only: ::std::option::Option<bool>,
    #[serde(
        rename = "consentRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub consent_required: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "directAccessGrantsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_access_grants_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "frontchannelLogout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frontchannel_logout: ::std::option::Option<bool>,
    #[serde(
        rename = "fullScopeAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_scope_allowed: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "implicitFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub implicit_flow_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
    #[serde(
        rename = "publicClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_client: ::std::option::Option<bool>,
    #[serde(
        rename = "serviceAccountsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_accounts_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "standardFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub standard_flow_enabled: ::std::option::Option<bool>,
}
impl ::std::default::Default for ClientTemplateRepresentation {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            bearer_only: Default::default(),
            consent_required: Default::default(),
            description: Default::default(),
            direct_access_grants_enabled: Default::default(),
            frontchannel_logout: Default::default(),
            full_scope_allowed: Default::default(),
            id: Default::default(),
            implicit_flow_enabled: Default::default(),
            name: Default::default(),
            protocol: Default::default(),
            protocol_mappers: Default::default(),
            public_client: Default::default(),
            service_accounts_enabled: Default::default(),
            standard_flow_enabled: Default::default(),
        }
    }
}
///`ClientTypeRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/PropertyConfig"
///      }
///    },
///    "name": {
///      "type": "string"
///    },
///    "parent": {
///      "type": "string"
///    },
///    "provider": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientTypeRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<::std::string::String, PropertyConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ClientTypeRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            name: Default::default(),
            parent: Default::default(),
            provider: Default::default(),
        }
    }
}
///`ClientTypesRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "client-types": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientTypeRepresentation"
///      }
///    },
///    "global-client-types": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientTypeRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientTypesRepresentation {
    #[serde(
        rename = "client-types",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub client_types: ::std::vec::Vec<ClientTypeRepresentation>,
    #[serde(
        rename = "global-client-types",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_client_types: ::std::vec::Vec<ClientTypeRepresentation>,
}
impl ::std::default::Default for ClientTypesRepresentation {
    fn default() -> Self {
        Self {
            client_types: Default::default(),
            global_client_types: Default::default(),
        }
    }
}
///`ComponentExportRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "$ref": "#/definitions/MultivaluedHashMapStringString"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "subComponents": {
///      "$ref": "#/definitions/MultivaluedHashMapStringComponentExportRepresentation"
///    },
///    "subType": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ComponentExportRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<MultivaluedHashMapStringString>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "subComponents",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_components: ::std::option::Option<
        MultivaluedHashMapStringComponentExportRepresentation,
    >,
    #[serde(
        rename = "subType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_type: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ComponentExportRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            id: Default::default(),
            name: Default::default(),
            provider_id: Default::default(),
            sub_components: Default::default(),
            sub_type: Default::default(),
        }
    }
}
///`ComponentRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "$ref": "#/definitions/MultivaluedHashMapStringString"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "parentId": {
///      "type": "string"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "providerType": {
///      "type": "string"
///    },
///    "subType": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ComponentRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<MultivaluedHashMapStringString>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "parentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "subType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_type: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ComponentRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            id: Default::default(),
            name: Default::default(),
            parent_id: Default::default(),
            provider_id: Default::default(),
            provider_type: Default::default(),
            sub_type: Default::default(),
        }
    }
}
///`ComponentTypeRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "helpText": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "metadata": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "properties": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ConfigPropertyRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ComponentTypeRepresentation {
    #[serde(
        rename = "helpText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub help_text: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub properties: ::std::vec::Vec<ConfigPropertyRepresentation>,
}
impl ::std::default::Default for ComponentTypeRepresentation {
    fn default() -> Self {
        Self {
            help_text: Default::default(),
            id: Default::default(),
            metadata: Default::default(),
            properties: Default::default(),
        }
    }
}
///`Composites`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "application": {
///      "deprecated": true,
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "client": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "realm": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Composites {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub application: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realm: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::default::Default for Composites {
    fn default() -> Self {
        Self {
            application: Default::default(),
            client: Default::default(),
            realm: Default::default(),
        }
    }
}
///`ConfigPropertyRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "defaultValue": {},
///    "helpText": {
///      "type": "string"
///    },
///    "label": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "options": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "readOnly": {
///      "type": "boolean"
///    },
///    "required": {
///      "type": "boolean"
///    },
///    "secret": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ConfigPropertyRepresentation {
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_value: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "helpText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub help_text: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub options: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "readOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub read_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<bool>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ConfigPropertyRepresentation {
    fn default() -> Self {
        Self {
            default_value: Default::default(),
            help_text: Default::default(),
            label: Default::default(),
            name: Default::default(),
            options: Default::default(),
            read_only: Default::default(),
            required: Default::default(),
            secret: Default::default(),
            type_: Default::default(),
        }
    }
}
///`Confirmation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "jkt": {
///      "type": "string"
///    },
///    "x5t#S256": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Confirmation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub jkt: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "x5t#S256",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x5t_s256: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Confirmation {
    fn default() -> Self {
        Self {
            jkt: Default::default(),
            x5t_s256: Default::default(),
        }
    }
}
///`CredentialRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "algorithm": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "config": {
///      "deprecated": true,
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/definitions/MultivaluedHashMapStringString"
///        }
///      ]
///    },
///    "counter": {
///      "deprecated": true,
///      "type": "integer",
///      "format": "int32"
///    },
///    "createdDate": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "credentialData": {
///      "type": "string"
///    },
///    "device": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "digits": {
///      "deprecated": true,
///      "type": "integer",
///      "format": "int32"
///    },
///    "federationLink": {
///      "type": "string"
///    },
///    "hashIterations": {
///      "deprecated": true,
///      "type": "integer",
///      "format": "int32"
///    },
///    "hashedSaltedValue": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "period": {
///      "deprecated": true,
///      "type": "integer",
///      "format": "int32"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "salt": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "secretData": {
///      "type": "string"
///    },
///    "temporary": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    },
///    "userLabel": {
///      "type": "string"
///    },
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CredentialRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<MultivaluedHashMapStringString>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub counter: ::std::option::Option<i32>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_date: ::std::option::Option<i64>,
    #[serde(
        rename = "credentialData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub credential_data: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub device: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub digits: ::std::option::Option<i32>,
    #[serde(
        rename = "federationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation_link: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hashIterations",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hash_iterations: ::std::option::Option<i32>,
    #[serde(
        rename = "hashedSaltedValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hashed_salted_value: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub salt: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "secretData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub secret_data: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub temporary: ::std::option::Option<bool>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "userLabel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CredentialRepresentation {
    fn default() -> Self {
        Self {
            algorithm: Default::default(),
            config: Default::default(),
            counter: Default::default(),
            created_date: Default::default(),
            credential_data: Default::default(),
            device: Default::default(),
            digits: Default::default(),
            federation_link: Default::default(),
            hash_iterations: Default::default(),
            hashed_salted_value: Default::default(),
            id: Default::default(),
            period: Default::default(),
            priority: Default::default(),
            salt: Default::default(),
            secret_data: Default::default(),
            temporary: Default::default(),
            type_: Default::default(),
            user_label: Default::default(),
            value: Default::default(),
        }
    }
}
///`DecisionEffect`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "PERMIT",
///    "DENY"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum DecisionEffect {
    #[serde(rename = "PERMIT")]
    Permit,
    #[serde(rename = "DENY")]
    Deny,
}
impl ::std::fmt::Display for DecisionEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Permit => f.write_str("PERMIT"),
            Self::Deny => f.write_str("DENY"),
        }
    }
}
impl ::std::str::FromStr for DecisionEffect {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PERMIT" => Ok(Self::Permit),
            "DENY" => Ok(Self::Deny),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DecisionStrategy`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "AFFIRMATIVE",
///    "UNANIMOUS",
///    "CONSENSUS"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum DecisionStrategy {
    #[serde(rename = "AFFIRMATIVE")]
    Affirmative,
    #[serde(rename = "UNANIMOUS")]
    Unanimous,
    #[serde(rename = "CONSENSUS")]
    Consensus,
}
impl ::std::fmt::Display for DecisionStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Affirmative => f.write_str("AFFIRMATIVE"),
            Self::Unanimous => f.write_str("UNANIMOUS"),
            Self::Consensus => f.write_str("CONSENSUS"),
        }
    }
}
impl ::std::str::FromStr for DecisionStrategy {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "AFFIRMATIVE" => Ok(Self::Affirmative),
            "UNANIMOUS" => Ok(Self::Unanimous),
            "CONSENSUS" => Ok(Self::Consensus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EnforcementMode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "PERMISSIVE",
///    "ENFORCING",
///    "DISABLED"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum EnforcementMode {
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "ENFORCING")]
    Enforcing,
    #[serde(rename = "DISABLED")]
    Disabled,
}
impl ::std::fmt::Display for EnforcementMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Permissive => f.write_str("PERMISSIVE"),
            Self::Enforcing => f.write_str("ENFORCING"),
            Self::Disabled => f.write_str("DISABLED"),
        }
    }
}
impl ::std::str::FromStr for EnforcementMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PERMISSIVE" => Ok(Self::Permissive),
            "ENFORCING" => Ok(Self::Enforcing),
            "DISABLED" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ErrorRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "errorMessage": {
///      "type": "string"
///    },
///    "errors": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ErrorRepresentation"
///      }
///    },
///    "field": {
///      "type": "string"
///    },
///    "params": {
///      "type": "array",
///      "items": {}
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ErrorRepresentation {
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub errors: ::std::vec::Vec<ErrorRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub field: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub params: ::std::vec::Vec<::serde_json::Value>,
}
impl ::std::default::Default for ErrorRepresentation {
    fn default() -> Self {
        Self {
            error_message: Default::default(),
            errors: Default::default(),
            field: Default::default(),
            params: Default::default(),
        }
    }
}
///`EvaluationResultRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "allowedScopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "deniedScopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PolicyResultRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "resource": {
///      "$ref": "#/definitions/ResourceRepresentation"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      }
///    },
///    "status": {
///      "$ref": "#/definitions/DecisionEffect"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct EvaluationResultRepresentation {
    #[serde(
        rename = "allowedScopes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_scopes: ::std::option::Option<Vec<ScopeRepresentation>>,
    #[serde(
        rename = "deniedScopes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub denied_scopes: ::std::option::Option<Vec<ScopeRepresentation>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policies: ::std::option::Option<Vec<PolicyResultRepresentation>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resource: ::std::option::Option<ResourceRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<ScopeRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<DecisionEffect>,
}
impl ::std::default::Default for EvaluationResultRepresentation {
    fn default() -> Self {
        Self {
            allowed_scopes: Default::default(),
            denied_scopes: Default::default(),
            policies: Default::default(),
            resource: Default::default(),
            scopes: Default::default(),
            status: Default::default(),
        }
    }
}
///`EventRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "clientId": {
///      "type": "string"
///    },
///    "details": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "error": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "ipAddress": {
///      "type": "string"
///    },
///    "realmId": {
///      "type": "string"
///    },
///    "sessionId": {
///      "type": "string"
///    },
///    "time": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "type": {
///      "type": "string"
///    },
///    "userId": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct EventRepresentation {
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub details: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ipAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time: ::std::option::Option<i64>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "userId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for EventRepresentation {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            details: Default::default(),
            error: Default::default(),
            id: Default::default(),
            ip_address: Default::default(),
            realm_id: Default::default(),
            session_id: Default::default(),
            time: Default::default(),
            type_: Default::default(),
            user_id: Default::default(),
        }
    }
}
///`FederatedIdentityRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "identityProvider": {
///      "type": "string"
///    },
///    "userId": {
///      "type": "string"
///    },
///    "userName": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FederatedIdentityRepresentation {
    #[serde(
        rename = "identityProvider",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub identity_provider: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "userId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "userName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for FederatedIdentityRepresentation {
    fn default() -> Self {
        Self {
            identity_provider: Default::default(),
            user_id: Default::default(),
            user_name: Default::default(),
        }
    }
}
///`GlobalRequestResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "failedRequests": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "successRequests": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GlobalRequestResult {
    #[serde(
        rename = "failedRequests",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub failed_requests: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "successRequests",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub success_requests: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for GlobalRequestResult {
    fn default() -> Self {
        Self {
            failed_requests: Default::default(),
            success_requests: Default::default(),
        }
    }
}
///`GroupRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "access": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "boolean"
///      }
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "clientRoles": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "parentId": {
///      "type": "string"
///    },
///    "path": {
///      "type": "string"
///    },
///    "realmRoles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "subGroupCount": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "subGroups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/GroupRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct GroupRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "clientRoles",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client_roles: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "parentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub realm_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "subGroupCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_group_count: ::std::option::Option<i64>,
    #[serde(
        rename = "subGroups",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sub_groups: ::std::vec::Vec<GroupRepresentation>,
}
impl ::std::default::Default for GroupRepresentation {
    fn default() -> Self {
        Self {
            access: Default::default(),
            attributes: Default::default(),
            client_roles: Default::default(),
            id: Default::default(),
            name: Default::default(),
            parent_id: Default::default(),
            path: Default::default(),
            realm_roles: Default::default(),
            sub_group_count: Default::default(),
            sub_groups: Default::default(),
        }
    }
}
///`IdToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "acr": {
///      "type": "string"
///    },
///    "address": {
///      "$ref": "#/definitions/AddressClaimSet"
///    },
///    "at_hash": {
///      "type": "string"
///    },
///    "auth_time": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "azp": {
///      "type": "string"
///    },
///    "birthdate": {
///      "type": "string"
///    },
///    "c_hash": {
///      "type": "string"
///    },
///    "claims_locales": {
///      "type": "string"
///    },
///    "email": {
///      "type": "string"
///    },
///    "email_verified": {
///      "type": "boolean"
///    },
///    "exp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "family_name": {
///      "type": "string"
///    },
///    "gender": {
///      "type": "string"
///    },
///    "given_name": {
///      "type": "string"
///    },
///    "iat": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "iss": {
///      "type": "string"
///    },
///    "jti": {
///      "type": "string"
///    },
///    "locale": {
///      "type": "string"
///    },
///    "middle_name": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "nbf": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "nickname": {
///      "type": "string"
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "otherClaims": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "phone_number": {
///      "type": "string"
///    },
///    "phone_number_verified": {
///      "type": "boolean"
///    },
///    "picture": {
///      "type": "string"
///    },
///    "preferred_username": {
///      "type": "string"
///    },
///    "profile": {
///      "type": "string"
///    },
///    "s_hash": {
///      "type": "string"
///    },
///    "sid": {
///      "type": "string"
///    },
///    "sub": {
///      "type": "string"
///    },
///    "typ": {
///      "type": "string"
///    },
///    "updated_at": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "website": {
///      "type": "string"
///    },
///    "zoneinfo": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct IdToken {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub acr: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub address: ::std::option::Option<AddressClaimSet>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub at_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub auth_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub azp: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub birthdate: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub c_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub claims_locales: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email_verified: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gender: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub given_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iat: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iss: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub jti: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub middle_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nbf: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nickname: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "otherClaims",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub other_claims: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone_number_verified: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub picture: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub preferred_username: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profile: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub s_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sid: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub typ: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated_at: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub website: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub zoneinfo: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for IdToken {
    fn default() -> Self {
        Self {
            acr: Default::default(),
            address: Default::default(),
            at_hash: Default::default(),
            auth_time: Default::default(),
            azp: Default::default(),
            birthdate: Default::default(),
            c_hash: Default::default(),
            claims_locales: Default::default(),
            email: Default::default(),
            email_verified: Default::default(),
            exp: Default::default(),
            family_name: Default::default(),
            gender: Default::default(),
            given_name: Default::default(),
            iat: Default::default(),
            iss: Default::default(),
            jti: Default::default(),
            locale: Default::default(),
            middle_name: Default::default(),
            name: Default::default(),
            nbf: Default::default(),
            nickname: Default::default(),
            nonce: Default::default(),
            other_claims: Default::default(),
            phone_number: Default::default(),
            phone_number_verified: Default::default(),
            picture: Default::default(),
            preferred_username: Default::default(),
            profile: Default::default(),
            s_hash: Default::default(),
            sid: Default::default(),
            sub: Default::default(),
            typ: Default::default(),
            updated_at: Default::default(),
            website: Default::default(),
            zoneinfo: Default::default(),
        }
    }
}
///`IdentityProviderMapperRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "identityProviderAlias": {
///      "type": "string"
///    },
///    "identityProviderMapper": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct IdentityProviderMapperRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "identityProviderAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub identity_provider_alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "identityProviderMapper",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub identity_provider_mapper: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for IdentityProviderMapperRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            id: Default::default(),
            identity_provider_alias: Default::default(),
            identity_provider_mapper: Default::default(),
            name: Default::default(),
        }
    }
}
///`IdentityProviderMapperTypeRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "category": {
///      "type": "string"
///    },
///    "helpText": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "properties": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ConfigPropertyRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct IdentityProviderMapperTypeRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "helpText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub help_text: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub properties: ::std::vec::Vec<ConfigPropertyRepresentation>,
}
impl ::std::default::Default for IdentityProviderMapperTypeRepresentation {
    fn default() -> Self {
        Self {
            category: Default::default(),
            help_text: Default::default(),
            id: Default::default(),
            name: Default::default(),
            properties: Default::default(),
        }
    }
}
///`IdentityProviderRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "addReadTokenRoleOnCreate": {
///      "type": "boolean"
///    },
///    "alias": {
///      "type": "string"
///    },
///    "authenticateByDefault": {
///      "type": "boolean"
///    },
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "firstBrokerLoginFlowAlias": {
///      "type": "string"
///    },
///    "hideOnLogin": {
///      "type": "boolean"
///    },
///    "internalId": {
///      "type": "string"
///    },
///    "linkOnly": {
///      "type": "boolean"
///    },
///    "organizationId": {
///      "type": "string"
///    },
///    "postBrokerLoginFlowAlias": {
///      "type": "string"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "storeToken": {
///      "type": "boolean"
///    },
///    "trustEmail": {
///      "type": "boolean"
///    },
///    "updateProfileFirstLogin": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "updateProfileFirstLoginMode": {
///      "deprecated": true,
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct IdentityProviderRepresentation {
    #[serde(
        rename = "addReadTokenRoleOnCreate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub add_read_token_role_on_create: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticateByDefault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authenticate_by_default: ::std::option::Option<bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "firstBrokerLoginFlowAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_broker_login_flow_alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hideOnLogin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hide_on_login: ::std::option::Option<bool>,
    #[serde(
        rename = "internalId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "linkOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_only: ::std::option::Option<bool>,
    #[serde(
        rename = "organizationId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub organization_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "postBrokerLoginFlowAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub post_broker_login_flow_alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "storeToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_token: ::std::option::Option<bool>,
    #[serde(
        rename = "trustEmail",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trust_email: ::std::option::Option<bool>,
    #[serde(
        rename = "updateProfileFirstLogin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub update_profile_first_login: ::std::option::Option<bool>,
    #[serde(
        rename = "updateProfileFirstLoginMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub update_profile_first_login_mode: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for IdentityProviderRepresentation {
    fn default() -> Self {
        Self {
            add_read_token_role_on_create: Default::default(),
            alias: Default::default(),
            authenticate_by_default: Default::default(),
            config: Default::default(),
            display_name: Default::default(),
            enabled: Default::default(),
            first_broker_login_flow_alias: Default::default(),
            hide_on_login: Default::default(),
            internal_id: Default::default(),
            link_only: Default::default(),
            organization_id: Default::default(),
            post_broker_login_flow_alias: Default::default(),
            provider_id: Default::default(),
            store_token: Default::default(),
            trust_email: Default::default(),
            update_profile_first_login: Default::default(),
            update_profile_first_login_mode: Default::default(),
        }
    }
}
///`InstallationAdapterConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "auth-server-url": {
///      "type": "string"
///    },
///    "bearer-only": {
///      "type": "boolean"
///    },
///    "confidential-port": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "credentials": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "policy-enforcer": {
///      "$ref": "#/definitions/PolicyEnforcerConfig"
///    },
///    "public-client": {
///      "type": "boolean"
///    },
///    "realm": {
///      "type": "string"
///    },
///    "realm-public-key": {
///      "type": "string"
///    },
///    "resource": {
///      "type": "string"
///    },
///    "ssl-required": {
///      "type": "string"
///    },
///    "use-resource-role-mappings": {
///      "type": "boolean"
///    },
///    "verify-token-audience": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct InstallationAdapterConfig {
    #[serde(
        rename = "auth-server-url",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub auth_server_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "bearer-only",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_only: ::std::option::Option<bool>,
    #[serde(
        rename = "confidential-port",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub confidential_port: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub credentials: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "policy-enforcer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub policy_enforcer: ::std::option::Option<PolicyEnforcerConfig>,
    #[serde(
        rename = "public-client",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_client: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realm: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realm-public-key",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_public_key: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ssl-required",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ssl_required: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "use-resource-role-mappings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_resource_role_mappings: ::std::option::Option<bool>,
    #[serde(
        rename = "verify-token-audience",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verify_token_audience: ::std::option::Option<bool>,
}
impl ::std::default::Default for InstallationAdapterConfig {
    fn default() -> Self {
        Self {
            auth_server_url: Default::default(),
            bearer_only: Default::default(),
            confidential_port: Default::default(),
            credentials: Default::default(),
            policy_enforcer: Default::default(),
            public_client: Default::default(),
            realm: Default::default(),
            realm_public_key: Default::default(),
            resource: Default::default(),
            ssl_required: Default::default(),
            use_resource_role_mappings: Default::default(),
            verify_token_audience: Default::default(),
        }
    }
}
///`KeyMetadataRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "algorithm": {
///      "type": "string"
///    },
///    "certificate": {
///      "type": "string"
///    },
///    "kid": {
///      "type": "string"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "providerPriority": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "publicKey": {
///      "type": "string"
///    },
///    "status": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    },
///    "use": {
///      "$ref": "#/definitions/KeyUse"
///    },
///    "validTo": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct KeyMetadataRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kid: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "providerPriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_priority: ::std::option::Option<i64>,
    #[serde(
        rename = "publicKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_key: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "use",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_: ::std::option::Option<KeyUse>,
    #[serde(
        rename = "validTo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_to: ::std::option::Option<i64>,
}
impl ::std::default::Default for KeyMetadataRepresentation {
    fn default() -> Self {
        Self {
            algorithm: Default::default(),
            certificate: Default::default(),
            kid: Default::default(),
            provider_id: Default::default(),
            provider_priority: Default::default(),
            public_key: Default::default(),
            status: Default::default(),
            type_: Default::default(),
            use_: Default::default(),
            valid_to: Default::default(),
        }
    }
}
///`KeyStoreConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "format": {
///      "type": "string"
///    },
///    "keyAlias": {
///      "type": "string"
///    },
///    "keyPassword": {
///      "type": "string"
///    },
///    "realmAlias": {
///      "type": "string"
///    },
///    "realmCertificate": {
///      "type": "boolean"
///    },
///    "storePassword": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct KeyStoreConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "keyAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub key_alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "keyPassword",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub key_password: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmCertificate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_certificate: ::std::option::Option<bool>,
    #[serde(
        rename = "storePassword",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_password: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for KeyStoreConfig {
    fn default() -> Self {
        Self {
            format: Default::default(),
            key_alias: Default::default(),
            key_password: Default::default(),
            realm_alias: Default::default(),
            realm_certificate: Default::default(),
            store_password: Default::default(),
        }
    }
}
///`KeyUse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "SIG",
///    "ENC"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum KeyUse {
    #[serde(rename = "SIG")]
    Sig,
    #[serde(rename = "ENC")]
    Enc,
}
impl ::std::fmt::Display for KeyUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sig => f.write_str("SIG"),
            Self::Enc => f.write_str("ENC"),
        }
    }
}
impl ::std::str::FromStr for KeyUse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SIG" => Ok(Self::Sig),
            "ENC" => Ok(Self::Enc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for KeyUse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for KeyUse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for KeyUse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`KeysMetadataRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "active": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/KeyMetadataRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct KeysMetadataRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub active: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keys: ::std::vec::Vec<KeyMetadataRepresentation>,
}
impl ::std::default::Default for KeysMetadataRepresentation {
    fn default() -> Self {
        Self {
            active: Default::default(),
            keys: Default::default(),
        }
    }
}
///`Logic`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "POSITIVE",
///    "NEGATIVE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum Logic {
    #[serde(rename = "POSITIVE")]
    Positive,
    #[serde(rename = "NEGATIVE")]
    Negative,
}
impl ::std::fmt::Display for Logic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Positive => f.write_str("POSITIVE"),
            Self::Negative => f.write_str("NEGATIVE"),
        }
    }
}
impl ::std::str::FromStr for Logic {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "POSITIVE" => Ok(Self::Positive),
            "NEGATIVE" => Ok(Self::Negative),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Logic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Logic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Logic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ManagementPermissionReference`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "enabled": {
///      "type": "boolean"
///    },
///    "resource": {
///      "type": "string"
///    },
///    "scopePermissions": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManagementPermissionReference {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "scopePermissions",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub scope_permissions: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::default::Default for ManagementPermissionReference {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            resource: Default::default(),
            scope_permissions: Default::default(),
        }
    }
}
///`MappingsRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "clientMappings": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/ClientMappingsRepresentation"
///      }
///    },
///    "realmMappings": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RoleRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MappingsRepresentation {
    #[serde(
        rename = "clientMappings",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client_mappings: ::std::collections::BTreeMap<
        ::std::string::String,
        ClientMappingsRepresentation,
    >,
    #[serde(
        rename = "realmMappings",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub realm_mappings: ::std::vec::Vec<RoleRepresentation>,
}
impl ::std::default::Default for MappingsRepresentation {
    fn default() -> Self {
        Self {
            client_mappings: Default::default(),
            realm_mappings: Default::default(),
        }
    }
}
///`MemberRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "access": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "boolean"
///      }
///    },
///    "applicationRoles": {
///      "deprecated": true,
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "clientConsents": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserConsentRepresentation"
///      }
///    },
///    "clientRoles": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "createdTimestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "credentials": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/CredentialRepresentation"
///      }
///    },
///    "disableableCredentialTypes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "email": {
///      "type": "string"
///    },
///    "emailVerified": {
///      "type": "boolean"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "federatedIdentities": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/FederatedIdentityRepresentation"
///      }
///    },
///    "federationLink": {
///      "type": "string"
///    },
///    "firstName": {
///      "type": "string"
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "lastName": {
///      "type": "string"
///    },
///    "membershipType": {
///      "$ref": "#/definitions/MembershipType"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "origin": {
///      "type": "string"
///    },
///    "realmRoles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "requiredActions": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "self": {
///      "type": "string"
///    },
///    "serviceAccountClientId": {
///      "type": "string"
///    },
///    "socialLinks": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SocialLinkRepresentation"
///      }
///    },
///    "totp": {
///      "type": "boolean"
///    },
///    "userProfileMetadata": {
///      "$ref": "#/definitions/UserProfileMetadata"
///    },
///    "username": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MemberRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    #[serde(
        rename = "applicationRoles",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub application_roles: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "clientConsents",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub client_consents: ::std::vec::Vec<UserConsentRepresentation>,
    #[serde(
        rename = "clientRoles",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client_roles: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "createdTimestamp",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub credentials: ::std::vec::Vec<CredentialRepresentation>,
    #[serde(
        rename = "disableableCredentialTypes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disableable_credential_types: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "emailVerified",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub email_verified: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "federatedIdentities",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub federated_identities: ::std::vec::Vec<FederatedIdentityRepresentation>,
    #[serde(
        rename = "federationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation_link: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "firstName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "lastName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "membershipType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub membership_type: ::std::option::Option<MembershipType>,
    #[serde(
        rename = "notBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub not_before: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub realm_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "requiredActions",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub required_actions: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "self",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub self_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "serviceAccountClientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_account_client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "socialLinks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub social_links: ::std::vec::Vec<SocialLinkRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub totp: ::std::option::Option<bool>,
    #[serde(
        rename = "userProfileMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_profile_metadata: ::std::option::Option<UserProfileMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for MemberRepresentation {
    fn default() -> Self {
        Self {
            access: Default::default(),
            application_roles: Default::default(),
            attributes: Default::default(),
            client_consents: Default::default(),
            client_roles: Default::default(),
            created_timestamp: Default::default(),
            credentials: Default::default(),
            disableable_credential_types: Default::default(),
            email: Default::default(),
            email_verified: Default::default(),
            enabled: Default::default(),
            federated_identities: Default::default(),
            federation_link: Default::default(),
            first_name: Default::default(),
            groups: Default::default(),
            id: Default::default(),
            last_name: Default::default(),
            membership_type: Default::default(),
            not_before: Default::default(),
            origin: Default::default(),
            realm_roles: Default::default(),
            required_actions: Default::default(),
            self_: Default::default(),
            service_account_client_id: Default::default(),
            social_links: Default::default(),
            totp: Default::default(),
            user_profile_metadata: Default::default(),
            username: Default::default(),
        }
    }
}
///`MembershipType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "UNMANAGED",
///    "MANAGED"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum MembershipType {
    #[serde(rename = "UNMANAGED")]
    Unmanaged,
    #[serde(rename = "MANAGED")]
    Managed,
}
impl ::std::fmt::Display for MembershipType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unmanaged => f.write_str("UNMANAGED"),
            Self::Managed => f.write_str("MANAGED"),
        }
    }
}
impl ::std::str::FromStr for MembershipType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UNMANAGED" => Ok(Self::Unmanaged),
            "MANAGED" => Ok(Self::Managed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MembershipType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MembershipType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MembershipType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MethodConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "method": {
///      "type": "string"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "scopes-enforcement-mode": {
///      "$ref": "#/definitions/ScopeEnforcementMode"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MethodConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "scopes-enforcement-mode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scopes_enforcement_mode: ::std::option::Option<ScopeEnforcementMode>,
}
impl ::std::default::Default for MethodConfig {
    fn default() -> Self {
        Self {
            method: Default::default(),
            scopes: Default::default(),
            scopes_enforcement_mode: Default::default(),
        }
    }
}
///`MultivaluedHashMapStringComponentExportRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": {
///    "type": "array",
///    "items": {
///      "$ref": "#/definitions/ComponentExportRepresentation"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MultivaluedHashMapStringComponentExportRepresentation(
    pub ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<ComponentExportRepresentation>,
    >,
);
impl ::std::ops::Deref for MultivaluedHashMapStringComponentExportRepresentation {
    type Target = ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<ComponentExportRepresentation>,
    >;
    fn deref(
        &self,
    ) -> &::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<ComponentExportRepresentation>,
    > {
        &self.0
    }
}
impl ::std::convert::From<MultivaluedHashMapStringComponentExportRepresentation>
for ::std::collections::BTreeMap<
    ::std::string::String,
    ::std::vec::Vec<ComponentExportRepresentation>,
> {
    fn from(value: MultivaluedHashMapStringComponentExportRepresentation) -> Self {
        value.0
    }
}
impl ::std::convert::From<
    ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<ComponentExportRepresentation>,
    >,
> for MultivaluedHashMapStringComponentExportRepresentation {
    fn from(
        value: ::std::collections::BTreeMap<
            ::std::string::String,
            ::std::vec::Vec<ComponentExportRepresentation>,
        >,
    ) -> Self {
        Self(value)
    }
}
///`MultivaluedHashMapStringString`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": {
///    "type": "array",
///    "items": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MultivaluedHashMapStringString(
    pub ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
);
impl ::std::ops::Deref for MultivaluedHashMapStringString {
    type Target = ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >;
    fn deref(
        &self,
    ) -> &::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    > {
        &self.0
    }
}
impl ::std::convert::From<MultivaluedHashMapStringString>
for ::std::collections::BTreeMap<
    ::std::string::String,
    ::std::vec::Vec<::std::string::String>,
> {
    fn from(value: MultivaluedHashMapStringString) -> Self {
        value.0
    }
}
impl ::std::convert::From<
    ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
> for MultivaluedHashMapStringString {
    fn from(
        value: ::std::collections::BTreeMap<
            ::std::string::String,
            ::std::vec::Vec<::std::string::String>,
        >,
    ) -> Self {
        Self(value)
    }
}
///`OAuthClientRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "deprecated": true,
///  "type": "object",
///  "properties": {
///    "access": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "boolean"
///      }
///    },
///    "adminUrl": {
///      "type": "string"
///    },
///    "alwaysDisplayInConsole": {
///      "type": "boolean"
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authenticationFlowBindingOverrides": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authorizationServicesEnabled": {
///      "type": "boolean"
///    },
///    "authorizationSettings": {
///      "$ref": "#/definitions/ResourceServerRepresentation"
///    },
///    "baseUrl": {
///      "type": "string"
///    },
///    "bearerOnly": {
///      "type": "boolean"
///    },
///    "claims": {
///      "deprecated": true,
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ClaimRepresentation"
///        }
///      ]
///    },
///    "clientAuthenticatorType": {
///      "type": "string"
///    },
///    "clientId": {
///      "type": "string"
///    },
///    "clientTemplate": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "consentRequired": {
///      "type": "boolean"
///    },
///    "defaultClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultRoles": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "directAccessGrantsEnabled": {
///      "type": "boolean"
///    },
///    "directGrantsOnly": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "frontchannelLogout": {
///      "type": "boolean"
///    },
///    "fullScopeAllowed": {
///      "type": "boolean"
///    },
///    "id": {
///      "type": "string"
///    },
///    "implicitFlowEnabled": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "nodeReRegistrationTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "optionalClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "origin": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProtocolMapperRepresentation"
///      }
///    },
///    "publicClient": {
///      "type": "boolean"
///    },
///    "redirectUris": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "registeredNodes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "int32"
///      }
///    },
///    "registrationAccessToken": {
///      "type": "string"
///    },
///    "rootUrl": {
///      "type": "string"
///    },
///    "secret": {
///      "type": "string"
///    },
///    "serviceAccountsEnabled": {
///      "type": "boolean"
///    },
///    "standardFlowEnabled": {
///      "type": "boolean"
///    },
///    "surrogateAuthRequired": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    },
///    "useTemplateConfig": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "useTemplateMappers": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "useTemplateScope": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "webOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct OAuthClientRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    #[serde(
        rename = "adminUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "alwaysDisplayInConsole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub always_display_in_console: ::std::option::Option<bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authenticationFlowBindingOverrides",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub authentication_flow_binding_overrides: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authorizationServicesEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_services_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "authorizationSettings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_settings: ::std::option::Option<ResourceServerRepresentation>,
    #[serde(
        rename = "baseUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "bearerOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub claims: ::std::option::Option<ClaimRepresentation>,
    #[serde(
        rename = "clientAuthenticatorType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_authenticator_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_template: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "consentRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub consent_required: ::std::option::Option<bool>,
    #[serde(
        rename = "defaultClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "directAccessGrantsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_access_grants_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "directGrantsOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_grants_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "frontchannelLogout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frontchannel_logout: ::std::option::Option<bool>,
    #[serde(
        rename = "fullScopeAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_scope_allowed: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "implicitFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub implicit_flow_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "nodeReRegistrationTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub node_re_registration_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "notBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub not_before: ::std::option::Option<i32>,
    #[serde(
        rename = "optionalClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub optional_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
    #[serde(
        rename = "publicClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_client: ::std::option::Option<bool>,
    #[serde(
        rename = "redirectUris",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub redirect_uris: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "registeredNodes",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub registered_nodes: ::std::collections::BTreeMap<::std::string::String, i32>,
    #[serde(
        rename = "registrationAccessToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_access_token: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rootUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub root_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "serviceAccountsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_accounts_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "standardFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub standard_flow_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "surrogateAuthRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surrogate_auth_required: ::std::option::Option<bool>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "useTemplateConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_config: ::std::option::Option<bool>,
    #[serde(
        rename = "useTemplateMappers",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_mappers: ::std::option::Option<bool>,
    #[serde(
        rename = "useTemplateScope",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_template_scope: ::std::option::Option<bool>,
    #[serde(
        rename = "webOrigins",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_origins: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for OAuthClientRepresentation {
    fn default() -> Self {
        Self {
            access: Default::default(),
            admin_url: Default::default(),
            always_display_in_console: Default::default(),
            attributes: Default::default(),
            authentication_flow_binding_overrides: Default::default(),
            authorization_services_enabled: Default::default(),
            authorization_settings: Default::default(),
            base_url: Default::default(),
            bearer_only: Default::default(),
            claims: Default::default(),
            client_authenticator_type: Default::default(),
            client_id: Default::default(),
            client_template: Default::default(),
            consent_required: Default::default(),
            default_client_scopes: Default::default(),
            default_roles: Default::default(),
            description: Default::default(),
            direct_access_grants_enabled: Default::default(),
            direct_grants_only: Default::default(),
            enabled: Default::default(),
            frontchannel_logout: Default::default(),
            full_scope_allowed: Default::default(),
            id: Default::default(),
            implicit_flow_enabled: Default::default(),
            name: Default::default(),
            node_re_registration_timeout: Default::default(),
            not_before: Default::default(),
            optional_client_scopes: Default::default(),
            origin: Default::default(),
            protocol: Default::default(),
            protocol_mappers: Default::default(),
            public_client: Default::default(),
            redirect_uris: Default::default(),
            registered_nodes: Default::default(),
            registration_access_token: Default::default(),
            root_url: Default::default(),
            secret: Default::default(),
            service_accounts_enabled: Default::default(),
            standard_flow_enabled: Default::default(),
            surrogate_auth_required: Default::default(),
            type_: Default::default(),
            use_template_config: Default::default(),
            use_template_mappers: Default::default(),
            use_template_scope: Default::default(),
            web_origins: Default::default(),
        }
    }
}
///`OrganizationDomainRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "name": {
///      "type": "string"
///    },
///    "verified": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrganizationDomainRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub verified: ::std::option::Option<bool>,
}
impl ::std::default::Default for OrganizationDomainRepresentation {
    fn default() -> Self {
        Self {
            name: Default::default(),
            verified: Default::default(),
        }
    }
}
///`OrganizationRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alias": {
///      "type": "string"
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "domains": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OrganizationDomainRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "id": {
///      "type": "string"
///    },
///    "identityProviders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/IdentityProviderRepresentation"
///      }
///    },
///    "members": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MemberRepresentation"
///      }
///    },
///    "name": {
///      "type": "string"
///    },
///    "redirectUrl": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct OrganizationRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub domains: ::std::option::Option<Vec<OrganizationDomainRepresentation>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "identityProviders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub identity_providers: ::std::vec::Vec<IdentityProviderRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub members: ::std::vec::Vec<MemberRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "redirectUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub redirect_url: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OrganizationRepresentation {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            attributes: Default::default(),
            description: Default::default(),
            domains: Default::default(),
            enabled: Default::default(),
            id: Default::default(),
            identity_providers: Default::default(),
            members: Default::default(),
            name: Default::default(),
            redirect_url: Default::default(),
        }
    }
}
///`PathCacheConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "lifespan": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "max-entries": {
///      "type": "integer",
///      "format": "int32"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PathCacheConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lifespan: ::std::option::Option<i64>,
    #[serde(
        rename = "max-entries",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_entries: ::std::option::Option<i32>,
}
impl ::std::default::Default for PathCacheConfig {
    fn default() -> Self {
        Self {
            lifespan: Default::default(),
            max_entries: Default::default(),
        }
    }
}
///`PathConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "claim-information-point": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {}
///      }
///    },
///    "enforcement-mode": {
///      "$ref": "#/definitions/EnforcementMode"
///    },
///    "id": {
///      "type": "string"
///    },
///    "invalidated": {
///      "type": "boolean"
///    },
///    "methods": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MethodConfig"
///      }
///    },
///    "name": {
///      "type": "string"
///    },
///    "path": {
///      "type": "string"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "static": {
///      "type": "boolean"
///    },
///    "staticPath": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PathConfig {
    #[serde(
        rename = "claim-information-point",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub claim_information_point: ::std::collections::BTreeMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(
        rename = "enforcement-mode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enforcement_mode: ::std::option::Option<EnforcementMode>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invalidated: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub methods: ::std::vec::Vec<MethodConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "static",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub static_: ::std::option::Option<bool>,
    #[serde(
        rename = "staticPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub static_path: ::std::option::Option<bool>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PathConfig {
    fn default() -> Self {
        Self {
            claim_information_point: Default::default(),
            enforcement_mode: Default::default(),
            id: Default::default(),
            invalidated: Default::default(),
            methods: Default::default(),
            name: Default::default(),
            path: Default::default(),
            scopes: Default::default(),
            static_: Default::default(),
            static_path: Default::default(),
            type_: Default::default(),
        }
    }
}
///`Permission`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "claims": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        },
///        "uniqueItems": true
///      }
///    },
///    "rsid": {
///      "type": "string"
///    },
///    "rsname": {
///      "type": "string"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Permission {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub claims: ::std::collections::BTreeMap<
        ::std::string::String,
        Vec<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rsid: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rsname: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::default::Default for Permission {
    fn default() -> Self {
        Self {
            claims: Default::default(),
            rsid: Default::default(),
            rsname: Default::default(),
            scopes: Default::default(),
        }
    }
}
///`PolicyEnforcementMode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ENFORCING",
///    "PERMISSIVE",
///    "DISABLED"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PolicyEnforcementMode {
    #[serde(rename = "ENFORCING")]
    Enforcing,
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "DISABLED")]
    Disabled,
}
impl ::std::fmt::Display for PolicyEnforcementMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enforcing => f.write_str("ENFORCING"),
            Self::Permissive => f.write_str("PERMISSIVE"),
            Self::Disabled => f.write_str("DISABLED"),
        }
    }
}
impl ::std::str::FromStr for PolicyEnforcementMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ENFORCING" => Ok(Self::Enforcing),
            "PERMISSIVE" => Ok(Self::Permissive),
            "DISABLED" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PolicyEnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PolicyEnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PolicyEnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PolicyEnforcerConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "auth-server-url": {
///      "type": "string"
///    },
///    "claim-information-point": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {}
///      }
///    },
///    "credentials": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "enforcement-mode": {
///      "$ref": "#/definitions/EnforcementMode"
///    },
///    "http-method-as-scope": {
///      "type": "boolean"
///    },
///    "lazy-load-paths": {
///      "type": "boolean"
///    },
///    "on-deny-redirect-to": {
///      "type": "string"
///    },
///    "path-cache": {
///      "$ref": "#/definitions/PathCacheConfig"
///    },
///    "paths": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PathConfig"
///      }
///    },
///    "realm": {
///      "type": "string"
///    },
///    "resource": {
///      "type": "string"
///    },
///    "user-managed-access": {
///      "$ref": "#/definitions/UserManagedAccessConfig"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PolicyEnforcerConfig {
    #[serde(
        rename = "auth-server-url",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub auth_server_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "claim-information-point",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub claim_information_point: ::std::collections::BTreeMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub credentials: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "enforcement-mode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enforcement_mode: ::std::option::Option<EnforcementMode>,
    #[serde(
        rename = "http-method-as-scope",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub http_method_as_scope: ::std::option::Option<bool>,
    #[serde(
        rename = "lazy-load-paths",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lazy_load_paths: ::std::option::Option<bool>,
    #[serde(
        rename = "on-deny-redirect-to",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_deny_redirect_to: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "path-cache",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub path_cache: ::std::option::Option<PathCacheConfig>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub paths: ::std::vec::Vec<PathConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realm: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "user-managed-access",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_managed_access: ::std::option::Option<UserManagedAccessConfig>,
}
impl ::std::default::Default for PolicyEnforcerConfig {
    fn default() -> Self {
        Self {
            auth_server_url: Default::default(),
            claim_information_point: Default::default(),
            credentials: Default::default(),
            enforcement_mode: Default::default(),
            http_method_as_scope: Default::default(),
            lazy_load_paths: Default::default(),
            on_deny_redirect_to: Default::default(),
            path_cache: Default::default(),
            paths: Default::default(),
            realm: Default::default(),
            resource: Default::default(),
            user_managed_access: Default::default(),
        }
    }
}
///`PolicyEvaluationRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "clientId": {
///      "type": "string"
///    },
///    "context": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "type": "string"
///        }
///      }
///    },
///    "entitlements": {
///      "type": "boolean"
///    },
///    "resourceType": {
///      "type": "string"
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ResourceRepresentation"
///      }
///    },
///    "roleIds": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "userId": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PolicyEvaluationRequest {
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub context: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::collections::BTreeMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub entitlements: ::std::option::Option<bool>,
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub resources: ::std::vec::Vec<ResourceRepresentation>,
    #[serde(
        rename = "roleIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub role_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "userId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PolicyEvaluationRequest {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            context: Default::default(),
            entitlements: Default::default(),
            resource_type: Default::default(),
            resources: Default::default(),
            role_ids: Default::default(),
            user_id: Default::default(),
        }
    }
}
///`PolicyEvaluationResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "entitlements": {
///      "type": "boolean"
///    },
///    "results": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/EvaluationResultRepresentation"
///      }
///    },
///    "rpt": {
///      "$ref": "#/definitions/AccessToken"
///    },
///    "status": {
///      "$ref": "#/definitions/DecisionEffect"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PolicyEvaluationResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub entitlements: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub results: ::std::vec::Vec<EvaluationResultRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rpt: ::std::option::Option<AccessToken>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<DecisionEffect>,
}
impl ::std::default::Default for PolicyEvaluationResponse {
    fn default() -> Self {
        Self {
            entitlements: Default::default(),
            results: Default::default(),
            rpt: Default::default(),
            status: Default::default(),
        }
    }
}
///`PolicyProviderRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "group": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PolicyProviderRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub group: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PolicyProviderRepresentation {
    fn default() -> Self {
        Self {
            group: Default::default(),
            name: Default::default(),
            type_: Default::default(),
        }
    }
}
///`PolicyRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "decisionStrategy": {
///      "$ref": "#/definitions/DecisionStrategy"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "logic": {
///      "$ref": "#/definitions/Logic"
///    },
///    "name": {
///      "type": "string"
///    },
///    "owner": {
///      "type": "string"
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "resourceType": {
///      "type": "string"
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "resourcesData": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ResourceRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "scopesData": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct PolicyRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "decisionStrategy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub decision_strategy: ::std::option::Option<DecisionStrategy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logic: ::std::option::Option<Logic>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policies: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "resourcesData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resources_data: ::std::option::Option<Vec<ResourceRepresentation>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "scopesData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scopes_data: ::std::option::Option<Vec<ScopeRepresentation>>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PolicyRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            decision_strategy: Default::default(),
            description: Default::default(),
            id: Default::default(),
            logic: Default::default(),
            name: Default::default(),
            owner: Default::default(),
            policies: Default::default(),
            resource_type: Default::default(),
            resources: Default::default(),
            resources_data: Default::default(),
            scopes: Default::default(),
            scopes_data: Default::default(),
            type_: Default::default(),
        }
    }
}
///`PolicyResultRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "associatedPolicies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PolicyResultRepresentation"
///      }
///    },
///    "policy": {
///      "$ref": "#/definitions/PolicyRepresentation"
///    },
///    "resourceType": {
///      "type": "string"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "status": {
///      "$ref": "#/definitions/DecisionEffect"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PolicyResultRepresentation {
    #[serde(
        rename = "associatedPolicies",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub associated_policies: ::std::vec::Vec<PolicyResultRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<PolicyRepresentation>,
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<DecisionEffect>,
}
impl ::std::default::Default for PolicyResultRepresentation {
    fn default() -> Self {
        Self {
            associated_policies: Default::default(),
            policy: Default::default(),
            resource_type: Default::default(),
            scopes: Default::default(),
            status: Default::default(),
        }
    }
}
///`PropertyConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "applicable": {
///      "type": "boolean"
///    },
///    "value": {}
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PropertyConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub applicable: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::serde_json::Value>,
}
impl ::std::default::Default for PropertyConfig {
    fn default() -> Self {
        Self {
            applicable: Default::default(),
            value: Default::default(),
        }
    }
}
///`ProtocolMapperEvaluationRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "containerId": {
///      "type": "string"
///    },
///    "containerName": {
///      "type": "string"
///    },
///    "containerType": {
///      "type": "string"
///    },
///    "mapperId": {
///      "type": "string"
///    },
///    "mapperName": {
///      "type": "string"
///    },
///    "protocolMapper": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProtocolMapperEvaluationRepresentation {
    #[serde(
        rename = "containerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "containerName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub container_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "containerType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub container_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "mapperId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mapper_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "mapperName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mapper_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMapper",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub protocol_mapper: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ProtocolMapperEvaluationRepresentation {
    fn default() -> Self {
        Self {
            container_id: Default::default(),
            container_name: Default::default(),
            container_type: Default::default(),
            mapper_id: Default::default(),
            mapper_name: Default::default(),
            protocol_mapper: Default::default(),
        }
    }
}
///`ProtocolMapperRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "consentRequired": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "consentText": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string"
///    },
///    "protocolMapper": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ProtocolMapperRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "consentRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub consent_required: ::std::option::Option<bool>,
    #[serde(
        rename = "consentText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub consent_text: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMapper",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub protocol_mapper: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ProtocolMapperRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            consent_required: Default::default(),
            consent_text: Default::default(),
            id: Default::default(),
            name: Default::default(),
            protocol: Default::default(),
            protocol_mapper: Default::default(),
        }
    }
}
///`PublishedRealmRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "account-service": {
///      "type": "string"
///    },
///    "public_key": {
///      "type": "string"
///    },
///    "realm": {
///      "type": "string"
///    },
///    "token-service": {
///      "type": "string"
///    },
///    "tokens-not-before": {
///      "type": "integer",
///      "format": "int32"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublishedRealmRepresentation {
    #[serde(
        rename = "account-service",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_service: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub public_key: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realm: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "token-service",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_service: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "tokens-not-before",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tokens_not_before: ::std::option::Option<i32>,
}
impl ::std::default::Default for PublishedRealmRepresentation {
    fn default() -> Self {
        Self {
            account_service: Default::default(),
            public_key: Default::default(),
            realm: Default::default(),
            token_service: Default::default(),
            tokens_not_before: Default::default(),
        }
    }
}
///`RealmEventsConfigRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "adminEventsDetailsEnabled": {
///      "type": "boolean"
///    },
///    "adminEventsEnabled": {
///      "type": "boolean"
///    },
///    "enabledEventTypes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "eventsEnabled": {
///      "type": "boolean"
///    },
///    "eventsExpiration": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "eventsListeners": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RealmEventsConfigRepresentation {
    #[serde(
        rename = "adminEventsDetailsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_events_details_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "adminEventsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_events_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "enabledEventTypes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enabled_event_types: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "eventsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "eventsExpiration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_expiration: ::std::option::Option<i64>,
    #[serde(
        rename = "eventsListeners",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub events_listeners: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for RealmEventsConfigRepresentation {
    fn default() -> Self {
        Self {
            admin_events_details_enabled: Default::default(),
            admin_events_enabled: Default::default(),
            enabled_event_types: Default::default(),
            events_enabled: Default::default(),
            events_expiration: Default::default(),
            events_listeners: Default::default(),
        }
    }
}
///`RealmRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "accessCodeLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "accessCodeLifespanLogin": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "accessCodeLifespanUserAction": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "accessTokenLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "accessTokenLifespanForImplicitFlow": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "accountTheme": {
///      "type": "string"
///    },
///    "actionTokenGeneratedByAdminLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "actionTokenGeneratedByUserLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "adminEventsDetailsEnabled": {
///      "type": "boolean"
///    },
///    "adminEventsEnabled": {
///      "type": "boolean"
///    },
///    "adminPermissionsClient": {
///      "$ref": "#/definitions/ClientRepresentation"
///    },
///    "adminPermissionsEnabled": {
///      "type": "boolean"
///    },
///    "adminTheme": {
///      "type": "string"
///    },
///    "applicationScopeMappings": {
///      "deprecated": true,
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/ScopeMappingRepresentation"
///        }
///      }
///    },
///    "applications": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ApplicationRepresentation"
///      }
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authenticationFlows": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AuthenticationFlowRepresentation"
///      }
///    },
///    "authenticatorConfig": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AuthenticatorConfigRepresentation"
///      }
///    },
///    "browserFlow": {
///      "type": "string"
///    },
///    "browserSecurityHeaders": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "bruteForceProtected": {
///      "type": "boolean"
///    },
///    "bruteForceStrategy": {
///      "$ref": "#/definitions/BruteForceStrategy"
///    },
///    "certificate": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "clientAuthenticationFlow": {
///      "type": "string"
///    },
///    "clientOfflineSessionIdleTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "clientOfflineSessionMaxLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "clientPolicies": {
///      "$ref": "#/definitions/ClientPoliciesRepresentation"
///    },
///    "clientProfiles": {
///      "$ref": "#/definitions/ClientProfilesRepresentation"
///    },
///    "clientScopeMappings": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/ScopeMappingRepresentation"
///        }
///      }
///    },
///    "clientScopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientScopeRepresentation"
///      }
///    },
///    "clientSessionIdleTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "clientSessionMaxLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "clientTemplates": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientTemplateRepresentation"
///      }
///    },
///    "clients": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ClientRepresentation"
///      }
///    },
///    "codeSecret": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "components": {
///      "$ref": "#/definitions/MultivaluedHashMapStringComponentExportRepresentation"
///    },
///    "defaultDefaultClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultGroups": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultLocale": {
///      "type": "string"
///    },
///    "defaultOptionalClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultRole": {
///      "$ref": "#/definitions/RoleRepresentation"
///    },
///    "defaultRoles": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultSignatureAlgorithm": {
///      "type": "string"
///    },
///    "directGrantFlow": {
///      "type": "string"
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "displayNameHtml": {
///      "type": "string"
///    },
///    "dockerAuthenticationFlow": {
///      "type": "string"
///    },
///    "duplicateEmailsAllowed": {
///      "type": "boolean"
///    },
///    "editUsernameAllowed": {
///      "type": "boolean"
///    },
///    "emailTheme": {
///      "type": "string"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "enabledEventTypes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "eventsEnabled": {
///      "type": "boolean"
///    },
///    "eventsExpiration": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "eventsListeners": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "failureFactor": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "federatedUsers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserRepresentation"
///      }
///    },
///    "firstBrokerLoginFlow": {
///      "type": "string"
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/GroupRepresentation"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "identityProviderMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/IdentityProviderMapperRepresentation"
///      }
///    },
///    "identityProviders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/IdentityProviderRepresentation"
///      }
///    },
///    "internationalizationEnabled": {
///      "type": "boolean"
///    },
///    "keycloakVersion": {
///      "type": "string"
///    },
///    "localizationTexts": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "type": "string"
///        }
///      }
///    },
///    "loginTheme": {
///      "type": "string"
///    },
///    "loginWithEmailAllowed": {
///      "type": "boolean"
///    },
///    "maxDeltaTimeSeconds": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "maxFailureWaitSeconds": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "maxTemporaryLockouts": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "minimumQuickLoginWaitSeconds": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "oAuth2DeviceCodeLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "oAuth2DevicePollingInterval": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "oauthClients": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OAuthClientRepresentation"
///      }
///    },
///    "offlineSessionIdleTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "offlineSessionMaxLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "offlineSessionMaxLifespanEnabled": {
///      "type": "boolean"
///    },
///    "organizations": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OrganizationRepresentation"
///      }
///    },
///    "organizationsEnabled": {
///      "type": "boolean"
///    },
///    "otpPolicyAlgorithm": {
///      "type": "string"
///    },
///    "otpPolicyCodeReusable": {
///      "type": "boolean"
///    },
///    "otpPolicyDigits": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "otpPolicyInitialCounter": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "otpPolicyLookAheadWindow": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "otpPolicyPeriod": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "otpPolicyType": {
///      "type": "string"
///    },
///    "otpSupportedApplications": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "passwordCredentialGrantAllowed": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "passwordPolicy": {
///      "type": "string"
///    },
///    "permanentLockout": {
///      "type": "boolean"
///    },
///    "privateKey": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ProtocolMapperRepresentation"
///      }
///    },
///    "publicKey": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "quickLoginCheckMilliSeconds": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "realm": {
///      "type": "string"
///    },
///    "realmCacheEnabled": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "refreshTokenMaxReuse": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "registrationAllowed": {
///      "type": "boolean"
///    },
///    "registrationEmailAsUsername": {
///      "type": "boolean"
///    },
///    "registrationFlow": {
///      "type": "string"
///    },
///    "rememberMe": {
///      "type": "boolean"
///    },
///    "requiredActions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RequiredActionProviderRepresentation"
///      }
///    },
///    "requiredCredentials": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "resetCredentialsFlow": {
///      "type": "string"
///    },
///    "resetPasswordAllowed": {
///      "type": "boolean"
///    },
///    "revokeRefreshToken": {
///      "type": "boolean"
///    },
///    "roles": {
///      "$ref": "#/definitions/RolesRepresentation"
///    },
///    "scopeMappings": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeMappingRepresentation"
///      }
///    },
///    "smtpServer": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "social": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "socialProviders": {
///      "deprecated": true,
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "sslRequired": {
///      "type": "string"
///    },
///    "ssoSessionIdleTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "ssoSessionIdleTimeoutRememberMe": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "ssoSessionMaxLifespan": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "ssoSessionMaxLifespanRememberMe": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "supportedLocales": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "updateProfileOnInitialSocialLogin": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "userCacheEnabled": {
///      "deprecated": true,
///      "type": "boolean"
///    },
///    "userFederationMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserFederationMapperRepresentation"
///      }
///    },
///    "userFederationProviders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserFederationProviderRepresentation"
///      }
///    },
///    "userManagedAccessAllowed": {
///      "type": "boolean"
///    },
///    "users": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserRepresentation"
///      }
///    },
///    "verifiableCredentialsEnabled": {
///      "type": "boolean"
///    },
///    "verifyEmail": {
///      "type": "boolean"
///    },
///    "waitIncrementSeconds": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "webAuthnPolicyAcceptableAaguids": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyAttestationConveyancePreference": {
///      "type": "string"
///    },
///    "webAuthnPolicyAuthenticatorAttachment": {
///      "type": "string"
///    },
///    "webAuthnPolicyAvoidSameAuthenticatorRegister": {
///      "type": "boolean"
///    },
///    "webAuthnPolicyCreateTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "webAuthnPolicyExtraOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessAcceptableAaguids": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessAttestationConveyancePreference": {
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessAuthenticatorAttachment": {
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister": {
///      "type": "boolean"
///    },
///    "webAuthnPolicyPasswordlessCreateTimeout": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "webAuthnPolicyPasswordlessExtraOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessRequireResidentKey": {
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessRpEntityName": {
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessRpId": {
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessSignatureAlgorithms": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessUserVerificationRequirement": {
///      "type": "string"
///    },
///    "webAuthnPolicyRequireResidentKey": {
///      "type": "string"
///    },
///    "webAuthnPolicyRpEntityName": {
///      "type": "string"
///    },
///    "webAuthnPolicyRpId": {
///      "type": "string"
///    },
///    "webAuthnPolicySignatureAlgorithms": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyUserVerificationRequirement": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct RealmRepresentation {
    #[serde(
        rename = "accessCodeLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_code_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "accessCodeLifespanLogin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_code_lifespan_login: ::std::option::Option<i32>,
    #[serde(
        rename = "accessCodeLifespanUserAction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_code_lifespan_user_action: ::std::option::Option<i32>,
    #[serde(
        rename = "accessTokenLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_token_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "accessTokenLifespanForImplicitFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_token_lifespan_for_implicit_flow: ::std::option::Option<i32>,
    #[serde(
        rename = "accountTheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_theme: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "actionTokenGeneratedByAdminLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub action_token_generated_by_admin_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "actionTokenGeneratedByUserLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub action_token_generated_by_user_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "adminEventsDetailsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_events_details_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "adminEventsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_events_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "adminPermissionsClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_permissions_client: ::std::option::Option<ClientRepresentation>,
    #[serde(
        rename = "adminPermissionsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_permissions_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "adminTheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_theme: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "applicationScopeMappings",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub application_scope_mappings: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<ScopeMappingRepresentation>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub applications: ::std::vec::Vec<ApplicationRepresentation>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "authenticationFlows",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub authentication_flows: ::std::vec::Vec<AuthenticationFlowRepresentation>,
    #[serde(
        rename = "authenticatorConfig",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub authenticator_config: ::std::vec::Vec<AuthenticatorConfigRepresentation>,
    #[serde(
        rename = "browserFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub browser_flow: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "browserSecurityHeaders",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub browser_security_headers: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "bruteForceProtected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brute_force_protected: ::std::option::Option<bool>,
    #[serde(
        rename = "bruteForceStrategy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brute_force_strategy: ::std::option::Option<BruteForceStrategy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientAuthenticationFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_authentication_flow: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientOfflineSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_offline_session_idle_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "clientOfflineSessionMaxLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_offline_session_max_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "clientPolicies",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_policies: ::std::option::Option<ClientPoliciesRepresentation>,
    #[serde(
        rename = "clientProfiles",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_profiles: ::std::option::Option<ClientProfilesRepresentation>,
    #[serde(
        rename = "clientScopeMappings",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client_scope_mappings: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<ScopeMappingRepresentation>,
    >,
    #[serde(
        rename = "clientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub client_scopes: ::std::vec::Vec<ClientScopeRepresentation>,
    #[serde(
        rename = "clientSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_session_idle_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "clientSessionMaxLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_session_max_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "clientTemplates",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub client_templates: ::std::vec::Vec<ClientTemplateRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub clients: ::std::vec::Vec<ClientRepresentation>,
    #[serde(
        rename = "codeSecret",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_secret: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub components: ::std::option::Option<
        MultivaluedHashMapStringComponentExportRepresentation,
    >,
    #[serde(
        rename = "defaultDefaultClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_default_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultGroups",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_groups: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultLocale",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_locale: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "defaultOptionalClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_optional_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultRole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_role: ::std::option::Option<RoleRepresentation>,
    #[serde(
        rename = "defaultRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub default_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "defaultSignatureAlgorithm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_signature_algorithm: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "directGrantFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub direct_grant_flow: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayNameHtml",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name_html: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "dockerAuthenticationFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub docker_authentication_flow: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "duplicateEmailsAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duplicate_emails_allowed: ::std::option::Option<bool>,
    #[serde(
        rename = "editUsernameAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub edit_username_allowed: ::std::option::Option<bool>,
    #[serde(
        rename = "emailTheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub email_theme: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "enabledEventTypes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enabled_event_types: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "eventsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "eventsExpiration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_expiration: ::std::option::Option<i64>,
    #[serde(
        rename = "eventsListeners",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub events_listeners: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "failureFactor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub failure_factor: ::std::option::Option<i32>,
    #[serde(
        rename = "federatedUsers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub federated_users: ::std::vec::Vec<UserRepresentation>,
    #[serde(
        rename = "firstBrokerLoginFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_broker_login_flow: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<GroupRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "identityProviderMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub identity_provider_mappers: ::std::vec::Vec<IdentityProviderMapperRepresentation>,
    #[serde(
        rename = "identityProviders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub identity_providers: ::std::vec::Vec<IdentityProviderRepresentation>,
    #[serde(
        rename = "internationalizationEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internationalization_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "keycloakVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub keycloak_version: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "localizationTexts",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub localization_texts: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::collections::BTreeMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(
        rename = "loginTheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_theme: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "loginWithEmailAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_with_email_allowed: ::std::option::Option<bool>,
    #[serde(
        rename = "maxDeltaTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_delta_time_seconds: ::std::option::Option<i32>,
    #[serde(
        rename = "maxFailureWaitSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_failure_wait_seconds: ::std::option::Option<i32>,
    #[serde(
        rename = "maxTemporaryLockouts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_temporary_lockouts: ::std::option::Option<i32>,
    #[serde(
        rename = "minimumQuickLoginWaitSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub minimum_quick_login_wait_seconds: ::std::option::Option<i32>,
    #[serde(
        rename = "notBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub not_before: ::std::option::Option<i32>,
    #[serde(
        rename = "oAuth2DeviceCodeLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub o_auth2_device_code_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "oAuth2DevicePollingInterval",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub o_auth2_device_polling_interval: ::std::option::Option<i32>,
    #[serde(
        rename = "oauthClients",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub oauth_clients: ::std::vec::Vec<OAuthClientRepresentation>,
    #[serde(
        rename = "offlineSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offline_session_idle_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "offlineSessionMaxLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offline_session_max_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "offlineSessionMaxLifespanEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offline_session_max_lifespan_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub organizations: ::std::vec::Vec<OrganizationRepresentation>,
    #[serde(
        rename = "organizationsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub organizations_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "otpPolicyAlgorithm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_algorithm: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "otpPolicyCodeReusable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_code_reusable: ::std::option::Option<bool>,
    #[serde(
        rename = "otpPolicyDigits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_digits: ::std::option::Option<i32>,
    #[serde(
        rename = "otpPolicyInitialCounter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_initial_counter: ::std::option::Option<i32>,
    #[serde(
        rename = "otpPolicyLookAheadWindow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_look_ahead_window: ::std::option::Option<i32>,
    #[serde(
        rename = "otpPolicyPeriod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_period: ::std::option::Option<i32>,
    #[serde(
        rename = "otpPolicyType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "otpSupportedApplications",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub otp_supported_applications: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "passwordCredentialGrantAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub password_credential_grant_allowed: ::std::option::Option<bool>,
    #[serde(
        rename = "passwordPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub password_policy: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "permanentLockout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub permanent_lockout: ::std::option::Option<bool>,
    #[serde(
        rename = "privateKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub private_key: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
    #[serde(
        rename = "publicKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_key: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "quickLoginCheckMilliSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quick_login_check_milli_seconds: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub realm: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmCacheEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub realm_cache_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "refreshTokenMaxReuse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_token_max_reuse: ::std::option::Option<i32>,
    #[serde(
        rename = "registrationAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_allowed: ::std::option::Option<bool>,
    #[serde(
        rename = "registrationEmailAsUsername",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_email_as_username: ::std::option::Option<bool>,
    #[serde(
        rename = "registrationFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_flow: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rememberMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remember_me: ::std::option::Option<bool>,
    #[serde(
        rename = "requiredActions",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub required_actions: ::std::vec::Vec<RequiredActionProviderRepresentation>,
    #[serde(
        rename = "requiredCredentials",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub required_credentials: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "resetCredentialsFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reset_credentials_flow: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "resetPasswordAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reset_password_allowed: ::std::option::Option<bool>,
    #[serde(
        rename = "revokeRefreshToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub revoke_refresh_token: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roles: ::std::option::Option<RolesRepresentation>,
    #[serde(
        rename = "scopeMappings",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub scope_mappings: ::std::vec::Vec<ScopeMappingRepresentation>,
    #[serde(
        rename = "smtpServer",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub smtp_server: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub social: ::std::option::Option<bool>,
    #[serde(
        rename = "socialProviders",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub social_providers: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "sslRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ssl_required: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ssoSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_idle_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "ssoSessionIdleTimeoutRememberMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_idle_timeout_remember_me: ::std::option::Option<i32>,
    #[serde(
        rename = "ssoSessionMaxLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_max_lifespan: ::std::option::Option<i32>,
    #[serde(
        rename = "ssoSessionMaxLifespanRememberMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_max_lifespan_remember_me: ::std::option::Option<i32>,
    #[serde(
        rename = "supportedLocales",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supported_locales: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "updateProfileOnInitialSocialLogin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub update_profile_on_initial_social_login: ::std::option::Option<bool>,
    #[serde(
        rename = "userCacheEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_cache_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "userFederationMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub user_federation_mappers: ::std::vec::Vec<UserFederationMapperRepresentation>,
    #[serde(
        rename = "userFederationProviders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub user_federation_providers: ::std::vec::Vec<UserFederationProviderRepresentation>,
    #[serde(
        rename = "userManagedAccessAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_managed_access_allowed: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub users: ::std::vec::Vec<UserRepresentation>,
    #[serde(
        rename = "verifiableCredentialsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verifiable_credentials_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "verifyEmail",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verify_email: ::std::option::Option<bool>,
    #[serde(
        rename = "waitIncrementSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wait_increment_seconds: ::std::option::Option<i32>,
    #[serde(
        rename = "webAuthnPolicyAcceptableAaguids",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_acceptable_aaguids: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "webAuthnPolicyAttestationConveyancePreference",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_attestation_conveyance_preference: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyAuthenticatorAttachment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_authenticator_attachment: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyAvoidSameAuthenticatorRegister",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_avoid_same_authenticator_register: ::std::option::Option<bool>,
    #[serde(
        rename = "webAuthnPolicyCreateTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_create_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "webAuthnPolicyExtraOrigins",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_extra_origins: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "webAuthnPolicyPasswordlessAcceptableAaguids",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_passwordless_acceptable_aaguids: ::std::vec::Vec<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessAttestationConveyancePreference",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_attestation_conveyance_preference: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessAuthenticatorAttachment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_authenticator_attachment: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_avoid_same_authenticator_register: ::std::option::Option<
        bool,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessCreateTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_create_timeout: ::std::option::Option<i32>,
    #[serde(
        rename = "webAuthnPolicyPasswordlessExtraOrigins",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_passwordless_extra_origins: ::std::vec::Vec<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessRequireResidentKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_require_resident_key: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessRpEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_rp_entity_name: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessRpId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_rp_id: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessSignatureAlgorithms",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_passwordless_signature_algorithms: ::std::vec::Vec<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyPasswordlessUserVerificationRequirement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_user_verification_requirement: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyRequireResidentKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_require_resident_key: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(
        rename = "webAuthnPolicyRpEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_rp_entity_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "webAuthnPolicyRpId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_rp_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "webAuthnPolicySignatureAlgorithms",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_signature_algorithms: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "webAuthnPolicyUserVerificationRequirement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_user_verification_requirement: ::std::option::Option<
        ::std::string::String,
    >,
}
impl ::std::default::Default for RealmRepresentation {
    fn default() -> Self {
        Self {
            access_code_lifespan: Default::default(),
            access_code_lifespan_login: Default::default(),
            access_code_lifespan_user_action: Default::default(),
            access_token_lifespan: Default::default(),
            access_token_lifespan_for_implicit_flow: Default::default(),
            account_theme: Default::default(),
            action_token_generated_by_admin_lifespan: Default::default(),
            action_token_generated_by_user_lifespan: Default::default(),
            admin_events_details_enabled: Default::default(),
            admin_events_enabled: Default::default(),
            admin_permissions_client: Default::default(),
            admin_permissions_enabled: Default::default(),
            admin_theme: Default::default(),
            application_scope_mappings: Default::default(),
            applications: Default::default(),
            attributes: Default::default(),
            authentication_flows: Default::default(),
            authenticator_config: Default::default(),
            browser_flow: Default::default(),
            browser_security_headers: Default::default(),
            brute_force_protected: Default::default(),
            brute_force_strategy: Default::default(),
            certificate: Default::default(),
            client_authentication_flow: Default::default(),
            client_offline_session_idle_timeout: Default::default(),
            client_offline_session_max_lifespan: Default::default(),
            client_policies: Default::default(),
            client_profiles: Default::default(),
            client_scope_mappings: Default::default(),
            client_scopes: Default::default(),
            client_session_idle_timeout: Default::default(),
            client_session_max_lifespan: Default::default(),
            client_templates: Default::default(),
            clients: Default::default(),
            code_secret: Default::default(),
            components: Default::default(),
            default_default_client_scopes: Default::default(),
            default_groups: Default::default(),
            default_locale: Default::default(),
            default_optional_client_scopes: Default::default(),
            default_role: Default::default(),
            default_roles: Default::default(),
            default_signature_algorithm: Default::default(),
            direct_grant_flow: Default::default(),
            display_name: Default::default(),
            display_name_html: Default::default(),
            docker_authentication_flow: Default::default(),
            duplicate_emails_allowed: Default::default(),
            edit_username_allowed: Default::default(),
            email_theme: Default::default(),
            enabled: Default::default(),
            enabled_event_types: Default::default(),
            events_enabled: Default::default(),
            events_expiration: Default::default(),
            events_listeners: Default::default(),
            failure_factor: Default::default(),
            federated_users: Default::default(),
            first_broker_login_flow: Default::default(),
            groups: Default::default(),
            id: Default::default(),
            identity_provider_mappers: Default::default(),
            identity_providers: Default::default(),
            internationalization_enabled: Default::default(),
            keycloak_version: Default::default(),
            localization_texts: Default::default(),
            login_theme: Default::default(),
            login_with_email_allowed: Default::default(),
            max_delta_time_seconds: Default::default(),
            max_failure_wait_seconds: Default::default(),
            max_temporary_lockouts: Default::default(),
            minimum_quick_login_wait_seconds: Default::default(),
            not_before: Default::default(),
            o_auth2_device_code_lifespan: Default::default(),
            o_auth2_device_polling_interval: Default::default(),
            oauth_clients: Default::default(),
            offline_session_idle_timeout: Default::default(),
            offline_session_max_lifespan: Default::default(),
            offline_session_max_lifespan_enabled: Default::default(),
            organizations: Default::default(),
            organizations_enabled: Default::default(),
            otp_policy_algorithm: Default::default(),
            otp_policy_code_reusable: Default::default(),
            otp_policy_digits: Default::default(),
            otp_policy_initial_counter: Default::default(),
            otp_policy_look_ahead_window: Default::default(),
            otp_policy_period: Default::default(),
            otp_policy_type: Default::default(),
            otp_supported_applications: Default::default(),
            password_credential_grant_allowed: Default::default(),
            password_policy: Default::default(),
            permanent_lockout: Default::default(),
            private_key: Default::default(),
            protocol_mappers: Default::default(),
            public_key: Default::default(),
            quick_login_check_milli_seconds: Default::default(),
            realm: Default::default(),
            realm_cache_enabled: Default::default(),
            refresh_token_max_reuse: Default::default(),
            registration_allowed: Default::default(),
            registration_email_as_username: Default::default(),
            registration_flow: Default::default(),
            remember_me: Default::default(),
            required_actions: Default::default(),
            required_credentials: Default::default(),
            reset_credentials_flow: Default::default(),
            reset_password_allowed: Default::default(),
            revoke_refresh_token: Default::default(),
            roles: Default::default(),
            scope_mappings: Default::default(),
            smtp_server: Default::default(),
            social: Default::default(),
            social_providers: Default::default(),
            ssl_required: Default::default(),
            sso_session_idle_timeout: Default::default(),
            sso_session_idle_timeout_remember_me: Default::default(),
            sso_session_max_lifespan: Default::default(),
            sso_session_max_lifespan_remember_me: Default::default(),
            supported_locales: Default::default(),
            update_profile_on_initial_social_login: Default::default(),
            user_cache_enabled: Default::default(),
            user_federation_mappers: Default::default(),
            user_federation_providers: Default::default(),
            user_managed_access_allowed: Default::default(),
            users: Default::default(),
            verifiable_credentials_enabled: Default::default(),
            verify_email: Default::default(),
            wait_increment_seconds: Default::default(),
            web_authn_policy_acceptable_aaguids: Default::default(),
            web_authn_policy_attestation_conveyance_preference: Default::default(),
            web_authn_policy_authenticator_attachment: Default::default(),
            web_authn_policy_avoid_same_authenticator_register: Default::default(),
            web_authn_policy_create_timeout: Default::default(),
            web_authn_policy_extra_origins: Default::default(),
            web_authn_policy_passwordless_acceptable_aaguids: Default::default(),
            web_authn_policy_passwordless_attestation_conveyance_preference: Default::default(),
            web_authn_policy_passwordless_authenticator_attachment: Default::default(),
            web_authn_policy_passwordless_avoid_same_authenticator_register: Default::default(),
            web_authn_policy_passwordless_create_timeout: Default::default(),
            web_authn_policy_passwordless_extra_origins: Default::default(),
            web_authn_policy_passwordless_require_resident_key: Default::default(),
            web_authn_policy_passwordless_rp_entity_name: Default::default(),
            web_authn_policy_passwordless_rp_id: Default::default(),
            web_authn_policy_passwordless_signature_algorithms: Default::default(),
            web_authn_policy_passwordless_user_verification_requirement: Default::default(),
            web_authn_policy_require_resident_key: Default::default(),
            web_authn_policy_rp_entity_name: Default::default(),
            web_authn_policy_rp_id: Default::default(),
            web_authn_policy_signature_algorithms: Default::default(),
            web_authn_policy_user_verification_requirement: Default::default(),
        }
    }
}
///`RequiredActionConfigInfoRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "properties": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ConfigPropertyRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RequiredActionConfigInfoRepresentation {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub properties: ::std::vec::Vec<ConfigPropertyRepresentation>,
}
impl ::std::default::Default for RequiredActionConfigInfoRepresentation {
    fn default() -> Self {
        Self {
            properties: Default::default(),
        }
    }
}
///`RequiredActionConfigRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RequiredActionConfigRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::default::Default for RequiredActionConfigRepresentation {
    fn default() -> Self {
        Self { config: Default::default() }
    }
}
///`RequiredActionProviderRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alias": {
///      "type": "string"
///    },
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "defaultAction": {
///      "type": "boolean"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "providerId": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct RequiredActionProviderRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "defaultAction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_action: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<i32>,
    #[serde(
        rename = "providerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for RequiredActionProviderRepresentation {
    fn default() -> Self {
        Self {
            alias: Default::default(),
            config: Default::default(),
            default_action: Default::default(),
            enabled: Default::default(),
            name: Default::default(),
            priority: Default::default(),
            provider_id: Default::default(),
        }
    }
}
///`ResourceOwnerRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResourceOwnerRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ResourceOwnerRepresentation {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}
///`ResourceRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "_id": {
///      "type": "string"
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "icon_uri": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "owner": {
///      "readOnly": true,
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/definitions/ResourceOwnerRepresentation"
///        }
///      ]
///    },
///    "ownerManagedAccess": {
///      "type": "boolean"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "scopesUma": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    },
///    "uri": {
///      "deprecated": true,
///      "writeOnly": true,
///      "type": "string"
///    },
///    "uris": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ResourceRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub icon_uri: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "_id",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<ResourceOwnerRepresentation>,
    #[serde(
        rename = "ownerManagedAccess",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub owner_managed_access: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<ScopeRepresentation>>,
    #[serde(
        rename = "scopesUma",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scopes_uma: ::std::option::Option<Vec<ScopeRepresentation>>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub uris: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::default::Default for ResourceRepresentation {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            display_name: Default::default(),
            icon_uri: Default::default(),
            id: Default::default(),
            name: Default::default(),
            owner: Default::default(),
            owner_managed_access: Default::default(),
            scopes: Default::default(),
            scopes_uma: Default::default(),
            type_: Default::default(),
            uri: Default::default(),
            uris: Default::default(),
        }
    }
}
///`ResourceServerRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "allowRemoteResourceManagement": {
///      "type": "boolean"
///    },
///    "authorizationSchema": {
///      "$ref": "#/definitions/AuthorizationSchema"
///    },
///    "clientId": {
///      "type": "string"
///    },
///    "decisionStrategy": {
///      "$ref": "#/definitions/DecisionStrategy"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PolicyRepresentation"
///      }
///    },
///    "policyEnforcementMode": {
///      "$ref": "#/definitions/PolicyEnforcementMode"
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ResourceRepresentation"
///      }
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ScopeRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResourceServerRepresentation {
    #[serde(
        rename = "allowRemoteResourceManagement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_remote_resource_management: ::std::option::Option<bool>,
    #[serde(
        rename = "authorizationSchema",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_schema: ::std::option::Option<AuthorizationSchema>,
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "decisionStrategy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub decision_strategy: ::std::option::Option<DecisionStrategy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub policies: ::std::vec::Vec<PolicyRepresentation>,
    #[serde(
        rename = "policyEnforcementMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub policy_enforcement_mode: ::std::option::Option<PolicyEnforcementMode>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub resources: ::std::vec::Vec<ResourceRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<ScopeRepresentation>,
}
impl ::std::default::Default for ResourceServerRepresentation {
    fn default() -> Self {
        Self {
            allow_remote_resource_management: Default::default(),
            authorization_schema: Default::default(),
            client_id: Default::default(),
            decision_strategy: Default::default(),
            id: Default::default(),
            name: Default::default(),
            policies: Default::default(),
            policy_enforcement_mode: Default::default(),
            resources: Default::default(),
            scopes: Default::default(),
        }
    }
}
///`ResourceType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "groupType": {
///      "type": "string"
///    },
///    "scopeAliases": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        },
///        "uniqueItems": true
///      }
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResourceType {
    #[serde(
        rename = "groupType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "scopeAliases",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub scope_aliases: ::std::collections::BTreeMap<
        ::std::string::String,
        Vec<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ResourceType {
    fn default() -> Self {
        Self {
            group_type: Default::default(),
            scope_aliases: Default::default(),
            scopes: Default::default(),
            type_: Default::default(),
        }
    }
}
///`RoleRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "clientRole": {
///      "type": "boolean"
///    },
///    "composite": {
///      "type": "boolean"
///    },
///    "composites": {
///      "$ref": "#/definitions/Composites"
///    },
///    "containerId": {
///      "type": "string"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "scopeParamRequired": {
///      "deprecated": true,
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct RoleRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "clientRole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_role: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub composite: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub composites: ::std::option::Option<Composites>,
    #[serde(
        rename = "containerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub container_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "scopeParamRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scope_param_required: ::std::option::Option<bool>,
}
impl ::std::default::Default for RoleRepresentation {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            client_role: Default::default(),
            composite: Default::default(),
            composites: Default::default(),
            container_id: Default::default(),
            description: Default::default(),
            id: Default::default(),
            name: Default::default(),
            scope_param_required: Default::default(),
        }
    }
}
///`RolesRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "application": {
///      "deprecated": true,
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/RoleRepresentation"
///        }
///      }
///    },
///    "client": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/definitions/RoleRepresentation"
///        }
///      }
///    },
///    "realm": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RoleRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RolesRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub application: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<RoleRepresentation>,
    >,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<RoleRepresentation>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub realm: ::std::vec::Vec<RoleRepresentation>,
}
impl ::std::default::Default for RolesRepresentation {
    fn default() -> Self {
        Self {
            application: Default::default(),
            client: Default::default(),
            realm: Default::default(),
        }
    }
}
///`ScopeEnforcementMode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ALL",
///    "ANY",
///    "DISABLED"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ScopeEnforcementMode {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "DISABLED")]
    Disabled,
}
impl ::std::fmt::Display for ScopeEnforcementMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("ALL"),
            Self::Any => f.write_str("ANY"),
            Self::Disabled => f.write_str("DISABLED"),
        }
    }
}
impl ::std::str::FromStr for ScopeEnforcementMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ALL" => Ok(Self::All),
            "ANY" => Ok(Self::Any),
            "DISABLED" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScopeEnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScopeEnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScopeEnforcementMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ScopeMappingRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "client": {
///      "type": "string"
///    },
///    "clientScope": {
///      "type": "string"
///    },
///    "clientTemplate": {
///      "deprecated": true,
///      "type": "string"
///    },
///    "roles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "self": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ScopeMappingRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientScope",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_scope: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "clientTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_template: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roles: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "self",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub self_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ScopeMappingRepresentation {
    fn default() -> Self {
        Self {
            client: Default::default(),
            client_scope: Default::default(),
            client_template: Default::default(),
            roles: Default::default(),
            self_: Default::default(),
        }
    }
}
///`ScopeRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "displayName": {
///      "type": "string"
///    },
///    "iconUri": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PolicyRepresentation"
///      }
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ResourceRepresentation"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct ScopeRepresentation {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "iconUri",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_uri: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub policies: ::std::vec::Vec<PolicyRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub resources: ::std::vec::Vec<ResourceRepresentation>,
}
impl ::std::default::Default for ScopeRepresentation {
    fn default() -> Self {
        Self {
            display_name: Default::default(),
            icon_uri: Default::default(),
            id: Default::default(),
            name: Default::default(),
            policies: Default::default(),
            resources: Default::default(),
        }
    }
}
///`SocialLinkRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "socialProvider": {
///      "type": "string"
///    },
///    "socialUserId": {
///      "type": "string"
///    },
///    "socialUsername": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SocialLinkRepresentation {
    #[serde(
        rename = "socialProvider",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub social_provider: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "socialUserId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub social_user_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "socialUsername",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub social_username: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for SocialLinkRepresentation {
    fn default() -> Self {
        Self {
            social_provider: Default::default(),
            social_user_id: Default::default(),
            social_username: Default::default(),
        }
    }
}
///`UnmanagedAttributePolicy`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ENABLED",
///    "ADMIN_VIEW",
///    "ADMIN_EDIT"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum UnmanagedAttributePolicy {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ADMIN_VIEW")]
    AdminView,
    #[serde(rename = "ADMIN_EDIT")]
    AdminEdit,
}
impl ::std::fmt::Display for UnmanagedAttributePolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enabled => f.write_str("ENABLED"),
            Self::AdminView => f.write_str("ADMIN_VIEW"),
            Self::AdminEdit => f.write_str("ADMIN_EDIT"),
        }
    }
}
impl ::std::str::FromStr for UnmanagedAttributePolicy {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ENABLED" => Ok(Self::Enabled),
            "ADMIN_VIEW" => Ok(Self::AdminView),
            "ADMIN_EDIT" => Ok(Self::AdminEdit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UnmanagedAttributePolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UnmanagedAttributePolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UnmanagedAttributePolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`UpAttribute`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "annotations": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "group": {
///      "type": "string"
///    },
///    "multivalued": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "permissions": {
///      "$ref": "#/definitions/UPAttributePermissions"
///    },
///    "required": {
///      "$ref": "#/definitions/UPAttributeRequired"
///    },
///    "selector": {
///      "$ref": "#/definitions/UPAttributeSelector"
///    },
///    "validations": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {}
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpAttribute {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub annotations: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub group: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub multivalued: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<UpAttributePermissions>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<UpAttributeRequired>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<UpAttributeSelector>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub validations: ::std::collections::BTreeMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
}
impl ::std::default::Default for UpAttribute {
    fn default() -> Self {
        Self {
            annotations: Default::default(),
            display_name: Default::default(),
            group: Default::default(),
            multivalued: Default::default(),
            name: Default::default(),
            permissions: Default::default(),
            required: Default::default(),
            selector: Default::default(),
            validations: Default::default(),
        }
    }
}
///`UpAttributePermissions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "edit": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "view": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpAttributePermissions {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub edit: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub view: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::default::Default for UpAttributePermissions {
    fn default() -> Self {
        Self {
            edit: Default::default(),
            view: Default::default(),
        }
    }
}
///`UpAttributeRequired`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "roles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpAttributeRequired {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roles: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::default::Default for UpAttributeRequired {
    fn default() -> Self {
        Self {
            roles: Default::default(),
            scopes: Default::default(),
        }
    }
}
///`UpAttributeSelector`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpAttributeSelector {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::default::Default for UpAttributeSelector {
    fn default() -> Self {
        Self { scopes: Default::default() }
    }
}
///`UpConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "attributes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UPAttribute"
///      }
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UPGroup"
///      }
///    },
///    "unmanagedAttributePolicy": {
///      "$ref": "#/definitions/UnmanagedAttributePolicy"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpConfig {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attributes: ::std::vec::Vec<UpAttribute>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<UpGroup>,
    #[serde(
        rename = "unmanagedAttributePolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unmanaged_attribute_policy: ::std::option::Option<UnmanagedAttributePolicy>,
}
impl ::std::default::Default for UpConfig {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            groups: Default::default(),
            unmanaged_attribute_policy: Default::default(),
        }
    }
}
///`UpGroup`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "annotations": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "displayDescription": {
///      "type": "string"
///    },
///    "displayHeader": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpGroup {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub annotations: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "displayDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_header: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UpGroup {
    fn default() -> Self {
        Self {
            annotations: Default::default(),
            display_description: Default::default(),
            display_header: Default::default(),
            name: Default::default(),
        }
    }
}
///`UserConsentRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "clientId": {
///      "type": "string"
///    },
///    "createdDate": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "grantedClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "grantedRealmRoles": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "lastUpdatedDate": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserConsentRepresentation {
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "createdDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_date: ::std::option::Option<i64>,
    #[serde(
        rename = "grantedClientScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub granted_client_scopes: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "grantedRealmRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub granted_realm_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "lastUpdatedDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_updated_date: ::std::option::Option<i64>,
}
impl ::std::default::Default for UserConsentRepresentation {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            created_date: Default::default(),
            granted_client_scopes: Default::default(),
            granted_realm_roles: Default::default(),
            last_updated_date: Default::default(),
        }
    }
}
///`UserFederationMapperRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "federationMapperType": {
///      "type": "string"
///    },
///    "federationProviderDisplayName": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserFederationMapperRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "federationMapperType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation_mapper_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "federationProviderDisplayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation_provider_display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UserFederationMapperRepresentation {
    fn default() -> Self {
        Self {
            config: Default::default(),
            federation_mapper_type: Default::default(),
            federation_provider_display_name: Default::default(),
            id: Default::default(),
            name: Default::default(),
        }
    }
}
///`UserFederationProviderRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "changedSyncPeriod": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "config": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "fullSyncPeriod": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "id": {
///      "type": "string"
///    },
///    "lastSync": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "providerName": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserFederationProviderRepresentation {
    #[serde(
        rename = "changedSyncPeriod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub changed_sync_period: ::std::option::Option<i32>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub config: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "fullSyncPeriod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_sync_period: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "lastSync",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_sync: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<i32>,
    #[serde(
        rename = "providerName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub provider_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UserFederationProviderRepresentation {
    fn default() -> Self {
        Self {
            changed_sync_period: Default::default(),
            config: Default::default(),
            display_name: Default::default(),
            full_sync_period: Default::default(),
            id: Default::default(),
            last_sync: Default::default(),
            priority: Default::default(),
            provider_name: Default::default(),
        }
    }
}
///`UserManagedAccessConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct UserManagedAccessConfig(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for UserManagedAccessConfig {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<UserManagedAccessConfig>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: UserManagedAccessConfig) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for UserManagedAccessConfig {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`UserProfileAttributeGroupMetadata`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "annotations": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "displayDescription": {
///      "type": "string"
///    },
///    "displayHeader": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserProfileAttributeGroupMetadata {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub annotations: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "displayDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_header: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UserProfileAttributeGroupMetadata {
    fn default() -> Self {
        Self {
            annotations: Default::default(),
            display_description: Default::default(),
            display_header: Default::default(),
            name: Default::default(),
        }
    }
}
///`UserProfileAttributeMetadata`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "annotations": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "displayName": {
///      "type": "string"
///    },
///    "group": {
///      "type": "string"
///    },
///    "multivalued": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "readOnly": {
///      "type": "boolean"
///    },
///    "required": {
///      "type": "boolean"
///    },
///    "validators": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {}
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserProfileAttributeMetadata {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub annotations: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub group: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub multivalued: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "readOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub read_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub validators: ::std::collections::BTreeMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
}
impl ::std::default::Default for UserProfileAttributeMetadata {
    fn default() -> Self {
        Self {
            annotations: Default::default(),
            display_name: Default::default(),
            group: Default::default(),
            multivalued: Default::default(),
            name: Default::default(),
            read_only: Default::default(),
            required: Default::default(),
            validators: Default::default(),
        }
    }
}
///`UserProfileMetadata`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "attributes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserProfileAttributeMetadata"
///      }
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserProfileAttributeGroupMetadata"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserProfileMetadata {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attributes: ::std::vec::Vec<UserProfileAttributeMetadata>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<UserProfileAttributeGroupMetadata>,
}
impl ::std::default::Default for UserProfileMetadata {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            groups: Default::default(),
        }
    }
}
///`UserRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "access": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "boolean"
///      }
///    },
///    "applicationRoles": {
///      "deprecated": true,
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "attributes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "clientConsents": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UserConsentRepresentation"
///      }
///    },
///    "clientRoles": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "createdTimestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "credentials": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/CredentialRepresentation"
///      }
///    },
///    "disableableCredentialTypes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "email": {
///      "type": "string"
///    },
///    "emailVerified": {
///      "type": "boolean"
///    },
///    "enabled": {
///      "type": "boolean"
///    },
///    "federatedIdentities": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/FederatedIdentityRepresentation"
///      }
///    },
///    "federationLink": {
///      "type": "string"
///    },
///    "firstName": {
///      "type": "string"
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "lastName": {
///      "type": "string"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "origin": {
///      "type": "string"
///    },
///    "realmRoles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "requiredActions": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "self": {
///      "type": "string"
///    },
///    "serviceAccountClientId": {
///      "type": "string"
///    },
///    "socialLinks": {
///      "deprecated": true,
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SocialLinkRepresentation"
///      }
///    },
///    "totp": {
///      "type": "boolean"
///    },
///    "userProfileMetadata": {
///      "$ref": "#/definitions/UserProfileMetadata"
///    },
///    "username": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "k8s-schema",
    schemars(transform = crate::k8s::remove_k8s_managed_fields)
)]
pub struct UserRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    #[serde(
        rename = "applicationRoles",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub application_roles: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub attributes: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "clientConsents",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub client_consents: ::std::vec::Vec<UserConsentRepresentation>,
    #[serde(
        rename = "clientRoles",
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub client_roles: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    #[serde(
        rename = "createdTimestamp",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub credentials: ::std::vec::Vec<CredentialRepresentation>,
    #[serde(
        rename = "disableableCredentialTypes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disableable_credential_types: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "emailVerified",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub email_verified: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "federatedIdentities",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub federated_identities: ::std::vec::Vec<FederatedIdentityRepresentation>,
    #[serde(
        rename = "federationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation_link: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "firstName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "lastName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "notBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub not_before: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "realmRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub realm_roles: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "requiredActions",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub required_actions: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "self",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub self_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "serviceAccountClientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_account_client_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "socialLinks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub social_links: ::std::vec::Vec<SocialLinkRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub totp: ::std::option::Option<bool>,
    #[serde(
        rename = "userProfileMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_profile_metadata: ::std::option::Option<UserProfileMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UserRepresentation {
    fn default() -> Self {
        Self {
            access: Default::default(),
            application_roles: Default::default(),
            attributes: Default::default(),
            client_consents: Default::default(),
            client_roles: Default::default(),
            created_timestamp: Default::default(),
            credentials: Default::default(),
            disableable_credential_types: Default::default(),
            email: Default::default(),
            email_verified: Default::default(),
            enabled: Default::default(),
            federated_identities: Default::default(),
            federation_link: Default::default(),
            first_name: Default::default(),
            groups: Default::default(),
            id: Default::default(),
            last_name: Default::default(),
            not_before: Default::default(),
            origin: Default::default(),
            realm_roles: Default::default(),
            required_actions: Default::default(),
            self_: Default::default(),
            service_account_client_id: Default::default(),
            social_links: Default::default(),
            totp: Default::default(),
            user_profile_metadata: Default::default(),
            username: Default::default(),
        }
    }
}
///`UserSessionRepresentation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "clients": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "ipAddress": {
///      "type": "string"
///    },
///    "lastAccess": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "rememberMe": {
///      "type": "boolean"
///    },
///    "start": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "transientUser": {
///      "type": "boolean"
///    },
///    "userId": {
///      "type": "string"
///    },
///    "username": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UserSessionRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub clients: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ipAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "lastAccess",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_access: ::std::option::Option<i64>,
    #[serde(
        rename = "rememberMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remember_me: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<i64>,
    #[serde(
        rename = "transientUser",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transient_user: ::std::option::Option<bool>,
    #[serde(
        rename = "userId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UserSessionRepresentation {
    fn default() -> Self {
        Self {
            clients: Default::default(),
            id: Default::default(),
            ip_address: Default::default(),
            last_access: Default::default(),
            remember_me: Default::default(),
            start: Default::default(),
            transient_user: Default::default(),
            user_id: Default::default(),
            username: Default::default(),
        }
    }
}
