/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
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
///AbstractPolicyRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "decisionStrategy": {
///      "$ref": "#/$defs/DecisionStrategy"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "logic": {
///      "$ref": "#/$defs/Logic"
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
///        "$ref": "#/$defs/ResourceRepresentation"
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
///        "$ref": "#/$defs/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AbstractPolicyRepresentation>
for AbstractPolicyRepresentation {
    fn from(value: &AbstractPolicyRepresentation) -> Self {
        value.clone()
    }
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
///Access
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Access {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roles: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub verify_caller: ::std::option::Option<bool>,
}
impl ::std::convert::From<&Access> for Access {
    fn from(value: &Access) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Access {
    fn default() -> Self {
        Self {
            roles: Default::default(),
            verify_caller: Default::default(),
        }
    }
}
///AccessToken
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
///      "$ref": "#/$defs/AddressClaimSet"
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "authorization": {
///      "$ref": "#/$defs/Authorization"
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
///      "$ref": "#/$defs/Confirmation"
///    },
///    "email": {
///      "type": "string"
///    },
///    "email_verified": {
///      "type": "boolean"
///    },
///    "exp": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "$ref": "#/$defs/Access"
///    },
///    "resource_access": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/$defs/Access"
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "website": {
///      "type": "string"
///    },
///    "zoneinfo": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AccessToken> for AccessToken {
    fn from(value: &AccessToken) -> Self {
        value.clone()
    }
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
///Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Access Token Lifespan",
///  "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct AccessTokenLifespan(::std::string::String);
impl ::std::ops::Deref for AccessTokenLifespan {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccessTokenLifespan> for ::std::string::String {
    fn from(value: AccessTokenLifespan) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AccessTokenLifespan> for AccessTokenLifespan {
    fn from(value: &AccessTokenLifespan) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AccessTokenLifespan {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AccessTokenLifespan {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessTokenLifespan {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessTokenLifespan {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AccessTokenLifespan {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///AddressClaimSet
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AddressClaimSet> for AddressClaimSet {
    fn from(value: &AddressClaimSet) -> Self {
        value.clone()
    }
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
///AdminEventRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "authDetails": {
///      "$ref": "#/$defs/AuthDetailsRepresentation"
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AdminEventRepresentation> for AdminEventRepresentation {
    fn from(value: &AdminEventRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AdminEventRepresentation {
    fn default() -> Self {
        Self {
            auth_details: Default::default(),
            details: Default::default(),
            error: Default::default(),
            operation_type: Default::default(),
            realm_id: Default::default(),
            representation: Default::default(),
            resource_path: Default::default(),
            resource_type: Default::default(),
            time: Default::default(),
        }
    }
}
///Allow the external identity provider to create a new identifier to represent the principal.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Allow create",
///  "description": "Allow the external identity provider to create a new identifier to represent the principal.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum AllowCreate {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for AllowCreate {
    fn from(value: &AllowCreate) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AllowCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for AllowCreate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllowCreate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllowCreate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllowCreate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Allow regex pattern comparison",
///  "description": "If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum AllowRegexPatternComparison {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for AllowRegexPatternComparison {
    fn from(value: &AllowRegexPatternComparison) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AllowRegexPatternComparison {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for AllowRegexPatternComparison {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllowRegexPatternComparison {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllowRegexPatternComparison {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllowRegexPatternComparison {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Allowed clock skew",
///  "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct AllowedClockSkew(::std::string::String);
impl ::std::ops::Deref for AllowedClockSkew {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AllowedClockSkew> for ::std::string::String {
    fn from(value: AllowedClockSkew) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AllowedClockSkew> for AllowedClockSkew {
    fn from(value: &AllowedClockSkew) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AllowedClockSkew {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AllowedClockSkew {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllowedClockSkew {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllowedClockSkew {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AllowedClockSkew {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///ApplicationRepresentation
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
///      "$ref": "#/$defs/ResourceServerRepresentation"
///    },
///    "baseUrl": {
///      "type": "string"
///    },
///    "bearerOnly": {
///      "type": "boolean"
///    },
///    "claims": {
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/ClaimRepresentation"
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///        "$ref": "#/$defs/ProtocolMapperRepresentation"
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
///        "format": "int32",
///        "maximum": 2147483647.0,
///        "minimum": -2147483648.0
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
///      "type": "boolean"
///    },
///    "useTemplateMappers": {
///      "type": "boolean"
///    },
///    "useTemplateScope": {
///      "type": "boolean"
///    },
///    "webOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ApplicationRepresentation> for ApplicationRepresentation {
    fn from(value: &ApplicationRepresentation) -> Self {
        value.clone()
    }
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
///Communicates to an authenticator the preference of how to generate an attestation statement.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Attestation conveyance preference",
///  "description": "Communicates to an authenticator the preference of how to generate an attestation statement.",
///  "type": "string",
///  "enum": [
///    "not specified",
///    "none",
///    "indirect",
///    "direct"
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
pub enum AttestationConveyancePreference {
    #[serde(rename = "not specified")]
    NotSpecified,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "indirect")]
    Indirect,
    #[serde(rename = "direct")]
    Direct,
}
impl ::std::convert::From<&Self> for AttestationConveyancePreference {
    fn from(value: &AttestationConveyancePreference) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AttestationConveyancePreference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotSpecified => write!(f, "not specified"),
            Self::None => write!(f, "none"),
            Self::Indirect => write!(f, "indirect"),
            Self::Direct => write!(f, "direct"),
        }
    }
}
impl ::std::str::FromStr for AttestationConveyancePreference {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "not specified" => Ok(Self::NotSpecified),
            "none" => Ok(Self::None),
            "indirect" => Ok(Self::Indirect),
            "direct" => Ok(Self::Direct),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AttestationConveyancePreference {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for AttestationConveyancePreference {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttestationConveyancePreference {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Index of the Attribute Consuming Service profile to request during authentication.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Attribute Consuming Service Index",
///  "description": "Index of the Attribute Consuming Service profile to request during authentication.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct AttributeConsumingServiceIndex(::std::string::String);
impl ::std::ops::Deref for AttributeConsumingServiceIndex {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AttributeConsumingServiceIndex> for ::std::string::String {
    fn from(value: AttributeConsumingServiceIndex) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AttributeConsumingServiceIndex>
for AttributeConsumingServiceIndex {
    fn from(value: &AttributeConsumingServiceIndex) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AttributeConsumingServiceIndex {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AttributeConsumingServiceIndex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AttributeConsumingServiceIndex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttributeConsumingServiceIndex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AttributeConsumingServiceIndex {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///AuthDetailsRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthDetailsRepresentation> for AuthDetailsRepresentation {
    fn from(value: &AuthDetailsRepresentation) -> Self {
        value.clone()
    }
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
///Authentication
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Authentication",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum Authentication {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for Authentication {
    fn from(value: &Authentication) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Authentication {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for Authentication {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Authentication {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Authentication {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Authentication {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///AuthenticationExecutionExportRepresentation
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
///      "type": "boolean"
///    },
///    "flowAlias": {
///      "type": "string"
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "requirement": {
///      "type": "string",
///      "enum": [
///        "ALTERNATIVE",
///        "CONDITIONAL",
///        "DISABLED",
///        "REQUIRED"
///      ]
///    },
///    "userSetupAllowed": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
    pub requirement: ::std::option::Option<
        AuthenticationExecutionExportRepresentationRequirement,
    >,
    #[serde(
        rename = "userSetupAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_setup_allowed: ::std::option::Option<bool>,
}
impl ::std::convert::From<&AuthenticationExecutionExportRepresentation>
for AuthenticationExecutionExportRepresentation {
    fn from(value: &AuthenticationExecutionExportRepresentation) -> Self {
        value.clone()
    }
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
///AuthenticationExecutionExportRepresentationRequirement
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ALTERNATIVE",
///    "CONDITIONAL",
///    "DISABLED",
///    "REQUIRED"
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
pub enum AuthenticationExecutionExportRepresentationRequirement {
    #[serde(rename = "ALTERNATIVE")]
    Alternative,
    #[serde(rename = "CONDITIONAL")]
    Conditional,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "REQUIRED")]
    Required,
}
impl ::std::convert::From<&Self>
for AuthenticationExecutionExportRepresentationRequirement {
    fn from(value: &AuthenticationExecutionExportRepresentationRequirement) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AuthenticationExecutionExportRepresentationRequirement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alternative => write!(f, "ALTERNATIVE"),
            Self::Conditional => write!(f, "CONDITIONAL"),
            Self::Disabled => write!(f, "DISABLED"),
            Self::Required => write!(f, "REQUIRED"),
        }
    }
}
impl ::std::str::FromStr for AuthenticationExecutionExportRepresentationRequirement {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ALTERNATIVE" => Ok(Self::Alternative),
            "CONDITIONAL" => Ok(Self::Conditional),
            "DISABLED" => Ok(Self::Disabled),
            "REQUIRED" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for AuthenticationExecutionExportRepresentationRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for AuthenticationExecutionExportRepresentationRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for AuthenticationExecutionExportRepresentationRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///AuthenticationExecutionInfoRepresentation
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "level": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthenticationExecutionInfoRepresentation>
for AuthenticationExecutionInfoRepresentation {
    fn from(value: &AuthenticationExecutionInfoRepresentation) -> Self {
        value.clone()
    }
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
///AuthenticationExecutionRepresentation
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "requirement": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthenticationExecutionRepresentation>
for AuthenticationExecutionRepresentation {
    fn from(value: &AuthenticationExecutionRepresentation) -> Self {
        value.clone()
    }
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
///AuthenticationFlowOverrides
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Authentication flow overrides",
///  "type": "object",
///  "properties": {
///    "browser": {
///      "title": "Browser Flow",
///      "description": "Select the flow you want to use for browser authentication.",
///      "type": "string"
///    },
///    "direct_grant": {
///      "title": "Direct Grant Flow",
///      "description": "Select the flow you want to use for direct grant authentication.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuthenticationFlowOverrides {
    ///Select the flow you want to use for browser authentication.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub browser: ::std::option::Option<::std::string::String>,
    ///Select the flow you want to use for direct grant authentication.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub direct_grant: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::convert::From<&AuthenticationFlowOverrides> for AuthenticationFlowOverrides {
    fn from(value: &AuthenticationFlowOverrides) -> Self {
        value.clone()
    }
}
///AuthenticationFlowRepresentation
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
///        "$ref": "#/$defs/AuthenticationExecutionExportRepresentation"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthenticationFlowRepresentation>
for AuthenticationFlowRepresentation {
    fn from(value: &AuthenticationFlowRepresentation) -> Self {
        value.clone()
    }
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
///The way of identifying the end-user for whom authentication is being requested. Currently only "login_hint" is supported.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Authentication Requested User Hint",
///  "description": "The way of identifying the end-user for whom authentication is being requested. Currently only \"login_hint\" is supported.",
///  "type": "string",
///  "enum": [
///    "login_hint"
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
pub enum AuthenticationRequestedUserHint {
    #[serde(rename = "login_hint")]
    LoginHint,
}
impl ::std::convert::From<&Self> for AuthenticationRequestedUserHint {
    fn from(value: &AuthenticationRequestedUserHint) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AuthenticationRequestedUserHint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LoginHint => write!(f, "login_hint"),
        }
    }
}
impl ::std::str::FromStr for AuthenticationRequestedUserHint {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "login_hint" => Ok(Self::LoginHint),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuthenticationRequestedUserHint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for AuthenticationRequestedUserHint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuthenticationRequestedUserHint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Communicates to an authenticator an acceptable attachment pattern.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Authenticator Attachment",
///  "description": "Communicates to an authenticator an acceptable attachment pattern.",
///  "type": "string",
///  "enum": [
///    "not specified",
///    "platform",
///    "cross-platform"
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
pub enum AuthenticatorAttachment {
    #[serde(rename = "not specified")]
    NotSpecified,
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "cross-platform")]
    CrossPlatform,
}
impl ::std::convert::From<&Self> for AuthenticatorAttachment {
    fn from(value: &AuthenticatorAttachment) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AuthenticatorAttachment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotSpecified => write!(f, "not specified"),
            Self::Platform => write!(f, "platform"),
            Self::CrossPlatform => write!(f, "cross-platform"),
        }
    }
}
impl ::std::str::FromStr for AuthenticatorAttachment {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "not specified" => Ok(Self::NotSpecified),
            "platform" => Ok(Self::Platform),
            "cross-platform" => Ok(Self::CrossPlatform),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuthenticatorAttachment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AuthenticatorAttachment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuthenticatorAttachment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///AuthenticatorConfigInfoRepresentation
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
///        "$ref": "#/$defs/ConfigPropertyRepresentation"
///      }
///    },
///    "providerId": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthenticatorConfigInfoRepresentation>
for AuthenticatorConfigInfoRepresentation {
    fn from(value: &AuthenticatorConfigInfoRepresentation) -> Self {
        value.clone()
    }
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
///AuthenticatorConfigRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthenticatorConfigRepresentation>
for AuthenticatorConfigRepresentation {
    fn from(value: &AuthenticatorConfigRepresentation) -> Self {
        value.clone()
    }
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
///Authorization
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
///        "$ref": "#/$defs/Permission"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Authorization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: ::std::vec::Vec<Permission>,
}
impl ::std::convert::From<&Authorization> for Authorization {
    fn from(value: &Authorization) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Authorization {
    fn default() -> Self {
        Self {
            permissions: Default::default(),
        }
    }
}
///AuthorizationSchema
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
///        "$ref": "#/$defs/ResourceType"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&AuthorizationSchema> for AuthorizationSchema {
    fn from(value: &AuthorizationSchema) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AuthorizationSchema {
    fn default() -> Self {
        Self {
            resource_types: Default::default(),
        }
    }
}
///Does the external IDP support backchannel logout?
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Backchannel logout",
///  "description": "Does the external IDP support backchannel logout?",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum BackchannelLogout {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for BackchannelLogout {
    fn from(value: &BackchannelLogout) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BackchannelLogout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for BackchannelLogout {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BackchannelLogout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BackchannelLogout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BackchannelLogout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Backchannel Token Delivery Mode",
///  "description": "Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.",
///  "type": "string",
///  "enum": [
///    "ping",
///    "poll"
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
pub enum BackchannelTokenDeliveryMode {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "poll")]
    Poll,
}
impl ::std::convert::From<&Self> for BackchannelTokenDeliveryMode {
    fn from(value: &BackchannelTokenDeliveryMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BackchannelTokenDeliveryMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ping => write!(f, "ping"),
            Self::Poll => write!(f, "poll"),
        }
    }
}
impl ::std::str::FromStr for BackchannelTokenDeliveryMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ping" => Ok(Self::Ping),
            "poll" => Ok(Self::Poll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BackchannelTokenDeliveryMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BackchannelTokenDeliveryMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BackchannelTokenDeliveryMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///BruteForceStrategy
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
impl ::std::convert::From<&Self> for BruteForceStrategy {
    fn from(value: &BruteForceStrategy) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BruteForceStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Linear => write!(f, "LINEAR"),
            Self::Multiple => write!(f, "MULTIPLE"),
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
///CertificateRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&CertificateRepresentation> for CertificateRepresentation {
    fn from(value: &CertificateRepresentation) -> Self {
        value.clone()
    }
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
///ClaimRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClaimRepresentation> for ClaimRepresentation {
    fn from(value: &ClaimRepresentation) -> Self {
        value.clone()
    }
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
///Client Authenticator used for authentication of this client against Keycloak server
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Client Authenticator",
///  "description": "Client Authenticator used for authentication of this client against Keycloak server",
///  "type": "string",
///  "enum": [
///    "client-jwt",
///    "client-secret",
///    "client-secret-jwt",
///    "client-x509"
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
pub enum ClientAuthenticator {
    #[serde(rename = "client-jwt")]
    ClientJwt,
    #[serde(rename = "client-secret")]
    ClientSecret,
    #[serde(rename = "client-secret-jwt")]
    ClientSecretJwt,
    #[serde(rename = "client-x509")]
    ClientX509,
}
impl ::std::convert::From<&Self> for ClientAuthenticator {
    fn from(value: &ClientAuthenticator) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ClientAuthenticator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientJwt => write!(f, "client-jwt"),
            Self::ClientSecret => write!(f, "client-secret"),
            Self::ClientSecretJwt => write!(f, "client-secret-jwt"),
            Self::ClientX509 => write!(f, "client-x509"),
        }
    }
}
impl ::std::str::FromStr for ClientAuthenticator {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client-jwt" => Ok(Self::ClientJwt),
            "client-secret" => Ok(Self::ClientSecret),
            "client-secret-jwt" => Ok(Self::ClientSecretJwt),
            "client-x509" => Ok(Self::ClientX509),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientAuthenticator {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClientAuthenticator {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientAuthenticator {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///ClientInitialAccessCreatePresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "count": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "expiration": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientInitialAccessCreatePresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expiration: ::std::option::Option<i32>,
}
impl ::std::convert::From<&ClientInitialAccessCreatePresentation>
for ClientInitialAccessCreatePresentation {
    fn from(value: &ClientInitialAccessCreatePresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ClientInitialAccessCreatePresentation {
    fn default() -> Self {
        Self {
            count: Default::default(),
            expiration: Default::default(),
        }
    }
}
///ClientInitialAccessPresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "count": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "expiration": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "id": {
///      "type": "string"
///    },
///    "remainingCount": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "token": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientInitialAccessPresentation>
for ClientInitialAccessPresentation {
    fn from(value: &ClientInitialAccessPresentation) -> Self {
        value.clone()
    }
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
///ClientMappingsRepresentation
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
///        "$ref": "#/$defs/RoleRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientMappingsRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mappings: ::std::vec::Vec<RoleRepresentation>,
}
impl ::std::convert::From<&ClientMappingsRepresentation>
for ClientMappingsRepresentation {
    fn from(value: &ClientMappingsRepresentation) -> Self {
        value.clone()
    }
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
///Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Client Offline Session Idle",
///  "description": "Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct ClientOfflineSessionIdle(::std::string::String);
impl ::std::ops::Deref for ClientOfflineSessionIdle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ClientOfflineSessionIdle> for ::std::string::String {
    fn from(value: ClientOfflineSessionIdle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClientOfflineSessionIdle> for ClientOfflineSessionIdle {
    fn from(value: &ClientOfflineSessionIdle) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ClientOfflineSessionIdle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ClientOfflineSessionIdle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClientOfflineSessionIdle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientOfflineSessionIdle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ClientOfflineSessionIdle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Client Offline Session Max",
///  "description": "Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct ClientOfflineSessionMax(::std::string::String);
impl ::std::ops::Deref for ClientOfflineSessionMax {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ClientOfflineSessionMax> for ::std::string::String {
    fn from(value: ClientOfflineSessionMax) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClientOfflineSessionMax> for ClientOfflineSessionMax {
    fn from(value: &ClientOfflineSessionMax) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ClientOfflineSessionMax {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ClientOfflineSessionMax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClientOfflineSessionMax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientOfflineSessionMax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ClientOfflineSessionMax {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///ClientPoliciesRepresentation
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
///        "$ref": "#/$defs/ClientPolicyRepresentation"
///      }
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClientPolicyRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientPoliciesRepresentation>
for ClientPoliciesRepresentation {
    fn from(value: &ClientPoliciesRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ClientPoliciesRepresentation {
    fn default() -> Self {
        Self {
            global_policies: Default::default(),
            policies: Default::default(),
        }
    }
}
///ClientPolicyConditionRepresentation
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
///      "type": "object"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientPolicyConditionRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub configuration: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ClientPolicyConditionRepresentation>
for ClientPolicyConditionRepresentation {
    fn from(value: &ClientPolicyConditionRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ClientPolicyConditionRepresentation {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            configuration: Default::default(),
        }
    }
}
///ClientPolicyExecutorRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "configuration": {
///      "type": "object"
///    },
///    "executor": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientPolicyExecutorRepresentation {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub configuration: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub executor: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ClientPolicyExecutorRepresentation>
for ClientPolicyExecutorRepresentation {
    fn from(value: &ClientPolicyExecutorRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ClientPolicyExecutorRepresentation {
    fn default() -> Self {
        Self {
            configuration: Default::default(),
            executor: Default::default(),
        }
    }
}
///ClientPolicyRepresentation
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
///        "$ref": "#/$defs/ClientPolicyConditionRepresentation"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientPolicyRepresentation> for ClientPolicyRepresentation {
    fn from(value: &ClientPolicyRepresentation) -> Self {
        value.clone()
    }
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
///ClientProfileRepresentation
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
///        "$ref": "#/$defs/ClientPolicyExecutorRepresentation"
///      }
///    },
///    "name": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientProfileRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub executors: ::std::vec::Vec<ClientPolicyExecutorRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ClientProfileRepresentation> for ClientProfileRepresentation {
    fn from(value: &ClientProfileRepresentation) -> Self {
        value.clone()
    }
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
///ClientProfilesRepresentation
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
///        "$ref": "#/$defs/ClientProfileRepresentation"
///      }
///    },
///    "profiles": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClientProfileRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientProfilesRepresentation>
for ClientProfilesRepresentation {
    fn from(value: &ClientProfilesRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ClientProfilesRepresentation {
    fn default() -> Self {
        Self {
            global_profiles: Default::default(),
            profiles: Default::default(),
        }
    }
}
///ClientRepresentation
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
///      "title": "Admin URL",
///      "description": "URL to the admin interface of the client. Set this if the client supports the adapter REST API. This REST API allows the auth server to push revocation policies and other administrative tasks. Usually this is set to the base URL of the client.",
///      "type": "string"
///    },
///    "alwaysDisplayInConsole": {
///      "title": "Always display in UI",
///      "description": "Always list this client in the Account UI, even if the user does not have an active session.",
///      "type": "boolean"
///    },
///    "attributes": {
///      "type": "object",
///      "properties": {
///        "access.token.lifespan": {
///          "title": "Access Token Lifespan",
///          "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "access.token.signed.response.alg": {
///          "title": "Access token signature algorithm",
///          "description": "JWA algorithm used for signing access tokens.",
///          "type": "string"
///        },
///        "authorization.encrypted.response.alg": {
///          "title": "Authorization response encryption key management algorithm",
///          "description": "JWA Algorithm used for key management in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.",
///          "type": "string"
///        },
///        "authorization.encrypted.response.enc": {
///          "title": "Authorization response encryption content encryption algorithm",
///          "description": "JWA Algorithm used for content encryption in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.",
///          "type": "string"
///        },
///        "authorization.signed.response.alg": {
///          "title": "Authorization response signature algorithm",
///          "description": "JWA algorithm used for signing authorization response tokens when the response mode is jwt.",
///          "type": "string"
///        },
///        "client.offline.session.idle.timeout": {
///          "title": "Client Offline Session Idle",
///          "description": "Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "client.offline.session.max.lifespan": {
///          "title": "Client Offline Session Max",
///          "description": "Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "client.session.idle.timeout": {
///          "title": "Client Session Idle",
///          "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "client.session.max.lifespan": {
///          "title": "Client Session Max",
///          "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "client_credentials.use_refresh_token": {
///          "title": "Use refresh tokens for client credentials grant",
///          "description": "If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "exclude.session.state.from.auth.response": {
///          "title": "Exclude Session State From Authentication Response",
///          "description": "If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "id.token.encrypted.response.alg": {
///          "title": "ID token encryption key management algorithm",
///          "description": "JWA Algorithm used for key management in encrypting ID tokens. This option is needed if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.",
///          "type": "string"
///        },
///        "id.token.encrypted.response.enc": {
///          "title": "ID token encryption content encryption algorithm",
///          "description": "JWA Algorithm used for content encryption in encrypting ID tokens. This option is needed just if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.",
///          "type": "string"
///        },
///        "id.token.signed.response.alg": {
///          "title": "ID token signature algorithm",
///          "description": "JWA algorithm used for signing ID tokens.",
///          "type": "string"
///        },
///        "logoUri": {
///          "title": "Logo URL",
///          "description": "URL that references a logo for the Client application",
///          "type": "string"
///        },
///        "pkce.code.challenge.method": {
///          "title": "Proof Key for Code Exchange Code Challenge Method",
///          "description": "Choose which code challenge method for PKCE is used. If not specified, keycloak does not applies PKCE to a client unless the client sends an authorization request with appropriate code challenge and code exchange method.",
///          "type": "string"
///        },
///        "policyUri": {
///          "title": "Policy URL",
///          "description": "URL that the Relying Party Client provides to the End-User to read about the how the profile data will be used",
///          "type": "string"
///        },
///        "post.logout.redirect.uris": {
///          "title": "Valid post logout redirect URIs",
///          "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.",
///          "type": "string"
///        },
///        "request.object.encryption.alg": {
///          "title": "Request object encryption algorithm",
///          "description": "JWE algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', encryption is optional and any algorithm is allowed.",
///          "type": "string"
///        },
///        "request.object.encryption.enc": {
///          "title": "Request object content encryption algorithm",
///          "description": "JWE algorithm, which client needs to use when encrypting the content of the OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', any algorithm is allowed.",
///          "type": "string"
///        },
///        "request.object.required": {
///          "title": "Request object required",
///          "description": "Specifies if the client needs to provide a request object with their authorization requests, and what method they can use for this. If set to \"not required\", providing a request object is optional. In all other cases, providing a request object is mandatory. If set to \"request\", the request object must be provided by value. If set to \"request_uri\", the request object must be provided by reference. If set to \"request or request_uri\", either method can be used.",
///          "type": "string"
///        },
///        "request.object.signature.alg": {
///          "title": "Request object signature algorithm",
///          "description": "JWA algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', Request object can be signed by any algorithm (including 'none' ).",
///          "type": "string"
///        },
///        "require.pushed.authorization.requests": {
///          "title": "Pushed authorization request required",
///          "description": "Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "tls.client.certificate.bound.access.tokens": {
///          "title": "OAuth 2.0 Mutual TLS Certificate Bound Access Tokens Enabled",
///          "description": "This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "token.endpoint.auth.signing.alg": {
///          "title": "Signature algorithm",
///          "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.",
///          "type": "string"
///        },
///        "token.response.type.bearer.lower-case": {
///          "title": "Use lower-case bearer type in token responses",
///          "description": "If this is on, token responses will be set the with the type \"bearer\" in lower-case. By default, the server sets the type as \"Bearer\" as defined by RFC6750.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "tosUri": {
///          "title": "Terms of service URL",
///          "description": "URL that the Relying Party Client provides to the End-User to read about the Relying Party's terms of service",
///          "type": "string"
///        },
///        "use.refresh.tokens": {
///          "title": "Use refresh tokens",
///          "description": "If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "user.info.encrypted.response.alg": {
///          "title": "User info response encryption key management algorithm",
///          "description": "JWA Algorithm used for key management in encrypting User Info Endpoint responses. This option is needed if you want encrypted User Info Endpoint responses. If left empty, User Info Endpoint responses are not encrypted.",
///          "type": "string"
///        },
///        "user.info.encrypted.response.enc": {
///          "title": "User info response encryption content encryption algorithm",
///          "description": "JWA Algorithm used for content encryption in encrypting User Info Endpoint responses. If User Info response encryption key management algorithm is specified, the default for this value is A128CBC-HS256.",
///          "type": "string"
///        },
///        "user.info.response.signature.alg": {
///          "title": "User info signed response algorithm",
///          "description": "JWA algorithm used for signed User Info Endpoint response. If set to 'unsigned', User Info Response won't be signed and will be returned in application/json format.",
///          "type": "string"
///        },
///        "x509.allow.regex.pattern.comparison": {
///          "title": "Allow regex pattern comparison",
///          "description": "If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "x509.subjectdn": {
///          "title": "Subject DN",
///          "description": "A regular expression for validating Subject DN in the Client Certificate. Use \"(.*?)(?:$)\" to match all kind of expressions.",
///          "type": "string"
///        }
///      },
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authenticationFlowBindingOverrides": {
///      "title": "Authentication flow overrides",
///      "type": "object",
///      "properties": {
///        "browser": {
///          "title": "Browser Flow",
///          "description": "Select the flow you want to use for browser authentication.",
///          "type": "string"
///        },
///        "direct_grant": {
///          "title": "Direct Grant Flow",
///          "description": "Select the flow you want to use for direct grant authentication.",
///          "type": "string"
///        }
///      },
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authorizationServicesEnabled": {
///      "title": "Authorization",
///      "description": "Enable/Disable fine-grained authorization support for a client.",
///      "type": "boolean"
///    },
///    "authorizationSettings": {
///      "$ref": "#/$defs/ResourceServerRepresentation"
///    },
///    "baseUrl": {
///      "title": "Home URL",
///      "description": "Default URL to use when the auth server needs to redirect or link back to the client.",
///      "type": "string"
///    },
///    "bearerOnly": {
///      "description": "This is a special OIDC type. This client only allows bearer token requests and cannot participate in browser logins.",
///      "type": "boolean"
///    },
///    "clientAuthenticatorType": {
///      "title": "Client Authenticator",
///      "description": "Client Authenticator used for authentication of this client against Keycloak server",
///      "type": "string",
///      "enum": [
///        "client-jwt",
///        "client-secret",
///        "client-secret-jwt",
///        "client-x509"
///      ]
///    },
///    "clientId": {
///      "title": "Client ID",
///      "description": "The client identifier registered with the identity provider.",
///      "type": "string"
///    },
///    "clientTemplate": {
///      "type": "string"
///    },
///    "consentRequired": {
///      "title": "Consent required",
///      "description": "If enabled, users have to consent to client access.",
///      "type": "boolean"
///    },
///    "defaultClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultRoles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "description": {
///      "title": "Description",
///      "description": "Help text for the description of the new flow",
///      "type": "string"
///    },
///    "directAccessGrantsEnabled": {
///      "title": "Direct access grants",
///      "description": "This enables support for Direct Access Grants, which means that client has access to username/password of user and exchange it directly with Keycloak server for access token. In terms of OAuth2 specification, this enables support of 'Resource Owner Password Credentials Grant' for this client.",
///      "type": "boolean"
///    },
///    "directGrantsOnly": {
///      "type": "boolean"
///    },
///    "enabled": {
///      "title": "Enabled",
///      "description": "Disabled clients cannot initiate a login or have obtained access tokens.",
///      "type": "boolean"
///    },
///    "frontchannelLogout": {
///      "title": "Front channel logout",
///      "description": "When true, logout requires a browser redirect to client. When false, server performs a background invocation for logout.",
///      "type": "boolean"
///    },
///    "fullScopeAllowed": {
///      "title": "Full scope allowed",
///      "description": "Allows you to disable all restrictions.",
///      "type": "boolean"
///    },
///    "id": {
///      "type": "string"
///    },
///    "implicitFlowEnabled": {
///      "title": "Implicit flow",
///      "description": "This enables support for OpenID Connect redirect based authentication without authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Implicit Flow' for this client.",
///      "type": "boolean"
///    },
///    "name": {
///      "title": "Name",
///      "description": "Specifies display name of the client. For example 'My Client'. Supports keys for localized values as well. For example: ${my_client}.",
///      "type": "string"
///    },
///    "nodeReRegistrationTimeout": {
///      "title": "Node Re-registration timeout",
///      "description": "Interval to specify max time for registered clients cluster nodes to re-register. If cluster node will not send re-registration request to Keycloak within this time, it will be unregistered from Keycloak.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///      "title": "Protocol",
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ProtocolMapperRepresentation"
///      }
///    },
///    "publicClient": {
///      "title": "Client authentication",
///      "description": "This defines the type of the OIDC client. When it's ON, the OIDC type is set to confidential access type. When it's OFF, it is set to public access type.",
///      "type": "boolean"
///    },
///    "redirectUris": {
///      "title": "Valid redirect URIs",
///      "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "registeredNodes": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "int32",
///        "maximum": 2147483647.0,
///        "minimum": -2147483648.0
///      }
///    },
///    "registrationAccessToken": {
///      "title": "Registration access token",
///      "description": "The registration access token provides access for clients to the client registration service.",
///      "type": "string"
///    },
///    "rootUrl": {
///      "title": "Root URL",
///      "description": "Root URL appended to relative URLs",
///      "type": "string"
///    },
///    "secret": {
///      "title": "Client Secret",
///      "type": "string"
///    },
///    "serviceAccountsEnabled": {
///      "title": "Service accounts roles",
///      "description": "Allows you to authenticate this client to Keycloak and retrieve access token dedicated to this client. In terms of OAuth2 specification, this enables support of 'Client Credentials Grant' for this client.",
///      "type": "boolean"
///    },
///    "standardFlowEnabled": {
///      "title": "Standard flow",
///      "description": "This enables standard OpenID Connect redirect based authentication with authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Authorization Code Flow' for this client.",
///      "type": "boolean"
///    },
///    "surrogateAuthRequired": {
///      "type": "boolean"
///    },
///    "type": {
///      "type": "string"
///    },
///    "useTemplateConfig": {
///      "type": "boolean"
///    },
///    "useTemplateMappers": {
///      "type": "boolean"
///    },
///    "useTemplateScope": {
///      "type": "boolean"
///    },
///    "webOrigins": {
///      "title": "Web origins",
///      "description": "Allowed CORS origins. To permit all origins of Valid Redirect URIs, add '+'. This does not include the '*' wildcard though. To permit all origins, explicitly add '*'.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientRepresentation {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: BTreeMap::is_empty"
    )]
    pub access: ::std::collections::BTreeMap<::std::string::String, bool>,
    ///URL to the admin interface of the client. Set this if the client supports the adapter REST API. This REST API allows the auth server to push revocation policies and other administrative tasks. Usually this is set to the base URL of the client.
    #[serde(
        rename = "adminUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_url: ::std::option::Option<::std::string::String>,
    ///Always list this client in the Account UI, even if the user does not have an active session.
    #[serde(
        rename = "alwaysDisplayInConsole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub always_display_in_console: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attributes: ::std::option::Option<ClientRepresentationAttributes>,
    #[serde(
        rename = "authenticationFlowBindingOverrides",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authentication_flow_binding_overrides: ::std::option::Option<
        AuthenticationFlowOverrides,
    >,
    ///Enable/Disable fine-grained authorization support for a client.
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
    ///Default URL to use when the auth server needs to redirect or link back to the client.
    #[serde(
        rename = "baseUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_url: ::std::option::Option<::std::string::String>,
    ///This is a special OIDC type. This client only allows bearer token requests and cannot participate in browser logins.
    #[serde(
        rename = "bearerOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_only: ::std::option::Option<bool>,
    ///Client Authenticator used for authentication of this client against Keycloak server
    #[serde(
        rename = "clientAuthenticatorType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_authenticator_type: ::std::option::Option<ClientAuthenticator>,
    ///The client identifier registered with the identity provider.
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
    ///If enabled, users have to consent to client access.
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
    ///Help text for the description of the new flow
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///This enables support for Direct Access Grants, which means that client has access to username/password of user and exchange it directly with Keycloak server for access token. In terms of OAuth2 specification, this enables support of 'Resource Owner Password Credentials Grant' for this client.
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
    ///Disabled clients cannot initiate a login or have obtained access tokens.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    ///When true, logout requires a browser redirect to client. When false, server performs a background invocation for logout.
    #[serde(
        rename = "frontchannelLogout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frontchannel_logout: ::std::option::Option<bool>,
    ///Allows you to disable all restrictions.
    #[serde(
        rename = "fullScopeAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_scope_allowed: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    ///This enables support for OpenID Connect redirect based authentication without authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Implicit Flow' for this client.
    #[serde(
        rename = "implicitFlowEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub implicit_flow_enabled: ::std::option::Option<bool>,
    ///Specifies display name of the client. For example 'My Client'. Supports keys for localized values as well. For example: ${my_client}.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Interval to specify max time for registered clients cluster nodes to re-register. If cluster node will not send re-registration request to Keycloak within this time, it will be unregistered from Keycloak.
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
    ///This defines the type of the OIDC client. When it's ON, the OIDC type is set to confidential access type. When it's OFF, it is set to public access type.
    #[serde(
        rename = "publicClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_client: ::std::option::Option<bool>,
    ///Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.
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
    ///The registration access token provides access for clients to the client registration service.
    #[serde(
        rename = "registrationAccessToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_access_token: ::std::option::Option<::std::string::String>,
    ///Root URL appended to relative URLs
    #[serde(
        rename = "rootUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub root_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    ///Allows you to authenticate this client to Keycloak and retrieve access token dedicated to this client. In terms of OAuth2 specification, this enables support of 'Client Credentials Grant' for this client.
    #[serde(
        rename = "serviceAccountsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_accounts_enabled: ::std::option::Option<bool>,
    ///This enables standard OpenID Connect redirect based authentication with authorization code. In terms of OpenID Connect or OAuth2 specifications, this enables support of 'Authorization Code Flow' for this client.
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
    ///Allowed CORS origins. To permit all origins of Valid Redirect URIs, add '+'. This does not include the '*' wildcard though. To permit all origins, explicitly add '*'.
    #[serde(
        rename = "webOrigins",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_origins: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&ClientRepresentation> for ClientRepresentation {
    fn from(value: &ClientRepresentation) -> Self {
        value.clone()
    }
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
///ClientRepresentationAttributes
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "access.token.lifespan": {
///      "title": "Access Token Lifespan",
///      "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "access.token.signed.response.alg": {
///      "title": "Access token signature algorithm",
///      "description": "JWA algorithm used for signing access tokens.",
///      "type": "string"
///    },
///    "authorization.encrypted.response.alg": {
///      "title": "Authorization response encryption key management algorithm",
///      "description": "JWA Algorithm used for key management in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.",
///      "type": "string"
///    },
///    "authorization.encrypted.response.enc": {
///      "title": "Authorization response encryption content encryption algorithm",
///      "description": "JWA Algorithm used for content encryption in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.",
///      "type": "string"
///    },
///    "authorization.signed.response.alg": {
///      "title": "Authorization response signature algorithm",
///      "description": "JWA algorithm used for signing authorization response tokens when the response mode is jwt.",
///      "type": "string"
///    },
///    "client.offline.session.idle.timeout": {
///      "title": "Client Offline Session Idle",
///      "description": "Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "client.offline.session.max.lifespan": {
///      "title": "Client Offline Session Max",
///      "description": "Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "client.session.idle.timeout": {
///      "title": "Client Session Idle",
///      "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "client.session.max.lifespan": {
///      "title": "Client Session Max",
///      "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "client_credentials.use_refresh_token": {
///      "title": "Use refresh tokens for client credentials grant",
///      "description": "If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "exclude.session.state.from.auth.response": {
///      "title": "Exclude Session State From Authentication Response",
///      "description": "If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "id.token.encrypted.response.alg": {
///      "title": "ID token encryption key management algorithm",
///      "description": "JWA Algorithm used for key management in encrypting ID tokens. This option is needed if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.",
///      "type": "string"
///    },
///    "id.token.encrypted.response.enc": {
///      "title": "ID token encryption content encryption algorithm",
///      "description": "JWA Algorithm used for content encryption in encrypting ID tokens. This option is needed just if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.",
///      "type": "string"
///    },
///    "id.token.signed.response.alg": {
///      "title": "ID token signature algorithm",
///      "description": "JWA algorithm used for signing ID tokens.",
///      "type": "string"
///    },
///    "logoUri": {
///      "title": "Logo URL",
///      "description": "URL that references a logo for the Client application",
///      "type": "string"
///    },
///    "pkce.code.challenge.method": {
///      "title": "Proof Key for Code Exchange Code Challenge Method",
///      "description": "Choose which code challenge method for PKCE is used. If not specified, keycloak does not applies PKCE to a client unless the client sends an authorization request with appropriate code challenge and code exchange method.",
///      "type": "string"
///    },
///    "policyUri": {
///      "title": "Policy URL",
///      "description": "URL that the Relying Party Client provides to the End-User to read about the how the profile data will be used",
///      "type": "string"
///    },
///    "post.logout.redirect.uris": {
///      "title": "Valid post logout redirect URIs",
///      "description": "Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.",
///      "type": "string"
///    },
///    "request.object.encryption.alg": {
///      "title": "Request object encryption algorithm",
///      "description": "JWE algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', encryption is optional and any algorithm is allowed.",
///      "type": "string"
///    },
///    "request.object.encryption.enc": {
///      "title": "Request object content encryption algorithm",
///      "description": "JWE algorithm, which client needs to use when encrypting the content of the OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', any algorithm is allowed.",
///      "type": "string"
///    },
///    "request.object.required": {
///      "title": "Request object required",
///      "description": "Specifies if the client needs to provide a request object with their authorization requests, and what method they can use for this. If set to \"not required\", providing a request object is optional. In all other cases, providing a request object is mandatory. If set to \"request\", the request object must be provided by value. If set to \"request_uri\", the request object must be provided by reference. If set to \"request or request_uri\", either method can be used.",
///      "type": "string"
///    },
///    "request.object.signature.alg": {
///      "title": "Request object signature algorithm",
///      "description": "JWA algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', Request object can be signed by any algorithm (including 'none' ).",
///      "type": "string"
///    },
///    "require.pushed.authorization.requests": {
///      "title": "Pushed authorization request required",
///      "description": "Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "tls.client.certificate.bound.access.tokens": {
///      "title": "OAuth 2.0 Mutual TLS Certificate Bound Access Tokens Enabled",
///      "description": "This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "token.endpoint.auth.signing.alg": {
///      "title": "Signature algorithm",
///      "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.",
///      "type": "string"
///    },
///    "token.response.type.bearer.lower-case": {
///      "title": "Use lower-case bearer type in token responses",
///      "description": "If this is on, token responses will be set the with the type \"bearer\" in lower-case. By default, the server sets the type as \"Bearer\" as defined by RFC6750.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "tosUri": {
///      "title": "Terms of service URL",
///      "description": "URL that the Relying Party Client provides to the End-User to read about the Relying Party's terms of service",
///      "type": "string"
///    },
///    "use.refresh.tokens": {
///      "title": "Use refresh tokens",
///      "description": "If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "user.info.encrypted.response.alg": {
///      "title": "User info response encryption key management algorithm",
///      "description": "JWA Algorithm used for key management in encrypting User Info Endpoint responses. This option is needed if you want encrypted User Info Endpoint responses. If left empty, User Info Endpoint responses are not encrypted.",
///      "type": "string"
///    },
///    "user.info.encrypted.response.enc": {
///      "title": "User info response encryption content encryption algorithm",
///      "description": "JWA Algorithm used for content encryption in encrypting User Info Endpoint responses. If User Info response encryption key management algorithm is specified, the default for this value is A128CBC-HS256.",
///      "type": "string"
///    },
///    "user.info.response.signature.alg": {
///      "title": "User info signed response algorithm",
///      "description": "JWA algorithm used for signed User Info Endpoint response. If set to 'unsigned', User Info Response won't be signed and will be returned in application/json format.",
///      "type": "string"
///    },
///    "x509.allow.regex.pattern.comparison": {
///      "title": "Allow regex pattern comparison",
///      "description": "If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "x509.subjectdn": {
///      "title": "Subject DN",
///      "description": "A regular expression for validating Subject DN in the Client Certificate. Use \"(.*?)(?:$)\" to match all kind of expressions.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ClientRepresentationAttributes {
    ///Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.
    #[serde(
        rename = "access.token.lifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_token_lifespan: ::std::option::Option<AccessTokenLifespan>,
    ///JWA algorithm used for signing access tokens.
    #[serde(
        rename = "access.token.signed.response.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_token_signed_response_alg: ::std::option::Option<::std::string::String>,
    ///JWA Algorithm used for key management in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.
    #[serde(
        rename = "authorization.encrypted.response.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_encrypted_response_alg: ::std::option::Option<
        ::std::string::String,
    >,
    ///JWA Algorithm used for content encryption in encrypting the authorization response when the response mode is jwt. This option is needed if you want encrypted authorization response. If left empty, the authorization response is just signed, but not encrypted.
    #[serde(
        rename = "authorization.encrypted.response.enc",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_encrypted_response_enc: ::std::option::Option<
        ::std::string::String,
    >,
    ///JWA algorithm used for signing authorization response tokens when the response mode is jwt.
    #[serde(
        rename = "authorization.signed.response.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_signed_response_alg: ::std::option::Option<::std::string::String>,
    ///If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.
    #[serde(
        rename = "client_credentials.use_refresh_token",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_credentials_use_refresh_token: ::std::option::Option<
        UseRefreshTokensForClientCredentialsGrant,
    >,
    ///Time a client offline session is allowed to be idle before it expires. Offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Idle value.
    #[serde(
        rename = "client.offline.session.idle.timeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_offline_session_idle_timeout: ::std::option::Option<
        ClientOfflineSessionIdle,
    >,
    ///Max time before a client offline session is expired. If Offline Session Max Limited is enabled at realm level, offline tokens are invalidated when a client offline session is expired. The option does not affect the global user SSO session. If not set, it uses the realm Offline Session Max value.
    #[serde(
        rename = "client.offline.session.max.lifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_offline_session_max_lifespan: ::std::option::Option<
        ClientOfflineSessionMax,
    >,
    ///Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.
    #[serde(
        rename = "client.session.idle.timeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_session_idle_timeout: ::std::option::Option<ClientSessionIdle>,
    ///Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.
    #[serde(
        rename = "client.session.max.lifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_session_max_lifespan: ::std::option::Option<ClientSessionMax>,
    ///If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.
    #[serde(
        rename = "exclude.session.state.from.auth.response",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exclude_session_state_from_auth_response: ::std::option::Option<
        ExcludeSessionStateFromAuthenticationResponse,
    >,
    ///JWA Algorithm used for key management in encrypting ID tokens. This option is needed if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.
    #[serde(
        rename = "id.token.encrypted.response.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id_token_encrypted_response_alg: ::std::option::Option<::std::string::String>,
    ///JWA Algorithm used for content encryption in encrypting ID tokens. This option is needed just if you want encrypted ID tokens. If left empty, ID Tokens are just signed, but not encrypted.
    #[serde(
        rename = "id.token.encrypted.response.enc",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id_token_encrypted_response_enc: ::std::option::Option<::std::string::String>,
    ///JWA algorithm used for signing ID tokens.
    #[serde(
        rename = "id.token.signed.response.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id_token_signed_response_alg: ::std::option::Option<::std::string::String>,
    ///URL that references a logo for the Client application
    #[serde(
        rename = "logoUri",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub logo_uri: ::std::option::Option<::std::string::String>,
    ///Choose which code challenge method for PKCE is used. If not specified, keycloak does not applies PKCE to a client unless the client sends an authorization request with appropriate code challenge and code exchange method.
    #[serde(
        rename = "pkce.code.challenge.method",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pkce_code_challenge_method: ::std::option::Option<::std::string::String>,
    ///URL that the Relying Party Client provides to the End-User to read about the how the profile data will be used
    #[serde(
        rename = "policyUri",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub policy_uri: ::std::option::Option<::std::string::String>,
    ///Valid URI pattern a browser can redirect to after a successful login. Simple wildcards are allowed such as 'http://example.com/*'. Relative path can be specified too such as /my/relative/path/*. Relative paths are relative to the client root URL, or if none is specified the auth server root URL is used. For SAML, you must set valid URI patterns if you are relying on the consumer service URL embedded with the login request.
    #[serde(
        rename = "post.logout.redirect.uris",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub post_logout_redirect_uris: ::std::option::Option<::std::string::String>,
    ///JWE algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', encryption is optional and any algorithm is allowed.
    #[serde(
        rename = "request.object.encryption.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_object_encryption_alg: ::std::option::Option<::std::string::String>,
    ///JWE algorithm, which client needs to use when encrypting the content of the OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', any algorithm is allowed.
    #[serde(
        rename = "request.object.encryption.enc",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_object_encryption_enc: ::std::option::Option<::std::string::String>,
    ///Specifies if the client needs to provide a request object with their authorization requests, and what method they can use for this. If set to "not required", providing a request object is optional. In all other cases, providing a request object is mandatory. If set to "request", the request object must be provided by value. If set to "request_uri", the request object must be provided by reference. If set to "request or request_uri", either method can be used.
    #[serde(
        rename = "request.object.required",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_object_required: ::std::option::Option<::std::string::String>,
    ///JWA algorithm, which client needs to use when sending OIDC request object specified by 'request' or 'request_uri' parameters. If set to 'any', Request object can be signed by any algorithm (including 'none' ).
    #[serde(
        rename = "request.object.signature.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_object_signature_alg: ::std::option::Option<::std::string::String>,
    ///Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.
    #[serde(
        rename = "require.pushed.authorization.requests",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub require_pushed_authorization_requests: ::std::option::Option<
        PushedAuthorizationRequestRequired,
    >,
    ///This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.
    #[serde(
        rename = "tls.client.certificate.bound.access.tokens",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tls_client_certificate_bound_access_tokens: ::std::option::Option<
        OAuth20MutualTlsCertificateBoundAccessTokensEnabled,
    >,
    ///The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.
    #[serde(
        rename = "token.endpoint.auth.signing.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_endpoint_auth_signing_alg: ::std::option::Option<::std::string::String>,
    ///If this is on, token responses will be set the with the type "bearer" in lower-case. By default, the server sets the type as "Bearer" as defined by RFC6750.
    #[serde(
        rename = "token.response.type.bearer.lower-case",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_response_type_bearer_lower_case: ::std::option::Option<
        UseLowerCaseBearerTypeInTokenResponses,
    >,
    ///URL that the Relying Party Client provides to the End-User to read about the Relying Party's terms of service
    #[serde(
        rename = "tosUri",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tos_uri: ::std::option::Option<::std::string::String>,
    ///If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.
    #[serde(
        rename = "use.refresh.tokens",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_refresh_tokens: ::std::option::Option<UseRefreshTokens>,
    ///JWA Algorithm used for key management in encrypting User Info Endpoint responses. This option is needed if you want encrypted User Info Endpoint responses. If left empty, User Info Endpoint responses are not encrypted.
    #[serde(
        rename = "user.info.encrypted.response.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_info_encrypted_response_alg: ::std::option::Option<::std::string::String>,
    ///JWA Algorithm used for content encryption in encrypting User Info Endpoint responses. If User Info response encryption key management algorithm is specified, the default for this value is A128CBC-HS256.
    #[serde(
        rename = "user.info.encrypted.response.enc",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_info_encrypted_response_enc: ::std::option::Option<::std::string::String>,
    ///JWA algorithm used for signed User Info Endpoint response. If set to 'unsigned', User Info Response won't be signed and will be returned in application/json format.
    #[serde(
        rename = "user.info.response.signature.alg",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_info_response_signature_alg: ::std::option::Option<::std::string::String>,
    ///If OFF, then the Subject DN from given client certificate must exactly match the given DN from the 'Subject DN' property as described in the RFC8705 specification. The Subject DN can be in the RFC4514 or RFC1779 format. If ON, then the Subject DN from given client certificate should match regex specified by 'Subject DN' property.
    #[serde(
        rename = "x509.allow.regex.pattern.comparison",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x509_allow_regex_pattern_comparison: ::std::option::Option<
        AllowRegexPatternComparison,
    >,
    ///A regular expression for validating Subject DN in the Client Certificate. Use "(.*?)(?:$)" to match all kind of expressions.
    #[serde(
        rename = "x509.subjectdn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x509_subjectdn: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::convert::From<&ClientRepresentationAttributes>
for ClientRepresentationAttributes {
    fn from(value: &ClientRepresentationAttributes) -> Self {
        value.clone()
    }
}
///ClientScopeRepresentation
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
///      "type": "string",
///      "enum": [
///        "openid-connect",
///        "saml"
///      ]
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ProtocolMapperRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
    pub protocol: ::std::option::Option<ClientScopeRepresentationProtocol>,
    #[serde(
        rename = "protocolMappers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub protocol_mappers: ::std::vec::Vec<ProtocolMapperRepresentation>,
}
impl ::std::convert::From<&ClientScopeRepresentation> for ClientScopeRepresentation {
    fn from(value: &ClientScopeRepresentation) -> Self {
        value.clone()
    }
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
///ClientScopeRepresentationProtocol
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "openid-connect",
///    "saml"
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
pub enum ClientScopeRepresentationProtocol {
    #[serde(rename = "openid-connect")]
    OpenidConnect,
    #[serde(rename = "saml")]
    Saml,
}
impl ::std::convert::From<&Self> for ClientScopeRepresentationProtocol {
    fn from(value: &ClientScopeRepresentationProtocol) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ClientScopeRepresentationProtocol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OpenidConnect => write!(f, "openid-connect"),
            Self::Saml => write!(f, "saml"),
        }
    }
}
impl ::std::str::FromStr for ClientScopeRepresentationProtocol {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "openid-connect" => Ok(Self::OpenidConnect),
            "saml" => Ok(Self::Saml),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientScopeRepresentationProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ClientScopeRepresentationProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ClientScopeRepresentationProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Client Session Idle",
///  "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct ClientSessionIdle(::std::string::String);
impl ::std::ops::Deref for ClientSessionIdle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ClientSessionIdle> for ::std::string::String {
    fn from(value: ClientSessionIdle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClientSessionIdle> for ClientSessionIdle {
    fn from(value: &ClientSessionIdle) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ClientSessionIdle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ClientSessionIdle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClientSessionIdle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientSessionIdle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ClientSessionIdle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Client Session Max",
///  "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct ClientSessionMax(::std::string::String);
impl ::std::ops::Deref for ClientSessionMax {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ClientSessionMax> for ::std::string::String {
    fn from(value: ClientSessionMax) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClientSessionMax> for ClientSessionMax {
    fn from(value: &ClientSessionMax) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ClientSessionMax {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ClientSessionMax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClientSessionMax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientSessionMax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ClientSessionMax {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///ClientTemplateRepresentation
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
///        "$ref": "#/$defs/ProtocolMapperRepresentation"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientTemplateRepresentation>
for ClientTemplateRepresentation {
    fn from(value: &ClientTemplateRepresentation) -> Self {
        value.clone()
    }
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
///ClientTypeRepresentation
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
///        "$ref": "#/$defs/PropertyConfig"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientTypeRepresentation> for ClientTypeRepresentation {
    fn from(value: &ClientTypeRepresentation) -> Self {
        value.clone()
    }
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
///ClientTypesRepresentation
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
///        "$ref": "#/$defs/ClientTypeRepresentation"
///      }
///    },
///    "global-client-types": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClientTypeRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ClientTypesRepresentation> for ClientTypesRepresentation {
    fn from(value: &ClientTypesRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ClientTypesRepresentation {
    fn default() -> Self {
        Self {
            client_types: Default::default(),
            global_client_types: Default::default(),
        }
    }
}
///Specifies the comparison method used to evaluate the requested context classes or statements. The default is "Exact".
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Comparison",
///  "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".",
///  "type": "string",
///  "enum": [
///    "exact",
///    "minimum",
///    "maximum",
///    "better"
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
pub enum Comparison {
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "minimum")]
    Minimum,
    #[serde(rename = "maximum")]
    Maximum,
    #[serde(rename = "better")]
    Better,
}
impl ::std::convert::From<&Self> for Comparison {
    fn from(value: &Comparison) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Exact => write!(f, "exact"),
            Self::Minimum => write!(f, "minimum"),
            Self::Maximum => write!(f, "maximum"),
            Self::Better => write!(f, "better"),
        }
    }
}
impl ::std::str::FromStr for Comparison {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "exact" => Ok(Self::Exact),
            "minimum" => Ok(Self::Minimum),
            "maximum" => Ok(Self::Maximum),
            "better" => Ok(Self::Better),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Comparison {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Comparison {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Comparison {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///ComponentExportRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "$ref": "#/$defs/MultivaluedHashMapStringString"
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
///      "$ref": "#/$defs/MultivaluedHashMapStringComponentExportRepresentation"
///    },
///    "subType": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ComponentExportRepresentation>
for ComponentExportRepresentation {
    fn from(value: &ComponentExportRepresentation) -> Self {
        value.clone()
    }
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
///ComponentRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "$ref": "#/$defs/MultivaluedHashMapStringString"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ComponentRepresentation> for ComponentRepresentation {
    fn from(value: &ComponentRepresentation) -> Self {
        value.clone()
    }
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
///ComponentTypeRepresentation
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
///        "$ref": "#/$defs/ConfigPropertyRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ComponentTypeRepresentation> for ComponentTypeRepresentation {
    fn from(value: &ComponentTypeRepresentation) -> Self {
        value.clone()
    }
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
///Composites
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "application": {
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&Composites> for Composites {
    fn from(value: &Composites) -> Self {
        value.clone()
    }
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
///ConfigPropertyRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ConfigPropertyRepresentation>
for ConfigPropertyRepresentation {
    fn from(value: &ConfigPropertyRepresentation) -> Self {
        value.clone()
    }
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
///Confirmation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&Confirmation> for Confirmation {
    fn from(value: &Confirmation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Confirmation {
    fn default() -> Self {
        Self {
            jkt: Default::default(),
            x5t_s256: Default::default(),
        }
    }
}
///CredentialRepresentation
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
///    "config": {
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/MultivaluedHashMapStringString"
///        }
///      ]
///    },
///    "counter": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "createdDate": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "credentialData": {
///      "type": "string"
///    },
///    "device": {
///      "type": "string"
///    },
///    "digits": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "hashIterations": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "hashedSaltedValue": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "period": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "salt": {
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&CredentialRepresentation> for CredentialRepresentation {
    fn from(value: &CredentialRepresentation) -> Self {
        value.clone()
    }
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
///DecisionEffect
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
impl ::std::convert::From<&Self> for DecisionEffect {
    fn from(value: &DecisionEffect) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DecisionEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Permit => write!(f, "PERMIT"),
            Self::Deny => write!(f, "DENY"),
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
///DecisionStrategy
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
impl ::std::convert::From<&Self> for DecisionStrategy {
    fn from(value: &DecisionStrategy) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DecisionStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Affirmative => write!(f, "AFFIRMATIVE"),
            Self::Unanimous => write!(f, "UNANIMOUS"),
            Self::Consensus => write!(f, "CONSENSUS"),
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
///Default algorithm used to sign tokens for the realm
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Default Signature Algorithm",
///  "description": "Default algorithm used to sign tokens for the realm",
///  "type": "string",
///  "enum": [
///    "EdDSA",
///    "ES256",
///    "ES384",
///    "ES512",
///    "HS256",
///    "HS384",
///    "HS512",
///    "PS256",
///    "PS384",
///    "PS512",
///    "RS256",
///    "RS384",
///    "RS512"
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
pub enum DefaultSignatureAlgorithm {
    #[serde(rename = "EdDSA")]
    EdDsa,
    #[serde(rename = "ES256")]
    Es256,
    #[serde(rename = "ES384")]
    Es384,
    #[serde(rename = "ES512")]
    Es512,
    #[serde(rename = "HS256")]
    Hs256,
    #[serde(rename = "HS384")]
    Hs384,
    #[serde(rename = "HS512")]
    Hs512,
    #[serde(rename = "PS256")]
    Ps256,
    #[serde(rename = "PS384")]
    Ps384,
    #[serde(rename = "PS512")]
    Ps512,
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RS384")]
    Rs384,
    #[serde(rename = "RS512")]
    Rs512,
}
impl ::std::convert::From<&Self> for DefaultSignatureAlgorithm {
    fn from(value: &DefaultSignatureAlgorithm) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DefaultSignatureAlgorithm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EdDsa => write!(f, "EdDSA"),
            Self::Es256 => write!(f, "ES256"),
            Self::Es384 => write!(f, "ES384"),
            Self::Es512 => write!(f, "ES512"),
            Self::Hs256 => write!(f, "HS256"),
            Self::Hs384 => write!(f, "HS384"),
            Self::Hs512 => write!(f, "HS512"),
            Self::Ps256 => write!(f, "PS256"),
            Self::Ps384 => write!(f, "PS384"),
            Self::Ps512 => write!(f, "PS512"),
            Self::Rs256 => write!(f, "RS256"),
            Self::Rs384 => write!(f, "RS384"),
            Self::Rs512 => write!(f, "RS512"),
        }
    }
}
impl ::std::str::FromStr for DefaultSignatureAlgorithm {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EdDSA" => Ok(Self::EdDsa),
            "ES256" => Ok(Self::Es256),
            "ES384" => Ok(Self::Es384),
            "ES512" => Ok(Self::Es512),
            "HS256" => Ok(Self::Hs256),
            "HS384" => Ok(Self::Hs384),
            "HS512" => Ok(Self::Hs512),
            "PS256" => Ok(Self::Ps256),
            "PS384" => Ok(Self::Ps384),
            "PS512" => Ok(Self::Ps512),
            "RS256" => Ok(Self::Rs256),
            "RS384" => Ok(Self::Rs384),
            "RS512" => Ok(Self::Rs512),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DefaultSignatureAlgorithm {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DefaultSignatureAlgorithm {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DefaultSignatureAlgorithm {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///EnableSsl
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Enable SSL",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum EnableSsl {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for EnableSsl {
    fn from(value: &EnableSsl) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EnableSsl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for EnableSsl {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EnableSsl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EnableSsl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EnableSsl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///EnableStartTls
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Enable StartTLS",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum EnableStartTls {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for EnableStartTls {
    fn from(value: &EnableStartTls) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EnableStartTls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for EnableStartTls {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EnableStartTls {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EnableStartTls {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EnableStartTls {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///EnforcementMode
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
impl ::std::convert::From<&Self> for EnforcementMode {
    fn from(value: &EnforcementMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EnforcementMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Permissive => write!(f, "PERMISSIVE"),
            Self::Enforcing => write!(f, "ENFORCING"),
            Self::Disabled => write!(f, "DISABLED"),
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
///EvaluationResultRepresentation
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
///        "$ref": "#/$defs/ScopeRepresentation"
///      }
///    },
///    "policies": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PolicyResultRepresentation"
///      }
///    },
///    "resource": {
///      "$ref": "#/$defs/ResourceRepresentation"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ScopeRepresentation"
///      }
///    },
///    "status": {
///      "$ref": "#/$defs/DecisionEffect"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EvaluationResultRepresentation {
    #[serde(
        rename = "allowedScopes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub allowed_scopes: ::std::vec::Vec<ScopeRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub policies: ::std::vec::Vec<PolicyResultRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resource: ::std::option::Option<ResourceRepresentation>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<ScopeRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<DecisionEffect>,
}
impl ::std::convert::From<&EvaluationResultRepresentation>
for EvaluationResultRepresentation {
    fn from(value: &EvaluationResultRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EvaluationResultRepresentation {
    fn default() -> Self {
        Self {
            allowed_scopes: Default::default(),
            policies: Default::default(),
            resource: Default::default(),
            scopes: Default::default(),
            status: Default::default(),
        }
    }
}
///EventRepresentation
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "type": {
///      "type": "string"
///    },
///    "userId": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&EventRepresentation> for EventRepresentation {
    fn from(value: &EventRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EventRepresentation {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            details: Default::default(),
            error: Default::default(),
            ip_address: Default::default(),
            realm_id: Default::default(),
            session_id: Default::default(),
            time: Default::default(),
            type_: Default::default(),
            user_id: Default::default(),
        }
    }
}
///If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Exclude Session State From Authentication Response",
///  "description": "If this is on, the parameter 'session_state' will not be included in OpenID Connect Authentication Response. It is useful if the client uses an older OIDC / OAuth2 adapter, which does not support the 'session_state' parameter.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum ExcludeSessionStateFromAuthenticationResponse {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for ExcludeSessionStateFromAuthenticationResponse {
    fn from(value: &ExcludeSessionStateFromAuthenticationResponse) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExcludeSessionStateFromAuthenticationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for ExcludeSessionStateFromAuthenticationResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExcludeSessionStateFromAuthenticationResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExcludeSessionStateFromAuthenticationResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExcludeSessionStateFromAuthenticationResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Sets the expiration for events. Expired events are periodically deleted from the database.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Expiration",
///  "description": "Sets the expiration for events. Expired events are periodically deleted from the database.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct Expiration(::std::string::String);
impl ::std::ops::Deref for Expiration {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Expiration> for ::std::string::String {
    fn from(value: Expiration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Expiration> for Expiration {
    fn from(value: &Expiration) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Expiration {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Expiration {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Expiration {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Expiration {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Expiration {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///The expiration time of the "auth_req_id" in seconds since the authentication request was received.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Expires In",
///  "description": "The expiration time of the \"auth_req_id\" in seconds since the authentication request was received.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct ExpiresIn(::std::string::String);
impl ::std::ops::Deref for ExpiresIn {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExpiresIn> for ::std::string::String {
    fn from(value: ExpiresIn) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpiresIn> for ExpiresIn {
    fn from(value: &ExpiresIn) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ExpiresIn {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExpiresIn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpiresIn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpiresIn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpiresIn {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///FederatedIdentityRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&FederatedIdentityRepresentation>
for FederatedIdentityRepresentation {
    fn from(value: &FederatedIdentityRepresentation) -> Self {
        value.clone()
    }
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
///Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Force authentication",
///  "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum ForceAuthentication {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for ForceAuthentication {
    fn from(value: &ForceAuthentication) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ForceAuthentication {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for ForceAuthentication {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ForceAuthentication {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ForceAuthentication {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ForceAuthentication {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///GlobalRequestResult
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&GlobalRequestResult> for GlobalRequestResult {
    fn from(value: &GlobalRequestResult) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for GlobalRequestResult {
    fn default() -> Self {
        Self {
            failed_requests: Default::default(),
            success_requests: Default::default(),
        }
    }
}
///GroupRepresentation
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "subGroups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/GroupRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&GroupRepresentation> for GroupRepresentation {
    fn from(value: &GroupRepresentation) -> Self {
        value.clone()
    }
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
///If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Hide on login page",
///  "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum HideOnLoginPage {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for HideOnLoginPage {
    fn from(value: &HideOnLoginPage) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for HideOnLoginPage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for HideOnLoginPage {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HideOnLoginPage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HideOnLoginPage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HideOnLoginPage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "HTTP-POST binding for AuthnRequest",
///  "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum HttpPostBindingForAuthnRequest {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for HttpPostBindingForAuthnRequest {
    fn from(value: &HttpPostBindingForAuthnRequest) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for HttpPostBindingForAuthnRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for HttpPostBindingForAuthnRequest {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HttpPostBindingForAuthnRequest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HttpPostBindingForAuthnRequest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HttpPostBindingForAuthnRequest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "HTTP-POST binding logout",
///  "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum HttpPostBindingLogout {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for HttpPostBindingLogout {
    fn from(value: &HttpPostBindingLogout) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for HttpPostBindingLogout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for HttpPostBindingLogout {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HttpPostBindingLogout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HttpPostBindingLogout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HttpPostBindingLogout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "HTTP-POST binding response",
///  "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum HttpPostBindingResponse {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for HttpPostBindingResponse {
    fn from(value: &HttpPostBindingResponse) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for HttpPostBindingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for HttpPostBindingResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HttpPostBindingResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HttpPostBindingResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HttpPostBindingResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///IdToken
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
///      "$ref": "#/$defs/AddressClaimSet"
///    },
///    "at_hash": {
///      "type": "string"
///    },
///    "auth_time": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "website": {
///      "type": "string"
///    },
///    "zoneinfo": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&IdToken> for IdToken {
    fn from(value: &IdToken) -> Self {
        value.clone()
    }
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
///IdentityProviderMapperRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "config": {
///      "type": "object",
///      "properties": {
///        "attribute.friendly.name": {
///          "title": "Friendly name",
///          "description": "Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.",
///          "type": "string"
///        },
///        "attribute.name.format": {
///          "type": "string",
///          "enum": [
///            "ATTRIBUTE_FORMAT_BASIC",
///            "ATTRIBUTE_FORMAT_URI",
///            "ATTRIBUTE_FORMAT_UNSPECIFIED"
///          ]
///        },
///        "syncMode": {
///          "title": "Sync mode override",
///          "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.",
///          "type": "string",
///          "enum": [
///            "INHERIT",
///            "IMPORT",
///            "LEGACY",
///            "FORCE"
///          ]
///        },
///        "user.attribute": {
///          "title": "User Attribute Name",
///          "description": "Name of user attribute you want to hardcode",
///          "type": "string"
///        }
///      },
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
///      "title": "Name",
///      "description": "Name of the mapper.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdentityProviderMapperRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<IdentityProviderMapperRepresentationConfig>,
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
    ///Name of the mapper.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&IdentityProviderMapperRepresentation>
for IdentityProviderMapperRepresentation {
    fn from(value: &IdentityProviderMapperRepresentation) -> Self {
        value.clone()
    }
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
///IdentityProviderMapperRepresentationConfig
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "attribute.friendly.name": {
///      "title": "Friendly name",
///      "description": "Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.",
///      "type": "string"
///    },
///    "attribute.name.format": {
///      "type": "string",
///      "enum": [
///        "ATTRIBUTE_FORMAT_BASIC",
///        "ATTRIBUTE_FORMAT_URI",
///        "ATTRIBUTE_FORMAT_UNSPECIFIED"
///      ]
///    },
///    "syncMode": {
///      "title": "Sync mode override",
///      "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.",
///      "type": "string",
///      "enum": [
///        "INHERIT",
///        "IMPORT",
///        "LEGACY",
///        "FORCE"
///      ]
///    },
///    "user.attribute": {
///      "title": "User Attribute Name",
///      "description": "Name of user attribute you want to hardcode",
///      "type": "string"
///    }
///  },
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct IdentityProviderMapperRepresentationConfig {
    ///Friendly name of attribute to search for in assertion. You can leave this blank and specify a name instead.
    #[serde(
        rename = "attribute.friendly.name",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub attribute_friendly_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "attribute.name.format",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub attribute_name_format: ::std::option::Option<
        IdentityProviderMapperRepresentationConfigAttributeNameFormat,
    >,
    ///Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.
    #[serde(
        rename = "syncMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_mode: ::std::option::Option<SyncModeOverride>,
    ///Name of user attribute you want to hardcode
    #[serde(
        rename = "user.attribute",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_attribute: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::convert::From<&IdentityProviderMapperRepresentationConfig>
for IdentityProviderMapperRepresentationConfig {
    fn from(value: &IdentityProviderMapperRepresentationConfig) -> Self {
        value.clone()
    }
}
///IdentityProviderMapperRepresentationConfigAttributeNameFormat
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ATTRIBUTE_FORMAT_BASIC",
///    "ATTRIBUTE_FORMAT_URI",
///    "ATTRIBUTE_FORMAT_UNSPECIFIED"
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
pub enum IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    #[serde(rename = "ATTRIBUTE_FORMAT_BASIC")]
    AttributeFormatBasic,
    #[serde(rename = "ATTRIBUTE_FORMAT_URI")]
    AttributeFormatUri,
    #[serde(rename = "ATTRIBUTE_FORMAT_UNSPECIFIED")]
    AttributeFormatUnspecified,
}
impl ::std::convert::From<&Self>
for IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    fn from(
        value: &IdentityProviderMapperRepresentationConfigAttributeNameFormat,
    ) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display
for IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AttributeFormatBasic => write!(f, "ATTRIBUTE_FORMAT_BASIC"),
            Self::AttributeFormatUri => write!(f, "ATTRIBUTE_FORMAT_URI"),
            Self::AttributeFormatUnspecified => write!(f, "ATTRIBUTE_FORMAT_UNSPECIFIED"),
        }
    }
}
impl ::std::str::FromStr
for IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ATTRIBUTE_FORMAT_BASIC" => Ok(Self::AttributeFormatBasic),
            "ATTRIBUTE_FORMAT_URI" => Ok(Self::AttributeFormatUri),
            "ATTRIBUTE_FORMAT_UNSPECIFIED" => Ok(Self::AttributeFormatUnspecified),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for IdentityProviderMapperRepresentationConfigAttributeNameFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///IdentityProviderMapperTypeRepresentation
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
///        "$ref": "#/$defs/ConfigPropertyRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&IdentityProviderMapperTypeRepresentation>
for IdentityProviderMapperTypeRepresentation {
    fn from(value: &IdentityProviderMapperTypeRepresentation) -> Self {
        value.clone()
    }
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
///IdentityProviderRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "addReadTokenRoleOnCreate": {
///      "title": "Stored tokens readable",
///      "description": "Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.",
///      "type": "boolean"
///    },
///    "alias": {
///      "title": "Alias",
///      "description": "The alias uniquely identifies an identity provider and it is also used to build the redirect uri.",
///      "type": "string"
///    },
///    "authenticateByDefault": {
///      "type": "boolean"
///    },
///    "config": {
///      "type": "object",
///      "properties": {
///        "allowCreate": {
///          "title": "Allow create",
///          "description": "Allow the external identity provider to create a new identifier to represent the principal.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "allowedClockSkew": {
///          "title": "Allowed clock skew",
///          "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "attributeConsumingServiceIndex": {
///          "title": "Attribute Consuming Service Index",
///          "description": "Index of the Attribute Consuming Service profile to request during authentication.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "authnContextClassRefs": {
///          "title": "AuthnContext ClassRefs",
///          "description": "Ordered list of requested AuthnContext ClassRefs.",
///          "type": "string"
///        },
///        "authnContextComparisonType": {
///          "title": "Comparison",
///          "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".",
///          "type": "string",
///          "enum": [
///            "exact",
///            "minimum",
///            "maximum",
///            "better"
///          ]
///        },
///        "authnContextDeclRefs": {
///          "title": "AuthnContext DeclRefs",
///          "description": "Ordered list of requested AuthnContext DeclRefs.",
///          "type": "string"
///        },
///        "backchannelSupported": {
///          "title": "Backchannel logout",
///          "description": "Does the external IDP support backchannel logout?",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "encryptionAlgorithm": {
///          "title": "Encryption Algorithm",
///          "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.",
///          "type": "string"
///        },
///        "entityId": {
///          "title": "Service provider entity ID",
///          "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.",
///          "type": "string"
///        },
///        "forceAuthn": {
///          "title": "Force authentication",
///          "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "hideOnLoginPage": {
///          "title": "Hide on login page",
///          "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "idpEntityId": {
///          "title": "Identity provider entity ID",
///          "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.",
///          "type": "string"
///        },
///        "loginHint": {
///          "title": "Pass subject",
///          "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "nameIDPolicyFormat": {
///          "title": "NameID policy format",
///          "description": "Specifies the URI reference corresponding to a name identifier format.",
///          "type": "string"
///        },
///        "postBindingAuthnRequest": {
///          "title": "HTTP-POST binding for AuthnRequest",
///          "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "postBindingLogout": {
///          "title": "HTTP-POST binding logout",
///          "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "postBindingResponse": {
///          "title": "HTTP-POST binding response",
///          "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "principalAttribute": {
///          "title": "Principal attribute",
///          "description": "Name or Friendly Name of the attribute used to identify external users.",
///          "type": "string"
///        },
///        "principalType": {
///          "title": "Principal type",
///          "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.",
///          "type": "string",
///          "enum": [
///            "SUBJECT",
///            "ATTRIBUTE",
///            "FRIENDLY_ATTRIBUTE"
///          ]
///        },
///        "signSpMetadata": {
///          "title": "Sign service provider metadata",
///          "description": "Enable/disable signature of the provider SAML metadata.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "signatureAlgorithm": {
///          "title": "Signature algorithm",
///          "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.",
///          "type": "string"
///        },
///        "signingCertificate": {
///          "title": "Validating X509 certificates",
///          "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.",
///          "type": "string"
///        },
///        "singleSignOnServiceUrl": {
///          "title": "Single Sign-On service URL",
///          "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).",
///          "type": "string"
///        },
///        "syncMode": {
///          "title": "Sync mode",
///          "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.",
///          "type": "string",
///          "enum": [
///            "IMPORT",
///            "LEGACY",
///            "FORCE"
///          ]
///        },
///        "validateSignature": {
///          "title": "Validate Signatures",
///          "description": "Enable/disable signature validation of external IDP signatures.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "wantAssertionsEncrypted": {
///          "title": "Want Assertions encrypted",
///          "description": "Indicates whether this service provider expects an encrypted Assertion.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "wantAssertionsSigned": {
///          "title": "Want Assertions signed",
///          "description": "Indicates whether this service provider expects a signed Assertion.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "wantAuthnRequestsSigned": {
///          "title": "Want AuthnRequests signed",
///          "description": "Indicates whether the identity provider expects a signed AuthnRequest.",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "xmlSigKeyInfoKeyNameTransformer": {
///          "title": "SAML signature key name",
///          "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.",
///          "type": "string",
///          "enum": [
///            "NONE",
///            "KEY_ID",
///            "CERT_SUBJECT"
///          ]
///        }
///      },
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "displayName": {
///      "title": "Display name",
///      "description": "Friendly name for Identity Providers.",
///      "type": "string"
///    },
///    "enabled": {
///      "title": "Enabled",
///      "type": "boolean"
///    },
///    "firstBrokerLoginFlowAlias": {
///      "title": "First login flow override",
///      "description": "Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.",
///      "type": "string"
///    },
///    "hideOnLogin": {
///      "title": "Hide on login page",
///      "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.",
///      "type": "boolean"
///    },
///    "internalId": {
///      "type": "string"
///    },
///    "linkOnly": {
///      "title": "Account linking only",
///      "description": "If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.",
///      "type": "boolean"
///    },
///    "organizationId": {
///      "type": "string"
///    },
///    "postBrokerLoginFlowAlias": {
///      "title": "Post login flow",
///      "description": "Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to \"None\" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.",
///      "type": "string"
///    },
///    "providerId": {
///      "type": "string"
///    },
///    "storeToken": {
///      "title": "Store tokens",
///      "description": "Enable/disable if tokens must be stored after authenticating users.",
///      "type": "boolean"
///    },
///    "trustEmail": {
///      "title": "Trust Email",
///      "description": "If enabled, email provided by this provider is not verified even if verification is enabled for the realm.",
///      "type": "boolean"
///    },
///    "updateProfileFirstLogin": {
///      "type": "boolean"
///    },
///    "updateProfileFirstLoginMode": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdentityProviderRepresentation {
    ///Enable/disable if new users can read any stored tokens. This assigns the broker.read-token role.
    #[serde(
        rename = "addReadTokenRoleOnCreate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub add_read_token_role_on_create: ::std::option::Option<bool>,
    ///The alias uniquely identifies an identity provider and it is also used to build the redirect uri.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "authenticateByDefault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authenticate_by_default: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<IdentityProviderRepresentationConfig>,
    ///Friendly name for Identity Providers.
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    ///Alias of authentication flow, which is triggered after first login with this identity provider. Term 'First Login' means that no Keycloak account is currently linked to the authenticated identity provider account.
    #[serde(
        rename = "firstBrokerLoginFlowAlias",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_broker_login_flow_alias: ::std::option::Option<::std::string::String>,
    ///If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.
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
    ///If true, users cannot log in through this provider.  They can only link to this provider.  This is useful if you don't want to allow login from the provider, but want to integrate with a provider.
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
    ///Alias of authentication flow, which is triggered after each login with this identity provider. Useful if you want additional verification of each user authenticated with this identity provider (for example OTP). Leave this to "None" if you need no any additional authenticators to be triggered after login with this identity provider. Also note that authenticator implementations must assume that user is already set in ClientSession as identity provider already set it.
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
    ///Enable/disable if tokens must be stored after authenticating users.
    #[serde(
        rename = "storeToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_token: ::std::option::Option<bool>,
    ///If enabled, email provided by this provider is not verified even if verification is enabled for the realm.
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
impl ::std::convert::From<&IdentityProviderRepresentation>
for IdentityProviderRepresentation {
    fn from(value: &IdentityProviderRepresentation) -> Self {
        value.clone()
    }
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
///IdentityProviderRepresentationConfig
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "allowCreate": {
///      "title": "Allow create",
///      "description": "Allow the external identity provider to create a new identifier to represent the principal.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "allowedClockSkew": {
///      "title": "Allowed clock skew",
///      "description": "Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "attributeConsumingServiceIndex": {
///      "title": "Attribute Consuming Service Index",
///      "description": "Index of the Attribute Consuming Service profile to request during authentication.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "authnContextClassRefs": {
///      "title": "AuthnContext ClassRefs",
///      "description": "Ordered list of requested AuthnContext ClassRefs.",
///      "type": "string"
///    },
///    "authnContextComparisonType": {
///      "title": "Comparison",
///      "description": "Specifies the comparison method used to evaluate the requested context classes or statements. The default is \"Exact\".",
///      "type": "string",
///      "enum": [
///        "exact",
///        "minimum",
///        "maximum",
///        "better"
///      ]
///    },
///    "authnContextDeclRefs": {
///      "title": "AuthnContext DeclRefs",
///      "description": "Ordered list of requested AuthnContext DeclRefs.",
///      "type": "string"
///    },
///    "backchannelSupported": {
///      "title": "Backchannel logout",
///      "description": "Does the external IDP support backchannel logout?",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "encryptionAlgorithm": {
///      "title": "Encryption Algorithm",
///      "description": "Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.",
///      "type": "string"
///    },
///    "entityId": {
///      "title": "Service provider entity ID",
///      "description": "The Entity ID that will be used to uniquely identify this SAML Service Provider.",
///      "type": "string"
///    },
///    "forceAuthn": {
///      "title": "Force authentication",
///      "description": "Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "hideOnLoginPage": {
///      "title": "Hide on login page",
///      "description": "If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "idpEntityId": {
///      "title": "Identity provider entity ID",
///      "description": "The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.",
///      "type": "string"
///    },
///    "loginHint": {
///      "title": "Pass subject",
///      "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "nameIDPolicyFormat": {
///      "title": "NameID policy format",
///      "description": "Specifies the URI reference corresponding to a name identifier format.",
///      "type": "string"
///    },
///    "postBindingAuthnRequest": {
///      "title": "HTTP-POST binding for AuthnRequest",
///      "description": "Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "postBindingLogout": {
///      "title": "HTTP-POST binding logout",
///      "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "postBindingResponse": {
///      "title": "HTTP-POST binding response",
///      "description": "Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "principalAttribute": {
///      "title": "Principal attribute",
///      "description": "Name or Friendly Name of the attribute used to identify external users.",
///      "type": "string"
///    },
///    "principalType": {
///      "title": "Principal type",
///      "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.",
///      "type": "string",
///      "enum": [
///        "SUBJECT",
///        "ATTRIBUTE",
///        "FRIENDLY_ATTRIBUTE"
///      ]
///    },
///    "signSpMetadata": {
///      "title": "Sign service provider metadata",
///      "description": "Enable/disable signature of the provider SAML metadata.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "signatureAlgorithm": {
///      "title": "Signature algorithm",
///      "description": "The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.",
///      "type": "string"
///    },
///    "signingCertificate": {
///      "title": "Validating X509 certificates",
///      "description": "The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.",
///      "type": "string"
///    },
///    "singleSignOnServiceUrl": {
///      "title": "Single Sign-On service URL",
///      "description": "The Url that must be used to send authentication requests (SAML AuthnRequest).",
///      "type": "string"
///    },
///    "syncMode": {
///      "title": "Sync mode",
///      "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.",
///      "type": "string",
///      "enum": [
///        "IMPORT",
///        "LEGACY",
///        "FORCE"
///      ]
///    },
///    "validateSignature": {
///      "title": "Validate Signatures",
///      "description": "Enable/disable signature validation of external IDP signatures.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "wantAssertionsEncrypted": {
///      "title": "Want Assertions encrypted",
///      "description": "Indicates whether this service provider expects an encrypted Assertion.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "wantAssertionsSigned": {
///      "title": "Want Assertions signed",
///      "description": "Indicates whether this service provider expects a signed Assertion.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "wantAuthnRequestsSigned": {
///      "title": "Want AuthnRequests signed",
///      "description": "Indicates whether the identity provider expects a signed AuthnRequest.",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "xmlSigKeyInfoKeyNameTransformer": {
///      "title": "SAML signature key name",
///      "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.",
///      "type": "string",
///      "enum": [
///        "NONE",
///        "KEY_ID",
///        "CERT_SUBJECT"
///      ]
///    }
///  },
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct IdentityProviderRepresentationConfig {
    ///Allow the external identity provider to create a new identifier to represent the principal.
    #[serde(
        rename = "allowCreate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_create: ::std::option::Option<AllowCreate>,
    ///Clock skew in seconds that is tolerated when validating identity provider tokens. Default value is zero.
    #[serde(
        rename = "allowedClockSkew",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_clock_skew: ::std::option::Option<AllowedClockSkew>,
    ///Index of the Attribute Consuming Service profile to request during authentication.
    #[serde(
        rename = "attributeConsumingServiceIndex",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub attribute_consuming_service_index: ::std::option::Option<
        AttributeConsumingServiceIndex,
    >,
    ///Ordered list of requested AuthnContext ClassRefs.
    #[serde(
        rename = "authnContextClassRefs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authn_context_class_refs: ::std::option::Option<::std::string::String>,
    ///Specifies the comparison method used to evaluate the requested context classes or statements. The default is "Exact".
    #[serde(
        rename = "authnContextComparisonType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authn_context_comparison_type: ::std::option::Option<Comparison>,
    ///Ordered list of requested AuthnContext DeclRefs.
    #[serde(
        rename = "authnContextDeclRefs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authn_context_decl_refs: ::std::option::Option<::std::string::String>,
    ///Does the external IDP support backchannel logout?
    #[serde(
        rename = "backchannelSupported",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub backchannel_supported: ::std::option::Option<BackchannelLogout>,
    ///Encryption algorithm, which is used by SAML IDP for encryption of SAML documents, assertions or IDs. The corresponding decryption key for decrypt SAML document parts will be chosen based on this configured algorithm and should be available in realm keys for the encryption (ENC) usage. If algorithm is not configured, then any supported algorithm is allowed and decryption key will be chosen based on the algorithm configured in SAML document itself.
    #[serde(
        rename = "encryptionAlgorithm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub encryption_algorithm: ::std::option::Option<::std::string::String>,
    ///The Entity ID that will be used to uniquely identify this SAML Service Provider.
    #[serde(
        rename = "entityId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub entity_id: ::std::option::Option<::std::string::String>,
    ///Indicates whether the identity provider must authenticate the presenter directly rather than rely on a previous security context.
    #[serde(
        rename = "forceAuthn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_authn: ::std::option::Option<ForceAuthentication>,
    ///If hidden, login with this provider is possible only if requested explicitly, for example using the 'kc_idp_hint' parameter.
    #[serde(
        rename = "hideOnLoginPage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hide_on_login_page: ::std::option::Option<HideOnLoginPage>,
    ///The Entity ID used to validate the Issuer for received SAML assertions. If empty, no Issuer validation is performed.
    #[serde(
        rename = "idpEntityId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub idp_entity_id: ::std::option::Option<::std::string::String>,
    ///During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.
    #[serde(
        rename = "loginHint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_hint: ::std::option::Option<PassSubject>,
    ///Specifies the URI reference corresponding to a name identifier format.
    #[serde(
        rename = "nameIDPolicyFormat",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_id_policy_format: ::std::option::Option<::std::string::String>,
    ///Indicates whether the AuthnRequest must be sent using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.
    #[serde(
        rename = "postBindingAuthnRequest",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub post_binding_authn_request: ::std::option::Option<
        HttpPostBindingForAuthnRequest,
    >,
    ///Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.
    #[serde(
        rename = "postBindingLogout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub post_binding_logout: ::std::option::Option<HttpPostBindingLogout>,
    ///Indicates whether to respond to requests using HTTP-POST binding. If false, HTTP-REDIRECT binding will be used.
    #[serde(
        rename = "postBindingResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub post_binding_response: ::std::option::Option<HttpPostBindingResponse>,
    ///Name or Friendly Name of the attribute used to identify external users.
    #[serde(
        rename = "principalAttribute",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub principal_attribute: ::std::option::Option<::std::string::String>,
    ///Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.
    #[serde(
        rename = "principalType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub principal_type: ::std::option::Option<PrincipalType>,
    ///Enable/disable signature of the provider SAML metadata.
    #[serde(
        rename = "signSpMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sign_sp_metadata: ::std::option::Option<SignServiceProviderMetadata>,
    ///The signature algorithm to use to sign documents. Note that 'SHA1' based algorithms are deprecated and can be removed in the future. It is recommended to stick to some more secure algorithm instead of '*_SHA1'.
    #[serde(
        rename = "signatureAlgorithm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub signature_algorithm: ::std::option::Option<::std::string::String>,
    ///The public certificates Keycloak uses to validate the signatures of SAML requests and responses from the external IDP when Use metadata descriptor URL is OFF. Multiple certificates can be entered separated by comma (,). The certificates can be re-imported from the Metadata descriptor URL clicking the Import Keys action in the identity provider page. The action downloads the current certificates in the metadata endpoint and assigns them to the config in this same option. You need to click Save to definitely store the re-imported certificates.
    #[serde(
        rename = "signingCertificate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub signing_certificate: ::std::option::Option<::std::string::String>,
    ///The Url that must be used to send authentication requests (SAML AuthnRequest).
    #[serde(
        rename = "singleSignOnServiceUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub single_sign_on_service_url: ::std::option::Option<::std::string::String>,
    ///Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.
    #[serde(
        rename = "syncMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_mode: ::std::option::Option<SyncMode>,
    ///Enable/disable signature validation of external IDP signatures.
    #[serde(
        rename = "validateSignature",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub validate_signature: ::std::option::Option<ValidateSignatures>,
    ///Indicates whether this service provider expects an encrypted Assertion.
    #[serde(
        rename = "wantAssertionsEncrypted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub want_assertions_encrypted: ::std::option::Option<WantAssertionsEncrypted>,
    ///Indicates whether this service provider expects a signed Assertion.
    #[serde(
        rename = "wantAssertionsSigned",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub want_assertions_signed: ::std::option::Option<WantAssertionsSigned>,
    ///Indicates whether the identity provider expects a signed AuthnRequest.
    #[serde(
        rename = "wantAuthnRequestsSigned",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub want_authn_requests_signed: ::std::option::Option<WantAuthnRequestsSigned>,
    ///Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.
    #[serde(
        rename = "xmlSigKeyInfoKeyNameTransformer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub xml_sig_key_info_key_name_transformer: ::std::option::Option<
        SamlSignatureKeyName,
    >,
    #[serde(flatten)]
    pub extra: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::convert::From<&IdentityProviderRepresentationConfig>
for IdentityProviderRepresentationConfig {
    fn from(value: &IdentityProviderRepresentationConfig) -> Self {
        value.clone()
    }
}
///InstallationAdapterConfig
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "credentials": {
///      "type": "object",
///      "additionalProperties": {}
///    },
///    "policy-enforcer": {
///      "$ref": "#/$defs/PolicyEnforcerConfig"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&InstallationAdapterConfig> for InstallationAdapterConfig {
    fn from(value: &InstallationAdapterConfig) -> Self {
        value.clone()
    }
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
///The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Interval",
///  "description": "The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.",
///  "type": "string",
///  "pattern": "^[0-9]*$"
///}
/// ```
/// </details>
#[derive(
    ::schemars::JsonSchema,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct Interval(::std::string::String);
impl ::std::ops::Deref for Interval {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Interval> for ::std::string::String {
    fn from(value: Interval) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Interval> for Interval {
    fn from(value: &Interval) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Interval {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]*$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Interval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Interval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Interval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///KeyMetadataRepresentation
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///      "$ref": "#/$defs/KeyUse"
///    },
///    "validTo": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&KeyMetadataRepresentation> for KeyMetadataRepresentation {
    fn from(value: &KeyMetadataRepresentation) -> Self {
        value.clone()
    }
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
///KeyStoreConfig
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&KeyStoreConfig> for KeyStoreConfig {
    fn from(value: &KeyStoreConfig) -> Self {
        value.clone()
    }
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
///KeyUse
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
impl ::std::convert::From<&Self> for KeyUse {
    fn from(value: &KeyUse) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for KeyUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sig => write!(f, "SIG"),
            Self::Enc => write!(f, "ENC"),
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
///KeysMetadataRepresentation
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
///        "$ref": "#/$defs/KeyMetadataRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&KeysMetadataRepresentation> for KeysMetadataRepresentation {
    fn from(value: &KeysMetadataRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for KeysMetadataRepresentation {
    fn default() -> Self {
        Self {
            active: Default::default(),
            keys: Default::default(),
        }
    }
}
///Logic
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
impl ::std::convert::From<&Self> for Logic {
    fn from(value: &Logic) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Logic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Positive => write!(f, "POSITIVE"),
            Self::Negative => write!(f, "NEGATIVE"),
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
///ManagementPermissionReference
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ManagementPermissionReference>
for ManagementPermissionReference {
    fn from(value: &ManagementPermissionReference) -> Self {
        value.clone()
    }
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
///MappingsRepresentation
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
///        "$ref": "#/$defs/ClientMappingsRepresentation"
///      }
///    },
///    "realmMappings": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/RoleRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&MappingsRepresentation> for MappingsRepresentation {
    fn from(value: &MappingsRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MappingsRepresentation {
    fn default() -> Self {
        Self {
            client_mappings: Default::default(),
            realm_mappings: Default::default(),
        }
    }
}
///MemberRepresentation
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
///        "$ref": "#/$defs/UserConsentRepresentation"
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "credentials": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/CredentialRepresentation"
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
///        "$ref": "#/$defs/FederatedIdentityRepresentation"
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
///      "$ref": "#/$defs/MembershipType"
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SocialLinkRepresentation"
///      }
///    },
///    "totp": {
///      "type": "boolean"
///    },
///    "userProfileMetadata": {
///      "$ref": "#/$defs/UserProfileMetadata"
///    },
///    "username": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&MemberRepresentation> for MemberRepresentation {
    fn from(value: &MemberRepresentation) -> Self {
        value.clone()
    }
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
///MembershipType
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
impl ::std::convert::From<&Self> for MembershipType {
    fn from(value: &MembershipType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MembershipType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unmanaged => write!(f, "UNMANAGED"),
            Self::Managed => write!(f, "MANAGED"),
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
///MethodConfig
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
///      "$ref": "#/$defs/ScopeEnforcementMode"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&MethodConfig> for MethodConfig {
    fn from(value: &MethodConfig) -> Self {
        value.clone()
    }
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
///MultivaluedHashMapStringComponentExportRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": {
///    "type": "array",
///    "items": {
///      "$ref": "#/$defs/ComponentExportRepresentation"
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
impl ::std::convert::From<&MultivaluedHashMapStringComponentExportRepresentation>
for MultivaluedHashMapStringComponentExportRepresentation {
    fn from(value: &MultivaluedHashMapStringComponentExportRepresentation) -> Self {
        value.clone()
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
///MultivaluedHashMapStringString
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
impl ::std::convert::From<&MultivaluedHashMapStringString>
for MultivaluedHashMapStringString {
    fn from(value: &MultivaluedHashMapStringString) -> Self {
        value.clone()
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
///This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "OAuth 2.0 Mutual TLS Certificate Bound Access Tokens Enabled",
///  "description": "This enables support for OAuth 2.0 Mutual TLS Certificate Bound Access Tokens, which means that keycloak bind an access token and a refresh token with a X.509 certificate of a token requesting client exchanged in mutual TLS between keycloak's Token Endpoint and this client. These tokens can be treated as Holder-of-Key tokens instead of bearer tokens.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self>
for OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    fn from(value: &OAuth20MutualTlsCertificateBoundAccessTokensEnabled) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OAuth20MutualTlsCertificateBoundAccessTokensEnabled {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///OAuthClientRepresentation
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
///      "$ref": "#/$defs/ResourceServerRepresentation"
///    },
///    "baseUrl": {
///      "type": "string"
///    },
///    "bearerOnly": {
///      "type": "boolean"
///    },
///    "claims": {
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/ClaimRepresentation"
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///        "$ref": "#/$defs/ProtocolMapperRepresentation"
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
///        "format": "int32",
///        "maximum": 2147483647.0,
///        "minimum": -2147483648.0
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
///      "type": "boolean"
///    },
///    "useTemplateMappers": {
///      "type": "boolean"
///    },
///    "useTemplateScope": {
///      "type": "boolean"
///    },
///    "webOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&OAuthClientRepresentation> for OAuthClientRepresentation {
    fn from(value: &OAuthClientRepresentation) -> Self {
        value.clone()
    }
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
///OrganizationDomainRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrganizationDomainRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub verified: ::std::option::Option<bool>,
}
impl ::std::convert::From<&OrganizationDomainRepresentation>
for OrganizationDomainRepresentation {
    fn from(value: &OrganizationDomainRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for OrganizationDomainRepresentation {
    fn default() -> Self {
        Self {
            name: Default::default(),
            verified: Default::default(),
        }
    }
}
///OrganizationRepresentation
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
///        "$ref": "#/$defs/OrganizationDomainRepresentation"
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
///        "$ref": "#/$defs/IdentityProviderRepresentation"
///      }
///    },
///    "members": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/MemberRepresentation"
///      }
///    },
///    "name": {
///      "type": "string"
///    },
///    "redirectUrl": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&OrganizationRepresentation> for OrganizationRepresentation {
    fn from(value: &OrganizationRepresentation) -> Self {
        value.clone()
    }
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
///totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "OTP type",
///  "description": "totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.",
///  "type": "string",
///  "enum": [
///    "totp",
///    "hotp"
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
pub enum OtpType {
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "hotp")]
    Hotp,
}
impl ::std::convert::From<&Self> for OtpType {
    fn from(value: &OtpType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OtpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Totp => write!(f, "totp"),
            Self::Hotp => write!(f, "hotp"),
        }
    }
}
impl ::std::str::FromStr for OtpType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "totp" => Ok(Self::Totp),
            "hotp" => Ok(Self::Hotp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OtpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OtpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OtpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Pass subject",
///  "description": "During login phase, forward an optional login_hint query parameter to SAML AuthnRequest's Subject.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum PassSubject {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for PassSubject {
    fn from(value: &PassSubject) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PassSubject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for PassSubject {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PassSubject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PassSubject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PassSubject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///PathCacheConfig
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "lifespan": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "max-entries": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PathCacheConfig> for PathCacheConfig {
    fn from(value: &PathCacheConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PathCacheConfig {
    fn default() -> Self {
        Self {
            lifespan: Default::default(),
            max_entries: Default::default(),
        }
    }
}
///PathConfig
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
///      "$ref": "#/$defs/EnforcementMode"
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
///        "$ref": "#/$defs/MethodConfig"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PathConfig> for PathConfig {
    fn from(value: &PathConfig) -> Self {
        value.clone()
    }
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
///Permission
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&Permission> for Permission {
    fn from(value: &Permission) -> Self {
        value.clone()
    }
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
///PolicyEnforcementMode
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
impl ::std::convert::From<&Self> for PolicyEnforcementMode {
    fn from(value: &PolicyEnforcementMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PolicyEnforcementMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enforcing => write!(f, "ENFORCING"),
            Self::Permissive => write!(f, "PERMISSIVE"),
            Self::Disabled => write!(f, "DISABLED"),
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
///PolicyEnforcerConfig
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
///      "$ref": "#/$defs/EnforcementMode"
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
///      "$ref": "#/$defs/PathCacheConfig"
///    },
///    "paths": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/PathConfig"
///      }
///    },
///    "realm": {
///      "type": "string"
///    },
///    "resource": {
///      "type": "string"
///    },
///    "user-managed-access": {
///      "$ref": "#/$defs/UserManagedAccessConfig"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PolicyEnforcerConfig> for PolicyEnforcerConfig {
    fn from(value: &PolicyEnforcerConfig) -> Self {
        value.clone()
    }
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
///PolicyEvaluationRequest
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
///    "resources": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ResourceRepresentation"
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PolicyEvaluationRequest> for PolicyEvaluationRequest {
    fn from(value: &PolicyEvaluationRequest) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PolicyEvaluationRequest {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            context: Default::default(),
            entitlements: Default::default(),
            resources: Default::default(),
            role_ids: Default::default(),
            user_id: Default::default(),
        }
    }
}
///PolicyEvaluationResponse
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
///        "$ref": "#/$defs/EvaluationResultRepresentation"
///      }
///    },
///    "rpt": {
///      "$ref": "#/$defs/AccessToken"
///    },
///    "status": {
///      "$ref": "#/$defs/DecisionEffect"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PolicyEvaluationResponse> for PolicyEvaluationResponse {
    fn from(value: &PolicyEvaluationResponse) -> Self {
        value.clone()
    }
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
///PolicyProviderRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PolicyProviderRepresentation>
for PolicyProviderRepresentation {
    fn from(value: &PolicyProviderRepresentation) -> Self {
        value.clone()
    }
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
///PolicyRepresentation
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
///      "$ref": "#/$defs/DecisionStrategy"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "logic": {
///      "$ref": "#/$defs/Logic"
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
///        "$ref": "#/$defs/ResourceRepresentation"
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
///        "$ref": "#/$defs/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PolicyRepresentation> for PolicyRepresentation {
    fn from(value: &PolicyRepresentation) -> Self {
        value.clone()
    }
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
///PolicyResultRepresentation
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
///        "$ref": "#/$defs/PolicyResultRepresentation"
///      }
///    },
///    "policy": {
///      "$ref": "#/$defs/PolicyRepresentation"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "status": {
///      "$ref": "#/$defs/DecisionEffect"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PolicyResultRepresentation {
    #[serde(
        rename = "associatedPolicies",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub associated_policies: ::std::vec::Vec<PolicyResultRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<PolicyRepresentation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<DecisionEffect>,
}
impl ::std::convert::From<&PolicyResultRepresentation> for PolicyResultRepresentation {
    fn from(value: &PolicyResultRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PolicyResultRepresentation {
    fn default() -> Self {
        Self {
            associated_policies: Default::default(),
            policy: Default::default(),
            scopes: Default::default(),
            status: Default::default(),
        }
    }
}
///Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Principal type",
///  "description": "Way to identify and track external users from the assertion. Default is using Subject NameID, alternatively you can set up identifying attribute.",
///  "type": "string",
///  "enum": [
///    "SUBJECT",
///    "ATTRIBUTE",
///    "FRIENDLY_ATTRIBUTE"
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
pub enum PrincipalType {
    #[serde(rename = "SUBJECT")]
    Subject,
    #[serde(rename = "ATTRIBUTE")]
    Attribute,
    #[serde(rename = "FRIENDLY_ATTRIBUTE")]
    FriendlyAttribute,
}
impl ::std::convert::From<&Self> for PrincipalType {
    fn from(value: &PrincipalType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PrincipalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Subject => write!(f, "SUBJECT"),
            Self::Attribute => write!(f, "ATTRIBUTE"),
            Self::FriendlyAttribute => write!(f, "FRIENDLY_ATTRIBUTE"),
        }
    }
}
impl ::std::str::FromStr for PrincipalType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SUBJECT" => Ok(Self::Subject),
            "ATTRIBUTE" => Ok(Self::Attribute),
            "FRIENDLY_ATTRIBUTE" => Ok(Self::FriendlyAttribute),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PrincipalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PrincipalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PrincipalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///PropertyConfig
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PropertyConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub applicable: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&PropertyConfig> for PropertyConfig {
    fn from(value: &PropertyConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PropertyConfig {
    fn default() -> Self {
        Self {
            applicable: Default::default(),
            value: Default::default(),
        }
    }
}
///ProtocolMapperEvaluationRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ProtocolMapperEvaluationRepresentation>
for ProtocolMapperEvaluationRepresentation {
    fn from(value: &ProtocolMapperEvaluationRepresentation) -> Self {
        value.clone()
    }
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
///ProtocolMapperRepresentation
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
///      "type": "boolean"
///    },
///    "consentText": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "protocol": {
///      "type": "string",
///      "enum": [
///        "openid-connect",
///        "saml"
///      ]
///    },
///    "protocolMapper": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
    pub protocol: ::std::option::Option<ProtocolMapperRepresentationProtocol>,
    #[serde(
        rename = "protocolMapper",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub protocol_mapper: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ProtocolMapperRepresentation>
for ProtocolMapperRepresentation {
    fn from(value: &ProtocolMapperRepresentation) -> Self {
        value.clone()
    }
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
///ProtocolMapperRepresentationProtocol
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "openid-connect",
///    "saml"
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
pub enum ProtocolMapperRepresentationProtocol {
    #[serde(rename = "openid-connect")]
    OpenidConnect,
    #[serde(rename = "saml")]
    Saml,
}
impl ::std::convert::From<&Self> for ProtocolMapperRepresentationProtocol {
    fn from(value: &ProtocolMapperRepresentationProtocol) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ProtocolMapperRepresentationProtocol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OpenidConnect => write!(f, "openid-connect"),
            Self::Saml => write!(f, "saml"),
        }
    }
}
impl ::std::str::FromStr for ProtocolMapperRepresentationProtocol {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "openid-connect" => Ok(Self::OpenidConnect),
            "saml" => Ok(Self::Saml),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProtocolMapperRepresentationProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ProtocolMapperRepresentationProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProtocolMapperRepresentationProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///PublishedRealmRepresentation
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&PublishedRealmRepresentation>
for PublishedRealmRepresentation {
    fn from(value: &PublishedRealmRepresentation) -> Self {
        value.clone()
    }
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
///Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Pushed authorization request required",
///  "description": "Boolean parameter indicating whether the authorization server accepts authorization request data only via the pushed authorization request method.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum PushedAuthorizationRequestRequired {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for PushedAuthorizationRequestRequired {
    fn from(value: &PushedAuthorizationRequestRequired) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PushedAuthorizationRequestRequired {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for PushedAuthorizationRequestRequired {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PushedAuthorizationRequestRequired {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PushedAuthorizationRequestRequired {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PushedAuthorizationRequestRequired {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///RealmEventsConfigRepresentation
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "eventsListeners": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&RealmEventsConfigRepresentation>
for RealmEventsConfigRepresentation {
    fn from(value: &RealmEventsConfigRepresentation) -> Self {
        value.clone()
    }
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
///RealmRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "accessCodeLifespan": {
///      "title": "Client Login Timeout",
///      "description": "Max time a client has to finish the access token protocol. This should normally be 1 minute.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "accessCodeLifespanLogin": {
///      "title": "Login timeout",
///      "description": "Max time a user has to complete a login. This is recommended to be relatively long, such as 30 minutes or more.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "accessCodeLifespanUserAction": {
///      "title": "Login action timeout",
///      "description": "Max time a user has to complete login related actions like update password or configure totp. This is recommended to be relatively long, such as 5 minutes or more.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "accessTokenLifespan": {
///      "title": "Access Token Lifespan",
///      "description": "Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "accessTokenLifespanForImplicitFlow": {
///      "title": "Access Token Lifespan For Implicit Flow",
///      "description": "Max time before an access token issued during OpenID Connect Implicit Flow is expired. This value is recommended to be shorter than the SSO timeout. There is no possibility to refresh token during implicit flow, that's why there is a separate timeout different to 'Access Token Lifespan'.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "accountTheme": {
///      "title": "Account theme",
///      "description": "Select theme for login, OTP, grant, registration and forgot password pages.",
///      "type": "string"
///    },
///    "actionTokenGeneratedByAdminLifespan": {
///      "title": "Default Admin-Initiated Action Lifespan",
///      "description": "Maximum time before an action permit sent to a user by administrator is expired. This value is recommended to be long to allow administrators to send e-mails for users that are currently offline. The default timeout can be overridden immediately before issuing the token.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "actionTokenGeneratedByUserLifespan": {
///      "title": "User-Initiated Action Lifespan",
///      "description": "Maximum time before an action permit sent by a user (such as a forgot password e-mail) is expired. This value is recommended to be short because it's expected that the user would react to self-created action quickly.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "adminEventsDetailsEnabled": {
///      "title": "Include representation",
///      "description": "Include JSON representation for create and update requests.",
///      "type": "boolean"
///    },
///    "adminEventsEnabled": {
///      "title": "Save events",
///      "description": "If enabled, admin events are saved to the database, which makes events available to the Admin UI.",
///      "type": "boolean"
///    },
///    "adminPermissionsClient": {
///      "$ref": "#/$defs/ClientRepresentation"
///    },
///    "adminPermissionsEnabled": {
///      "type": "boolean"
///    },
///    "adminTheme": {
///      "title": "Admin theme",
///      "type": "string"
///    },
///    "applicationScopeMappings": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/$defs/ScopeMappingRepresentation"
///        }
///      }
///    },
///    "applications": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ApplicationRepresentation"
///      }
///    },
///    "attributes": {
///      "type": "object",
///      "properties": {
///        "adminEventsExpiration": {
///          "title": "Expiration",
///          "description": "Sets the expiration for events. Expired events are periodically deleted from the database.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "cibaAuthRequestedUserHint": {
///          "title": "Authentication Requested User Hint",
///          "description": "The way of identifying the end-user for whom authentication is being requested. Currently only \"login_hint\" is supported.",
///          "type": "string",
///          "enum": [
///            "login_hint"
///          ]
///        },
///        "cibaBackchannelTokenDeliveryMode": {
///          "title": "Backchannel Token Delivery Mode",
///          "description": "Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.",
///          "type": "string",
///          "enum": [
///            "ping",
///            "poll"
///          ]
///        },
///        "cibaExpiresIn": {
///          "title": "Expires In",
///          "description": "The expiration time of the \"auth_req_id\" in seconds since the authentication request was received.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "cibaInterval": {
///          "title": "Interval",
///          "description": "The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.",
///          "type": "string",
///          "pattern": "^[0-9]*$"
///        },
///        "frontendUrl": {
///          "title": "Frontend URL",
///          "description": "Set the frontend URL for the realm. Use in combination with the default hostname provider to override the base URL for frontend requests for a specific realm.",
///          "type": "string"
///        }
///      },
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "authenticationFlows": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AuthenticationFlowRepresentation"
///      }
///    },
///    "authenticatorConfig": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/AuthenticatorConfigRepresentation"
///      }
///    },
///    "browserFlow": {
///      "type": "string"
///    },
///    "browserSecurityHeaders": {
///      "type": "object",
///      "properties": {
///        "contentSecurityPolicy": {
///          "title": "Content-Security-Policy",
///          "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>",
///          "type": "string"
///        },
///        "contentSecurityPolicyReportOnly": {
///          "title": "Content-Security-Policy-Report-Only",
///          "description": "For testing Content Security Policies <1>Learn more</1>",
///          "type": "string"
///        },
///        "strictTransportSecurity": {
///          "title": "HTTP Strict Transport Security (HSTS)",
///          "description": "The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>",
///          "type": "string"
///        },
///        "xContentTypeOptions": {
///          "title": "X-Content-Type-Options",
///          "description": "The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>",
///          "type": "string"
///        },
///        "xFrameOptions": {
///          "title": "X-Frame-Options",
///          "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>",
///          "type": "string"
///        },
///        "xRobotsTag": {
///          "title": "X-Robots-Tag",
///          "description": "Prevent pages from appearing in search engines. <1>Learn more</1>",
///          "type": "string"
///        },
///        "xXSSProtection": {
///          "title": "X-XSS-Protection",
///          "description": "This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>",
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "bruteForceDetection": {
///      "title": "Brute force detection"
///    },
///    "bruteForceProtected": {
///      "type": "boolean"
///    },
///    "bruteForceStrategy": {
///      "title": "Strategy to increase wait time",
///      "description": "Multiple means wait time will be increased only when number of failures are multiples of '{{failureFactor}}'. Linear means each new failure starting at '{{failureFactor}}' will increase wait time.",
///      "$ref": "#/$defs/BruteForceStrategy"
///    },
///    "certificate": {
///      "type": "string"
///    },
///    "clientAuthenticationFlow": {
///      "type": "string"
///    },
///    "clientOfflineSessionIdleTimeout": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "clientOfflineSessionMaxLifespan": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "clientPolicies": {
///      "$ref": "#/$defs/ClientPoliciesRepresentation"
///    },
///    "clientProfiles": {
///      "$ref": "#/$defs/ClientProfilesRepresentation"
///    },
///    "clientScopeMappings": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/$defs/ScopeMappingRepresentation"
///        }
///      }
///    },
///    "clientScopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClientScopeRepresentation"
///      }
///    },
///    "clientSessionIdleTimeout": {
///      "title": "Client Session Idle",
///      "description": "Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "clientSessionMaxLifespan": {
///      "title": "Client Session Max",
///      "description": "Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "clientTemplates": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClientTemplateRepresentation"
///      }
///    },
///    "clients": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ClientRepresentation"
///      }
///    },
///    "codeSecret": {
///      "type": "string"
///    },
///    "components": {
///      "$ref": "#/$defs/MultivaluedHashMapStringComponentExportRepresentation"
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
///      "title": "Default locale",
///      "type": "string"
///    },
///    "defaultOptionalClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultRole": {
///      "$ref": "#/$defs/RoleRepresentation"
///    },
///    "defaultRoles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "defaultSignatureAlgorithm": {
///      "title": "Default Signature Algorithm",
///      "description": "Default algorithm used to sign tokens for the realm",
///      "type": "string",
///      "enum": [
///        "EdDSA",
///        "ES256",
///        "ES384",
///        "ES512",
///        "HS256",
///        "HS384",
///        "HS512",
///        "PS256",
///        "PS384",
///        "PS512",
///        "RS256",
///        "RS384",
///        "RS512"
///      ]
///    },
///    "directGrantFlow": {
///      "type": "string"
///    },
///    "displayName": {
///      "title": "Display name",
///      "type": "string"
///    },
///    "displayNameHtml": {
///      "title": "HTML Display name",
///      "type": "string"
///    },
///    "dockerAuthenticationFlow": {
///      "type": "string"
///    },
///    "duplicateEmailsAllowed": {
///      "title": "Duplicate emails",
///      "description": "Allow multiple users to have the same email address. Changing this setting will also clear the user's cache. It is recommended to manually update email constraints of existing users in the database after switching off support for duplicate email addresses.",
///      "type": "boolean"
///    },
///    "editUsernameAllowed": {
///      "title": "Edit username",
///      "description": "If enabled, the username field is editable, readonly otherwise.",
///      "type": "boolean"
///    },
///    "emailTheme": {
///      "title": "Email theme",
///      "description": "Select a theme for emails that are sent by the server.",
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
///      "title": "Save events",
///      "description": "If enabled, user events are saved to the database, which makes events available to the admin and account management UIs.",
///      "type": "boolean"
///    },
///    "eventsExpiration": {
///      "title": "Expiration",
///      "description": "Sets the expiration for events. Expired events are periodically deleted from the database.",
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "eventsListeners": {
///      "title": "Event listeners",
///      "description": "Configure what listeners receive events for the realm.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "failureFactor": {
///      "title": "Max login failures",
///      "description": "Max login failures",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "federatedUsers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UserRepresentation"
///      }
///    },
///    "firstBrokerLoginFlow": {
///      "type": "string"
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/GroupRepresentation"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "identityProviderMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/IdentityProviderMapperRepresentation"
///      }
///    },
///    "identityProviders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/IdentityProviderRepresentation"
///      }
///    },
///    "internationalizationEnabled": {
///      "title": "Internationalization",
///      "description": "If enabled, you can choose which locales you support for this realm and which locale is the default.",
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
///      "title": "Login theme",
///      "description": "Select theme for login, OTP, grant, registration and forgot password pages.",
///      "type": "string"
///    },
///    "loginWithEmailAllowed": {
///      "title": "Login with email",
///      "description": "Allow users to log in with their email address.",
///      "type": "boolean"
///    },
///    "maxDeltaTimeSeconds": {
///      "title": "Failure reset time",
///      "description": "When will failure count be reset?",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "maxFailureWaitSeconds": {
///      "title": "Max wait",
///      "description": "Max time a user will be locked out.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "maxTemporaryLockouts": {
///      "title": "Maximum temporary lockouts",
///      "description": "The number of temporary lockouts permitted before the user is permanently locked out.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "minimumQuickLoginWaitSeconds": {
///      "title": "Minimum quick login wait",
///      "description": "How long to wait after a quick login failure.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "notBefore": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "oAuth2DeviceCodeLifespan": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "oAuth2DevicePollingInterval": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "oauth2DeviceCodeLifespan": {
///      "title": "OAuth 2.0 Device Code Lifespan",
///      "description": "Max time before the device code and user code are expired. This value needs to be a long enough lifetime to be usable (allowing the user to retrieve their secondary device, navigate to the verification URI, login, etc.), but should be sufficiently short to limit the usability of a code obtained for phishing.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "oauth2DevicePollingInterval": {
///      "title": "OAuth 2.0 Device Polling Interval",
///      "description": "The minimum amount of time in seconds that the client should wait between polling requests to the token endpoint.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "oauthClients": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/OAuthClientRepresentation"
///      }
///    },
///    "offlineSessionIdleTimeout": {
///      "title": "Offline Session Idle",
///      "description": "Time an offline session is allowed to be idle before it expires. You need to use offline token to refresh at least once within this period; otherwise offline session will expire.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "offlineSessionMaxLifespan": {
///      "title": "Offline Session Max",
///      "description": "Max time before an offline session is expired regardless of activity.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "offlineSessionMaxLifespanEnabled": {
///      "title": "Offline Session Max Limited",
///      "description": "Enable offline session maximum lifetime",
///      "type": "boolean"
///    },
///    "organizations": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/OrganizationRepresentation"
///      }
///    },
///    "organizationsEnabled": {
///      "type": "boolean"
///    },
///    "otpPolicyAlgorithm": {
///      "title": "OTP hash algorithm",
///      "description": "What hashing algorithm should be used to generate the OTP.",
///      "type": "string"
///    },
///    "otpPolicyCodeReusable": {
///      "title": "Reusable token",
///      "description": "Possibility to use the same OTP code again after successful authentication.",
///      "type": "boolean"
///    },
///    "otpPolicyDigits": {
///      "title": "Number of digits",
///      "description": "How many digits should the OTP have?",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "otpPolicyInitialCounter": {
///      "title": "Initial counter",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "otpPolicyLookAheadWindow": {
///      "title": "Look around window",
///      "description": "How far around (extra token periods or counts) should the server look just in case the token generator and server are out of time sync or counter sync?",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "otpPolicyPeriod": {
///      "title": "OTP Token period",
///      "description": "How many seconds should an OTP token be valid? Defaults to 30 seconds.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "otpPolicyType": {
///      "title": "OTP type",
///      "description": "totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.",
///      "type": "string",
///      "enum": [
///        "totp",
///        "hotp"
///      ]
///    },
///    "otpSupportedApplications": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "passwordCredentialGrantAllowed": {
///      "type": "boolean"
///    },
///    "passwordPolicy": {
///      "type": "string"
///    },
///    "permanentLockout": {
///      "title": "Permanent lockout",
///      "type": "boolean"
///    },
///    "privateKey": {
///      "type": "string"
///    },
///    "protocolMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ProtocolMapperRepresentation"
///      }
///    },
///    "publicKey": {
///      "type": "string"
///    },
///    "quickLoginCheckMilliSeconds": {
///      "title": "Quick login check milliseconds",
///      "description": "If a failure happens concurrently too quickly, lock out the user.",
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "realm": {
///      "title": "Realm ID",
///      "type": "string"
///    },
///    "realmCacheEnabled": {
///      "type": "boolean"
///    },
///    "refreshTokenMaxReuse": {
///      "title": "Refresh Token Max Reuse",
///      "description": "Maximum number of times a refresh token can be reused. When a different token is used, revocation is immediate.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "registrationAllowed": {
///      "title": "User registration",
///      "description": "Enable/disable the registration page. A link for registration will show on login page too.",
///      "type": "boolean"
///    },
///    "registrationEmailAsUsername": {
///      "title": "Email as username",
///      "description": "Allow users to set email as username.",
///      "type": "boolean"
///    },
///    "registrationFlow": {
///      "type": "string"
///    },
///    "rememberMe": {
///      "title": "Remember me",
///      "description": "Show checkbox on login page to allow user to remain logged in between browser restarts until session expires.",
///      "type": "boolean"
///    },
///    "requiredActions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/RequiredActionProviderRepresentation"
///      }
///    },
///    "requiredCredentials": {
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
///      "title": "Specifies independent timeout for forgot password.",
///      "description": "Show a link on login page for user to click when they have forgotten their credentials.",
///      "type": "boolean"
///    },
///    "revokeRefreshToken": {
///      "title": "Revoke Refresh Token",
///      "description": "If enabled a refresh token can only be used up to 'Refresh Token Max Reuse' and is revoked when a different token is used. Otherwise refresh tokens are not revoked when used and can be used multiple times.",
///      "type": "boolean"
///    },
///    "roles": {
///      "$ref": "#/$defs/RolesRepresentation"
///    },
///    "scopeMappings": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ScopeMappingRepresentation"
///      }
///    },
///    "smtpServer": {
///      "type": "object",
///      "properties": {
///        "auth": {
///          "title": "Authentication",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "envelopeFrom": {
///          "title": "Envelope from",
///          "description": "An email address used for bounces (optional).",
///          "type": "string"
///        },
///        "from": {
///          "title": "From",
///          "type": "string"
///        },
///        "fromDisplayName": {
///          "title": "From display name",
///          "description": "A user-friendly name for the 'From' address (optional).",
///          "type": "string"
///        },
///        "host": {
///          "title": "Host",
///          "type": "string"
///        },
///        "password": {
///          "title": "Password",
///          "description": "SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.",
///          "type": "string"
///        },
///        "port": {
///          "title": "Port",
///          "type": "string"
///        },
///        "replyTo": {
///          "title": "Reply to",
///          "type": "string"
///        },
///        "replyToDisplayName": {
///          "title": "Reply to display name",
///          "description": "A user-friendly name for the 'Reply-To' address (optional).",
///          "type": "string"
///        },
///        "ssl": {
///          "title": "Enable SSL",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "starttls": {
///          "title": "Enable StartTLS",
///          "type": "string",
///          "enum": [
///            "true",
///            "false",
///            ""
///          ]
///        },
///        "user": {
///          "title": "Username",
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "social": {
///      "type": "boolean"
///    },
///    "socialProviders": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "sslRequired": {
///      "title": "Require SSL",
///      "description": "Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.",
///      "type": "string",
///      "enum": [
///        "all",
///        "external",
///        "none"
///      ]
///    },
///    "ssoSessionIdleTimeout": {
///      "title": "SSO Session Idle",
///      "description": "Time a session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "ssoSessionIdleTimeoutRememberMe": {
///      "title": "SSO Session Idle Remember Me",
///      "description": "Time a remember me session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Idle value.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "ssoSessionMaxLifespan": {
///      "title": "SSO Session Max",
///      "description": "Max time before a session is expired. Tokens and browser sessions are invalidated when a session is expired.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "ssoSessionMaxLifespanRememberMe": {
///      "title": "SSO Session Max Remember Me",
///      "description": "Max time before a session is expired when a user has set the remember me option. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Max value.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "supportedLocales": {
///      "title": "Supported locales",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "updateProfileOnInitialSocialLogin": {
///      "type": "boolean"
///    },
///    "userCacheEnabled": {
///      "type": "boolean"
///    },
///    "userFederationMappers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UserFederationMapperRepresentation"
///      }
///    },
///    "userFederationProviders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UserFederationProviderRepresentation"
///      }
///    },
///    "userManagedAccessAllowed": {
///      "title": "User-managed access",
///      "description": "If enabled, users are allowed to manage their resources and permissions using the Account Management UI.",
///      "type": "boolean"
///    },
///    "users": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UserRepresentation"
///      }
///    },
///    "verifiableCredentialsEnabled": {
///      "type": "boolean"
///    },
///    "verifyEmail": {
///      "title": "Verify email",
///      "description": "Require user to verify their email address after initial login or after address changes are submitted.",
///      "type": "boolean"
///    },
///    "waitIncrementSeconds": {
///      "title": "Wait increment",
///      "description": "When failure threshold has been met, how much time should the user be locked out?",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "webAuthnPolicyAcceptableAaguids": {
///      "title": "Acceptable AAGUIDs",
///      "description": "The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyAttestationConveyancePreference": {
///      "title": "Attestation conveyance preference",
///      "description": "Communicates to an authenticator the preference of how to generate an attestation statement.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "none",
///        "indirect",
///        "direct"
///      ]
///    },
///    "webAuthnPolicyAuthenticatorAttachment": {
///      "title": "Authenticator Attachment",
///      "description": "Communicates to an authenticator an acceptable attachment pattern.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "platform",
///        "cross-platform"
///      ]
///    },
///    "webAuthnPolicyAvoidSameAuthenticatorRegister": {
///      "title": "Avoid same authenticator registration",
///      "description": "Avoid registering an authenticator that has already been registered.",
///      "type": "boolean"
///    },
///    "webAuthnPolicyCreateTimeout": {
///      "title": "Timeout",
///      "description": "The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "webAuthnPolicyExtraOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessAcceptableAaguids": {
///      "title": "Acceptable AAGUIDs",
///      "description": "The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessAttestationConveyancePreference": {
///      "title": "Attestation conveyance preference",
///      "description": "Communicates to an authenticator the preference of how to generate an attestation statement.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "none",
///        "indirect",
///        "direct"
///      ]
///    },
///    "webAuthnPolicyPasswordlessAuthenticatorAttachment": {
///      "title": "Authenticator Attachment",
///      "description": "Communicates to an authenticator an acceptable attachment pattern.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "platform",
///        "cross-platform"
///      ]
///    },
///    "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister": {
///      "title": "Avoid same authenticator registration",
///      "description": "Avoid registering an authenticator that has already been registered.",
///      "type": "boolean"
///    },
///    "webAuthnPolicyPasswordlessCreateTimeout": {
///      "title": "Timeout",
///      "description": "The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.",
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "webAuthnPolicyPasswordlessExtraOrigins": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "webAuthnPolicyPasswordlessRequireResidentKey": {
///      "title": "Require discoverable credential",
///      "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "Yes",
///        "No"
///      ]
///    },
///    "webAuthnPolicyPasswordlessRpEntityName": {
///      "title": "Relying party entity name",
///      "description": "Human-readable server name as WebAuthn Relying Party",
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessRpId": {
///      "title": "Relying party ID",
///      "description": "The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.",
///      "type": "string"
///    },
///    "webAuthnPolicyPasswordlessSignatureAlgorithms": {
///      "title": "Signature algorithms",
///      "description": "The signature algorithms that should be used for the Authentication Assertion.",
///      "type": "array",
///      "items": {
///        "type": "string",
///        "enum": [
///          "Ed25519",
///          "ES256",
///          "ES384",
///          "ES512",
///          "RS256",
///          "RS384",
///          "RS512",
///          "RS1"
///        ]
///      }
///    },
///    "webAuthnPolicyPasswordlessUserVerificationRequirement": {
///      "title": "User verification requirement",
///      "description": "Communicates to an authenticator whether to require to verify a user.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "required",
///        "preferred",
///        "discouraged"
///      ]
///    },
///    "webAuthnPolicyRequireResidentKey": {
///      "title": "Require discoverable credential",
///      "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "Yes",
///        "No"
///      ]
///    },
///    "webAuthnPolicyRpEntityName": {
///      "title": "Relying party entity name",
///      "description": "Human-readable server name as WebAuthn Relying Party",
///      "type": "string"
///    },
///    "webAuthnPolicyRpId": {
///      "title": "Relying party ID",
///      "description": "The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.",
///      "type": "string"
///    },
///    "webAuthnPolicySignatureAlgorithms": {
///      "title": "Signature algorithms",
///      "description": "The signature algorithms that should be used for the Authentication Assertion.",
///      "type": "array",
///      "items": {
///        "type": "string",
///        "enum": [
///          "Ed25519",
///          "ES256",
///          "ES384",
///          "ES512",
///          "RS256",
///          "RS384",
///          "RS512",
///          "RS1"
///        ]
///      }
///    },
///    "webAuthnPolicyUserVerificationRequirement": {
///      "title": "User verification requirement",
///      "description": "Communicates to an authenticator whether to require to verify a user.",
///      "type": "string",
///      "enum": [
///        "not specified",
///        "required",
///        "preferred",
///        "discouraged"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RealmRepresentation {
    ///Max time a client has to finish the access token protocol. This should normally be 1 minute.
    #[serde(
        rename = "accessCodeLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_code_lifespan: ::std::option::Option<i32>,
    ///Max time a user has to complete a login. This is recommended to be relatively long, such as 30 minutes or more.
    #[serde(
        rename = "accessCodeLifespanLogin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_code_lifespan_login: ::std::option::Option<i32>,
    ///Max time a user has to complete login related actions like update password or configure totp. This is recommended to be relatively long, such as 5 minutes or more.
    #[serde(
        rename = "accessCodeLifespanUserAction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_code_lifespan_user_action: ::std::option::Option<i32>,
    ///Max time before an access token is expired. This value is recommended to be short relative to the SSO timeout.
    #[serde(
        rename = "accessTokenLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_token_lifespan: ::std::option::Option<i32>,
    ///Max time before an access token issued during OpenID Connect Implicit Flow is expired. This value is recommended to be shorter than the SSO timeout. There is no possibility to refresh token during implicit flow, that's why there is a separate timeout different to 'Access Token Lifespan'.
    #[serde(
        rename = "accessTokenLifespanForImplicitFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub access_token_lifespan_for_implicit_flow: ::std::option::Option<i32>,
    ///Select theme for login, OTP, grant, registration and forgot password pages.
    #[serde(
        rename = "accountTheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_theme: ::std::option::Option<::std::string::String>,
    ///Maximum time before an action permit sent to a user by administrator is expired. This value is recommended to be long to allow administrators to send e-mails for users that are currently offline. The default timeout can be overridden immediately before issuing the token.
    #[serde(
        rename = "actionTokenGeneratedByAdminLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub action_token_generated_by_admin_lifespan: ::std::option::Option<i32>,
    ///Maximum time before an action permit sent by a user (such as a forgot password e-mail) is expired. This value is recommended to be short because it's expected that the user would react to self-created action quickly.
    #[serde(
        rename = "actionTokenGeneratedByUserLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub action_token_generated_by_user_lifespan: ::std::option::Option<i32>,
    ///Include JSON representation for create and update requests.
    #[serde(
        rename = "adminEventsDetailsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_events_details_enabled: ::std::option::Option<bool>,
    ///If enabled, admin events are saved to the database, which makes events available to the Admin UI.
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attributes: ::std::option::Option<RealmRepresentationAttributes>,
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
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub browser_security_headers: ::std::option::Option<
        RealmRepresentationBrowserSecurityHeaders,
    >,
    #[serde(
        rename = "bruteForceDetection",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brute_force_detection: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "bruteForceProtected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brute_force_protected: ::std::option::Option<bool>,
    ///Multiple means wait time will be increased only when number of failures are multiples of '{{failureFactor}}'. Linear means each new failure starting at '{{failureFactor}}' will increase wait time.
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
    ///Time a client session is allowed to be idle before it expires. Tokens are invalidated when a client session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Idle value.
    #[serde(
        rename = "clientSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_session_idle_timeout: ::std::option::Option<i32>,
    ///Max time before a client session is expired. Tokens are invalidated when a session is expired. The option does not affect the global user SSO session. If not set, it uses the standard SSO Session Max value.
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
    ///Default algorithm used to sign tokens for the realm
    #[serde(
        rename = "defaultSignatureAlgorithm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_signature_algorithm: ::std::option::Option<DefaultSignatureAlgorithm>,
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
    ///Allow multiple users to have the same email address. Changing this setting will also clear the user's cache. It is recommended to manually update email constraints of existing users in the database after switching off support for duplicate email addresses.
    #[serde(
        rename = "duplicateEmailsAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duplicate_emails_allowed: ::std::option::Option<bool>,
    ///If enabled, the username field is editable, readonly otherwise.
    #[serde(
        rename = "editUsernameAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub edit_username_allowed: ::std::option::Option<bool>,
    ///Select a theme for emails that are sent by the server.
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
    ///If enabled, user events are saved to the database, which makes events available to the admin and account management UIs.
    #[serde(
        rename = "eventsEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_enabled: ::std::option::Option<bool>,
    ///Sets the expiration for events. Expired events are periodically deleted from the database.
    #[serde(
        rename = "eventsExpiration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_expiration: ::std::option::Option<i64>,
    ///Configure what listeners receive events for the realm.
    #[serde(
        rename = "eventsListeners",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub events_listeners: ::std::vec::Vec<::std::string::String>,
    ///Max login failures
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
    ///If enabled, you can choose which locales you support for this realm and which locale is the default.
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
    ///Select theme for login, OTP, grant, registration and forgot password pages.
    #[serde(
        rename = "loginTheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_theme: ::std::option::Option<::std::string::String>,
    ///Allow users to log in with their email address.
    #[serde(
        rename = "loginWithEmailAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_with_email_allowed: ::std::option::Option<bool>,
    ///When will failure count be reset?
    #[serde(
        rename = "maxDeltaTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_delta_time_seconds: ::std::option::Option<i32>,
    ///Max time a user will be locked out.
    #[serde(
        rename = "maxFailureWaitSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_failure_wait_seconds: ::std::option::Option<i32>,
    ///The number of temporary lockouts permitted before the user is permanently locked out.
    #[serde(
        rename = "maxTemporaryLockouts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_temporary_lockouts: ::std::option::Option<i32>,
    ///How long to wait after a quick login failure.
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
    ///Max time before the device code and user code are expired. This value needs to be a long enough lifetime to be usable (allowing the user to retrieve their secondary device, navigate to the verification URI, login, etc.), but should be sufficiently short to limit the usability of a code obtained for phishing.
    #[serde(
        rename = "oauth2DeviceCodeLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub oauth2_device_code_lifespan: ::std::option::Option<i32>,
    ///The minimum amount of time in seconds that the client should wait between polling requests to the token endpoint.
    #[serde(
        rename = "oauth2DevicePollingInterval",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub oauth2_device_polling_interval: ::std::option::Option<i32>,
    #[serde(
        rename = "oauthClients",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub oauth_clients: ::std::vec::Vec<OAuthClientRepresentation>,
    ///Time an offline session is allowed to be idle before it expires. You need to use offline token to refresh at least once within this period; otherwise offline session will expire.
    #[serde(
        rename = "offlineSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offline_session_idle_timeout: ::std::option::Option<i32>,
    ///Max time before an offline session is expired regardless of activity.
    #[serde(
        rename = "offlineSessionMaxLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offline_session_max_lifespan: ::std::option::Option<i32>,
    ///Enable offline session maximum lifetime
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
    ///What hashing algorithm should be used to generate the OTP.
    #[serde(
        rename = "otpPolicyAlgorithm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_algorithm: ::std::option::Option<::std::string::String>,
    ///Possibility to use the same OTP code again after successful authentication.
    #[serde(
        rename = "otpPolicyCodeReusable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_code_reusable: ::std::option::Option<bool>,
    ///How many digits should the OTP have?
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
    ///How far around (extra token periods or counts) should the server look just in case the token generator and server are out of time sync or counter sync?
    #[serde(
        rename = "otpPolicyLookAheadWindow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_look_ahead_window: ::std::option::Option<i32>,
    ///How many seconds should an OTP token be valid? Defaults to 30 seconds.
    #[serde(
        rename = "otpPolicyPeriod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_period: ::std::option::Option<i32>,
    ///totp is Time-Based One Time Password. 'hotp' is a counter base one time password in which the server keeps a counter to hash against.
    #[serde(
        rename = "otpPolicyType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub otp_policy_type: ::std::option::Option<OtpType>,
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
    ///If a failure happens concurrently too quickly, lock out the user.
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
    ///Maximum number of times a refresh token can be reused. When a different token is used, revocation is immediate.
    #[serde(
        rename = "refreshTokenMaxReuse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_token_max_reuse: ::std::option::Option<i32>,
    ///Enable/disable the registration page. A link for registration will show on login page too.
    #[serde(
        rename = "registrationAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registration_allowed: ::std::option::Option<bool>,
    ///Allow users to set email as username.
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
    ///Show checkbox on login page to allow user to remain logged in between browser restarts until session expires.
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
    ///Show a link on login page for user to click when they have forgotten their credentials.
    #[serde(
        rename = "resetPasswordAllowed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reset_password_allowed: ::std::option::Option<bool>,
    ///If enabled a refresh token can only be used up to 'Refresh Token Max Reuse' and is revoked when a different token is used. Otherwise refresh tokens are not revoked when used and can be used multiple times.
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
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub smtp_server: ::std::option::Option<RealmRepresentationSmtpServer>,
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
    ///Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.
    #[serde(
        rename = "sslRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ssl_required: ::std::option::Option<RequireSsl>,
    ///Time a session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired.
    #[serde(
        rename = "ssoSessionIdleTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_idle_timeout: ::std::option::Option<i32>,
    ///Time a remember me session is allowed to be idle before it expires. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Idle value.
    #[serde(
        rename = "ssoSessionIdleTimeoutRememberMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_idle_timeout_remember_me: ::std::option::Option<i32>,
    ///Max time before a session is expired. Tokens and browser sessions are invalidated when a session is expired.
    #[serde(
        rename = "ssoSessionMaxLifespan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sso_session_max_lifespan: ::std::option::Option<i32>,
    ///Max time before a session is expired when a user has set the remember me option. Tokens and browser sessions are invalidated when a session is expired. If not set it uses the standard SSO Session Max value.
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
    ///If enabled, users are allowed to manage their resources and permissions using the Account Management UI.
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
    ///Require user to verify their email address after initial login or after address changes are submitted.
    #[serde(
        rename = "verifyEmail",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verify_email: ::std::option::Option<bool>,
    ///When failure threshold has been met, how much time should the user be locked out?
    #[serde(
        rename = "waitIncrementSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wait_increment_seconds: ::std::option::Option<i32>,
    ///The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).
    #[serde(
        rename = "webAuthnPolicyAcceptableAaguids",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_acceptable_aaguids: ::std::vec::Vec<::std::string::String>,
    ///Communicates to an authenticator the preference of how to generate an attestation statement.
    #[serde(
        rename = "webAuthnPolicyAttestationConveyancePreference",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_attestation_conveyance_preference: ::std::option::Option<
        AttestationConveyancePreference,
    >,
    ///Communicates to an authenticator an acceptable attachment pattern.
    #[serde(
        rename = "webAuthnPolicyAuthenticatorAttachment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_authenticator_attachment: ::std::option::Option<
        AuthenticatorAttachment,
    >,
    ///Avoid registering an authenticator that has already been registered.
    #[serde(
        rename = "webAuthnPolicyAvoidSameAuthenticatorRegister",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_avoid_same_authenticator_register: ::std::option::Option<bool>,
    ///The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.
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
    ///The list of allowed AAGUIDs of which an authenticator can be registered. An AAGUID is a 128-bit identifier indicating the authenticator's type (e.g., make and model).
    #[serde(
        rename = "webAuthnPolicyPasswordlessAcceptableAaguids",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_passwordless_acceptable_aaguids: ::std::vec::Vec<
        ::std::string::String,
    >,
    ///Communicates to an authenticator the preference of how to generate an attestation statement.
    #[serde(
        rename = "webAuthnPolicyPasswordlessAttestationConveyancePreference",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_attestation_conveyance_preference: ::std::option::Option<
        AttestationConveyancePreference,
    >,
    ///Communicates to an authenticator an acceptable attachment pattern.
    #[serde(
        rename = "webAuthnPolicyPasswordlessAuthenticatorAttachment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_authenticator_attachment: ::std::option::Option<
        AuthenticatorAttachment,
    >,
    ///Avoid registering an authenticator that has already been registered.
    #[serde(
        rename = "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_avoid_same_authenticator_register: ::std::option::Option<
        bool,
    >,
    ///The timeout value for creating the user's public key credential in seconds. If set to 0, this timeout option is not adapted.
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
    ///It tells an authenticator whether to create a public key credential as a Discoverable Credential.
    #[serde(
        rename = "webAuthnPolicyPasswordlessRequireResidentKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_require_resident_key: ::std::option::Option<
        RequireDiscoverableCredential,
    >,
    ///Human-readable server name as WebAuthn Relying Party
    #[serde(
        rename = "webAuthnPolicyPasswordlessRpEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_rp_entity_name: ::std::option::Option<
        ::std::string::String,
    >,
    ///The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.
    #[serde(
        rename = "webAuthnPolicyPasswordlessRpId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_rp_id: ::std::option::Option<
        ::std::string::String,
    >,
    ///The signature algorithms that should be used for the Authentication Assertion.
    #[serde(
        rename = "webAuthnPolicyPasswordlessSignatureAlgorithms",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_passwordless_signature_algorithms: ::std::vec::Vec<
        SignatureAlgorithmsItem,
    >,
    ///Communicates to an authenticator whether to require to verify a user.
    #[serde(
        rename = "webAuthnPolicyPasswordlessUserVerificationRequirement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_passwordless_user_verification_requirement: ::std::option::Option<
        UserVerificationRequirement,
    >,
    ///It tells an authenticator whether to create a public key credential as a Discoverable Credential.
    #[serde(
        rename = "webAuthnPolicyRequireResidentKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_require_resident_key: ::std::option::Option<
        RequireDiscoverableCredential,
    >,
    ///Human-readable server name as WebAuthn Relying Party
    #[serde(
        rename = "webAuthnPolicyRpEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_rp_entity_name: ::std::option::Option<::std::string::String>,
    ///The WebAuthn Relying Party ID (RpID). It must be the origin's effective domain, e.g. 'company.com' or 'auth.company.com'.
    #[serde(
        rename = "webAuthnPolicyRpId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_rp_id: ::std::option::Option<::std::string::String>,
    ///The signature algorithms that should be used for the Authentication Assertion.
    #[serde(
        rename = "webAuthnPolicySignatureAlgorithms",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub web_authn_policy_signature_algorithms: ::std::vec::Vec<SignatureAlgorithmsItem>,
    ///Communicates to an authenticator whether to require to verify a user.
    #[serde(
        rename = "webAuthnPolicyUserVerificationRequirement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub web_authn_policy_user_verification_requirement: ::std::option::Option<
        UserVerificationRequirement,
    >,
}
impl ::std::convert::From<&RealmRepresentation> for RealmRepresentation {
    fn from(value: &RealmRepresentation) -> Self {
        value.clone()
    }
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
            brute_force_detection: Default::default(),
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
            oauth2_device_code_lifespan: Default::default(),
            oauth2_device_polling_interval: Default::default(),
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
///RealmRepresentationAttributes
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "adminEventsExpiration": {
///      "title": "Expiration",
///      "description": "Sets the expiration for events. Expired events are periodically deleted from the database.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "cibaAuthRequestedUserHint": {
///      "title": "Authentication Requested User Hint",
///      "description": "The way of identifying the end-user for whom authentication is being requested. Currently only \"login_hint\" is supported.",
///      "type": "string",
///      "enum": [
///        "login_hint"
///      ]
///    },
///    "cibaBackchannelTokenDeliveryMode": {
///      "title": "Backchannel Token Delivery Mode",
///      "description": "Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.",
///      "type": "string",
///      "enum": [
///        "ping",
///        "poll"
///      ]
///    },
///    "cibaExpiresIn": {
///      "title": "Expires In",
///      "description": "The expiration time of the \"auth_req_id\" in seconds since the authentication request was received.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "cibaInterval": {
///      "title": "Interval",
///      "description": "The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.",
///      "type": "string",
///      "pattern": "^[0-9]*$"
///    },
///    "frontendUrl": {
///      "title": "Frontend URL",
///      "description": "Set the frontend URL for the realm. Use in combination with the default hostname provider to override the base URL for frontend requests for a specific realm.",
///      "type": "string"
///    }
///  },
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RealmRepresentationAttributes {
    ///Sets the expiration for events. Expired events are periodically deleted from the database.
    #[serde(
        rename = "adminEventsExpiration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_events_expiration: ::std::option::Option<Expiration>,
    ///The way of identifying the end-user for whom authentication is being requested. Currently only "login_hint" is supported.
    #[serde(
        rename = "cibaAuthRequestedUserHint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ciba_auth_requested_user_hint: ::std::option::Option<
        AuthenticationRequestedUserHint,
    >,
    ///Specifies how the CD (Consumption Device) gets the authentication result and related tokens. This mode will be used by default for the CIBA clients, which do not have other mode explicitly set.
    #[serde(
        rename = "cibaBackchannelTokenDeliveryMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ciba_backchannel_token_delivery_mode: ::std::option::Option<
        BackchannelTokenDeliveryMode,
    >,
    ///The expiration time of the "auth_req_id" in seconds since the authentication request was received.
    #[serde(
        rename = "cibaExpiresIn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ciba_expires_in: ::std::option::Option<ExpiresIn>,
    ///The minimum amount of time in seconds that the CD (Consumption Device) must wait between polling requests to the token endpoint. If set to 0, the CD must use 5 as the default value according to the CIBA specification.
    #[serde(
        rename = "cibaInterval",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ciba_interval: ::std::option::Option<Interval>,
    ///Set the frontend URL for the realm. Use in combination with the default hostname provider to override the base URL for frontend requests for a specific realm.
    #[serde(
        rename = "frontendUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frontend_url: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::std::collections::BTreeMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
impl ::std::convert::From<&RealmRepresentationAttributes>
for RealmRepresentationAttributes {
    fn from(value: &RealmRepresentationAttributes) -> Self {
        value.clone()
    }
}
///RealmRepresentationBrowserSecurityHeaders
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "contentSecurityPolicy": {
///      "title": "Content-Security-Policy",
///      "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>",
///      "type": "string"
///    },
///    "contentSecurityPolicyReportOnly": {
///      "title": "Content-Security-Policy-Report-Only",
///      "description": "For testing Content Security Policies <1>Learn more</1>",
///      "type": "string"
///    },
///    "strictTransportSecurity": {
///      "title": "HTTP Strict Transport Security (HSTS)",
///      "description": "The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>",
///      "type": "string"
///    },
///    "xContentTypeOptions": {
///      "title": "X-Content-Type-Options",
///      "description": "The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>",
///      "type": "string"
///    },
///    "xFrameOptions": {
///      "title": "X-Frame-Options",
///      "description": "Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>",
///      "type": "string"
///    },
///    "xRobotsTag": {
///      "title": "X-Robots-Tag",
///      "description": "Prevent pages from appearing in search engines. <1>Learn more</1>",
///      "type": "string"
///    },
///    "xXSSProtection": {
///      "title": "X-XSS-Protection",
///      "description": "This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RealmRepresentationBrowserSecurityHeaders {
    ///Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>
    #[serde(
        rename = "contentSecurityPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub content_security_policy: ::std::option::Option<::std::string::String>,
    ///For testing Content Security Policies <1>Learn more</1>
    #[serde(
        rename = "contentSecurityPolicyReportOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub content_security_policy_report_only: ::std::option::Option<
        ::std::string::String,
    >,
    ///The Strict-Transport-Security HTTP header tells browsers to always use HTTPS. Once a browser sees this header, it will only visit the site over HTTPS for the time specified (1 year) at max-age, including the subdomains. <1>Learn more</1>
    #[serde(
        rename = "strictTransportSecurity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strict_transport_security: ::std::option::Option<::std::string::String>,
    ///The default value prevents Internet Explorer and Google Chrome from MIME-sniffing a response away from the declared content-type. <1>Learn more</1>
    #[serde(
        rename = "xContentTypeOptions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x_content_type_options: ::std::option::Option<::std::string::String>,
    ///Default value prevents pages from being included by non-origin iframes. <1>Learn more</1>
    #[serde(
        rename = "xFrameOptions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x_frame_options: ::std::option::Option<::std::string::String>,
    ///Prevent pages from appearing in search engines. <1>Learn more</1>
    #[serde(
        rename = "xRobotsTag",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x_robots_tag: ::std::option::Option<::std::string::String>,
    ///This header configures the Cross-site scripting (XSS) filter in your browser. Using the default behaviour, the browser will prevent rendering of the page when a XSS attack is detected. <1>Learn more</1>
    #[serde(
        rename = "xXSSProtection",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x_xss_protection: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&RealmRepresentationBrowserSecurityHeaders>
for RealmRepresentationBrowserSecurityHeaders {
    fn from(value: &RealmRepresentationBrowserSecurityHeaders) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RealmRepresentationBrowserSecurityHeaders {
    fn default() -> Self {
        Self {
            content_security_policy: Default::default(),
            content_security_policy_report_only: Default::default(),
            strict_transport_security: Default::default(),
            x_content_type_options: Default::default(),
            x_frame_options: Default::default(),
            x_robots_tag: Default::default(),
            x_xss_protection: Default::default(),
        }
    }
}
///RealmRepresentationSmtpServer
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "auth": {
///      "title": "Authentication",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "envelopeFrom": {
///      "title": "Envelope from",
///      "description": "An email address used for bounces (optional).",
///      "type": "string"
///    },
///    "from": {
///      "title": "From",
///      "type": "string"
///    },
///    "fromDisplayName": {
///      "title": "From display name",
///      "description": "A user-friendly name for the 'From' address (optional).",
///      "type": "string"
///    },
///    "host": {
///      "title": "Host",
///      "type": "string"
///    },
///    "password": {
///      "title": "Password",
///      "description": "SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.",
///      "type": "string"
///    },
///    "port": {
///      "title": "Port",
///      "type": "string"
///    },
///    "replyTo": {
///      "title": "Reply to",
///      "type": "string"
///    },
///    "replyToDisplayName": {
///      "title": "Reply to display name",
///      "description": "A user-friendly name for the 'Reply-To' address (optional).",
///      "type": "string"
///    },
///    "ssl": {
///      "title": "Enable SSL",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "starttls": {
///      "title": "Enable StartTLS",
///      "type": "string",
///      "enum": [
///        "true",
///        "false",
///        ""
///      ]
///    },
///    "user": {
///      "title": "Username",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RealmRepresentationSmtpServer {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub auth: ::std::option::Option<Authentication>,
    ///An email address used for bounces (optional).
    #[serde(
        rename = "envelopeFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub envelope_from: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from: ::std::option::Option<::std::string::String>,
    ///A user-friendly name for the 'From' address (optional).
    #[serde(
        rename = "fromDisplayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub from_display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub host: ::std::option::Option<::std::string::String>,
    ///SMTP password. This field is able to obtain its value from vault, use ${vault.ID} format.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "replyTo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reply_to: ::std::option::Option<::std::string::String>,
    ///A user-friendly name for the 'Reply-To' address (optional).
    #[serde(
        rename = "replyToDisplayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reply_to_display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ssl: ::std::option::Option<EnableSsl>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub starttls: ::std::option::Option<EnableStartTls>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&RealmRepresentationSmtpServer>
for RealmRepresentationSmtpServer {
    fn from(value: &RealmRepresentationSmtpServer) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RealmRepresentationSmtpServer {
    fn default() -> Self {
        Self {
            auth: Default::default(),
            envelope_from: Default::default(),
            from: Default::default(),
            from_display_name: Default::default(),
            host: Default::default(),
            password: Default::default(),
            port: Default::default(),
            reply_to: Default::default(),
            reply_to_display_name: Default::default(),
            ssl: Default::default(),
            starttls: Default::default(),
            user: Default::default(),
        }
    }
}
///It tells an authenticator whether to create a public key credential as a Discoverable Credential.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Require discoverable credential",
///  "description": "It tells an authenticator whether to create a public key credential as a Discoverable Credential.",
///  "type": "string",
///  "enum": [
///    "not specified",
///    "Yes",
///    "No"
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
pub enum RequireDiscoverableCredential {
    #[serde(rename = "not specified")]
    NotSpecified,
    Yes,
    No,
}
impl ::std::convert::From<&Self> for RequireDiscoverableCredential {
    fn from(value: &RequireDiscoverableCredential) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RequireDiscoverableCredential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotSpecified => write!(f, "not specified"),
            Self::Yes => write!(f, "Yes"),
            Self::No => write!(f, "No"),
        }
    }
}
impl ::std::str::FromStr for RequireDiscoverableCredential {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "not specified" => Ok(Self::NotSpecified),
            "Yes" => Ok(Self::Yes),
            "No" => Ok(Self::No),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequireDiscoverableCredential {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequireDiscoverableCredential {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequireDiscoverableCredential {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Require SSL",
///  "description": "Is HTTPS required? 'None' means HTTPS is not required for any client IP address. 'External requests' means localhost and private IP addresses can access without HTTPS. 'All requests' means HTTPS is required for all IP addresses.",
///  "type": "string",
///  "enum": [
///    "all",
///    "external",
///    "none"
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
pub enum RequireSsl {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "none")]
    None,
}
impl ::std::convert::From<&Self> for RequireSsl {
    fn from(value: &RequireSsl) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RequireSsl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => write!(f, "all"),
            Self::External => write!(f, "external"),
            Self::None => write!(f, "none"),
        }
    }
}
impl ::std::str::FromStr for RequireSsl {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "external" => Ok(Self::External),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequireSsl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequireSsl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequireSsl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///RequiredActionConfigInfoRepresentation
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
///        "$ref": "#/$defs/ConfigPropertyRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RequiredActionConfigInfoRepresentation {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub properties: ::std::vec::Vec<ConfigPropertyRepresentation>,
}
impl ::std::convert::From<&RequiredActionConfigInfoRepresentation>
for RequiredActionConfigInfoRepresentation {
    fn from(value: &RequiredActionConfigInfoRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RequiredActionConfigInfoRepresentation {
    fn default() -> Self {
        Self {
            properties: Default::default(),
        }
    }
}
///RequiredActionConfigRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&RequiredActionConfigRepresentation>
for RequiredActionConfigRepresentation {
    fn from(value: &RequiredActionConfigRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RequiredActionConfigRepresentation {
    fn default() -> Self {
        Self { config: Default::default() }
    }
}
///RequiredActionProviderRepresentation
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "providerId": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&RequiredActionProviderRepresentation>
for RequiredActionProviderRepresentation {
    fn from(value: &RequiredActionProviderRepresentation) -> Self {
        value.clone()
    }
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
///ResourceOwnerRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ResourceOwnerRepresentation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResourceOwnerRepresentation> for ResourceOwnerRepresentation {
    fn from(value: &ResourceOwnerRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResourceOwnerRepresentation {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}
///ResourceRepresentation
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
///      "type": "object",
///      "allOf": [
///        {
///          "$ref": "#/$defs/ResourceOwnerRepresentation"
///        }
///      ]
///    },
///    "ownerManagedAccess": {
///      "type": "boolean"
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "scopesUma": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ScopeRepresentation"
///      },
///      "uniqueItems": true
///    },
///    "type": {
///      "type": "string"
///    },
///    "uri": {
///      "type": "string"
///    },
///    "uris": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ResourceRepresentation> for ResourceRepresentation {
    fn from(value: &ResourceRepresentation) -> Self {
        value.clone()
    }
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
///ResourceServerRepresentation
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
///      "$ref": "#/$defs/AuthorizationSchema"
///    },
///    "clientId": {
///      "type": "string"
///    },
///    "decisionStrategy": {
///      "$ref": "#/$defs/DecisionStrategy"
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
///        "$ref": "#/$defs/PolicyRepresentation"
///      }
///    },
///    "policyEnforcementMode": {
///      "$ref": "#/$defs/PolicyEnforcementMode"
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ResourceRepresentation"
///      }
///    },
///    "scopes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ScopeRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ResourceServerRepresentation>
for ResourceServerRepresentation {
    fn from(value: &ResourceServerRepresentation) -> Self {
        value.clone()
    }
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
///ResourceType
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
///    },
///    "type": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ResourceType {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResourceType> for ResourceType {
    fn from(value: &ResourceType) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResourceType {
    fn default() -> Self {
        Self {
            scopes: Default::default(),
            type_: Default::default(),
        }
    }
}
///RoleRepresentation
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
///      "$ref": "#/$defs/Composites"
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
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&RoleRepresentation> for RoleRepresentation {
    fn from(value: &RoleRepresentation) -> Self {
        value.clone()
    }
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
///RolesRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "application": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/$defs/RoleRepresentation"
///        }
///      }
///    },
///    "client": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/$defs/RoleRepresentation"
///        }
///      }
///    },
///    "realm": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/RoleRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&RolesRepresentation> for RolesRepresentation {
    fn from(value: &RolesRepresentation) -> Self {
        value.clone()
    }
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
///Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "SAML signature key name",
///  "description": "Signed SAML documents contain identification of signing key in KeyName element. For Keycloak / RH-SSO counter-party, use KEY_ID, for MS AD FS use CERT_SUBJECT, for others check and use NONE if no other option works.",
///  "type": "string",
///  "enum": [
///    "NONE",
///    "KEY_ID",
///    "CERT_SUBJECT"
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
pub enum SamlSignatureKeyName {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "KEY_ID")]
    KeyId,
    #[serde(rename = "CERT_SUBJECT")]
    CertSubject,
}
impl ::std::convert::From<&Self> for SamlSignatureKeyName {
    fn from(value: &SamlSignatureKeyName) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SamlSignatureKeyName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => write!(f, "NONE"),
            Self::KeyId => write!(f, "KEY_ID"),
            Self::CertSubject => write!(f, "CERT_SUBJECT"),
        }
    }
}
impl ::std::str::FromStr for SamlSignatureKeyName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NONE" => Ok(Self::None),
            "KEY_ID" => Ok(Self::KeyId),
            "CERT_SUBJECT" => Ok(Self::CertSubject),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SamlSignatureKeyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SamlSignatureKeyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SamlSignatureKeyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///ScopeEnforcementMode
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
impl ::std::convert::From<&Self> for ScopeEnforcementMode {
    fn from(value: &ScopeEnforcementMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ScopeEnforcementMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => write!(f, "ALL"),
            Self::Any => write!(f, "ANY"),
            Self::Disabled => write!(f, "DISABLED"),
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
///ScopeMappingRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ScopeMappingRepresentation> for ScopeMappingRepresentation {
    fn from(value: &ScopeMappingRepresentation) -> Self {
        value.clone()
    }
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
///ScopeRepresentation
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
///        "$ref": "#/$defs/PolicyRepresentation"
///      }
///    },
///    "resources": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/ResourceRepresentation"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&ScopeRepresentation> for ScopeRepresentation {
    fn from(value: &ScopeRepresentation) -> Self {
        value.clone()
    }
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
///Enable/disable signature of the provider SAML metadata.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Sign service provider metadata",
///  "description": "Enable/disable signature of the provider SAML metadata.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum SignServiceProviderMetadata {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for SignServiceProviderMetadata {
    fn from(value: &SignServiceProviderMetadata) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SignServiceProviderMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for SignServiceProviderMetadata {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SignServiceProviderMetadata {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SignServiceProviderMetadata {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SignServiceProviderMetadata {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///SignatureAlgorithmsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Ed25519",
///    "ES256",
///    "ES384",
///    "ES512",
///    "RS256",
///    "RS384",
///    "RS512",
///    "RS1"
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
pub enum SignatureAlgorithmsItem {
    Ed25519,
    #[serde(rename = "ES256")]
    Es256,
    #[serde(rename = "ES384")]
    Es384,
    #[serde(rename = "ES512")]
    Es512,
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RS384")]
    Rs384,
    #[serde(rename = "RS512")]
    Rs512,
    #[serde(rename = "RS1")]
    Rs1,
}
impl ::std::convert::From<&Self> for SignatureAlgorithmsItem {
    fn from(value: &SignatureAlgorithmsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SignatureAlgorithmsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ed25519 => write!(f, "Ed25519"),
            Self::Es256 => write!(f, "ES256"),
            Self::Es384 => write!(f, "ES384"),
            Self::Es512 => write!(f, "ES512"),
            Self::Rs256 => write!(f, "RS256"),
            Self::Rs384 => write!(f, "RS384"),
            Self::Rs512 => write!(f, "RS512"),
            Self::Rs1 => write!(f, "RS1"),
        }
    }
}
impl ::std::str::FromStr for SignatureAlgorithmsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Ed25519" => Ok(Self::Ed25519),
            "ES256" => Ok(Self::Es256),
            "ES384" => Ok(Self::Es384),
            "ES512" => Ok(Self::Es512),
            "RS256" => Ok(Self::Rs256),
            "RS384" => Ok(Self::Rs384),
            "RS512" => Ok(Self::Rs512),
            "RS1" => Ok(Self::Rs1),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SignatureAlgorithmsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SignatureAlgorithmsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SignatureAlgorithmsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///SocialLinkRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&SocialLinkRepresentation> for SocialLinkRepresentation {
    fn from(value: &SocialLinkRepresentation) -> Self {
        value.clone()
    }
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
///Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Sync mode",
///  "description": "Default sync mode for all mappers. The sync mode determines when user data will be synced using the mappers. Possible values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider.",
///  "type": "string",
///  "enum": [
///    "IMPORT",
///    "LEGACY",
///    "FORCE"
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
pub enum SyncMode {
    #[serde(rename = "IMPORT")]
    Import,
    #[serde(rename = "LEGACY")]
    Legacy,
    #[serde(rename = "FORCE")]
    Force,
}
impl ::std::convert::From<&Self> for SyncMode {
    fn from(value: &SyncMode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SyncMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Import => write!(f, "IMPORT"),
            Self::Legacy => write!(f, "LEGACY"),
            Self::Force => write!(f, "FORCE"),
        }
    }
}
impl ::std::str::FromStr for SyncMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "IMPORT" => Ok(Self::Import),
            "LEGACY" => Ok(Self::Legacy),
            "FORCE" => Ok(Self::Force),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SyncMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Sync mode override",
///  "description": "Overrides the default sync mode of the IDP for this mapper. Values are: 'legacy' to keep the behaviour before this option was introduced, 'import' to only import the user once during first login of the user with this identity provider, 'force' to always update the user during every login with this identity provider and 'inherit' to use the sync mode defined in the identity provider for this mapper.",
///  "type": "string",
///  "enum": [
///    "INHERIT",
///    "IMPORT",
///    "LEGACY",
///    "FORCE"
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
pub enum SyncModeOverride {
    #[serde(rename = "INHERIT")]
    Inherit,
    #[serde(rename = "IMPORT")]
    Import,
    #[serde(rename = "LEGACY")]
    Legacy,
    #[serde(rename = "FORCE")]
    Force,
}
impl ::std::convert::From<&Self> for SyncModeOverride {
    fn from(value: &SyncModeOverride) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SyncModeOverride {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inherit => write!(f, "INHERIT"),
            Self::Import => write!(f, "IMPORT"),
            Self::Legacy => write!(f, "LEGACY"),
            Self::Force => write!(f, "FORCE"),
        }
    }
}
impl ::std::str::FromStr for SyncModeOverride {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "INHERIT" => Ok(Self::Inherit),
            "IMPORT" => Ok(Self::Import),
            "LEGACY" => Ok(Self::Legacy),
            "FORCE" => Ok(Self::Force),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SyncModeOverride {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncModeOverride {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncModeOverride {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///UnmanagedAttributePolicy
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
impl ::std::convert::From<&Self> for UnmanagedAttributePolicy {
    fn from(value: &UnmanagedAttributePolicy) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UnmanagedAttributePolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enabled => write!(f, "ENABLED"),
            Self::AdminView => write!(f, "ADMIN_VIEW"),
            Self::AdminEdit => write!(f, "ADMIN_EDIT"),
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
///UpAttribute
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
///      "$ref": "#/$defs/UPAttributePermissions"
///    },
///    "required": {
///      "$ref": "#/$defs/UPAttributeRequired"
///    },
///    "selector": {
///      "$ref": "#/$defs/UPAttributeSelector"
///    },
///    "validations": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {}
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UpAttribute> for UpAttribute {
    fn from(value: &UpAttribute) -> Self {
        value.clone()
    }
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
///UpAttributePermissions
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpAttributePermissions {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub edit: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub view: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::convert::From<&UpAttributePermissions> for UpAttributePermissions {
    fn from(value: &UpAttributePermissions) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UpAttributePermissions {
    fn default() -> Self {
        Self {
            edit: Default::default(),
            view: Default::default(),
        }
    }
}
///UpAttributeRequired
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpAttributeRequired {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roles: ::std::option::Option<Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::convert::From<&UpAttributeRequired> for UpAttributeRequired {
    fn from(value: &UpAttributeRequired) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UpAttributeRequired {
    fn default() -> Self {
        Self {
            roles: Default::default(),
            scopes: Default::default(),
        }
    }
}
///UpAttributeSelector
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpAttributeSelector {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::convert::From<&UpAttributeSelector> for UpAttributeSelector {
    fn from(value: &UpAttributeSelector) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UpAttributeSelector {
    fn default() -> Self {
        Self { scopes: Default::default() }
    }
}
///UpConfig
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
///        "$ref": "#/$defs/UPAttribute"
///      }
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UPGroup"
///      }
///    },
///    "unmanagedAttributePolicy": {
///      "$ref": "#/$defs/UnmanagedAttributePolicy"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UpConfig> for UpConfig {
    fn from(value: &UpConfig) -> Self {
        value.clone()
    }
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
///UpGroup
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UpGroup> for UpGroup {
    fn from(value: &UpGroup) -> Self {
        value.clone()
    }
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
///If this is on, token responses will be set the with the type "bearer" in lower-case. By default, the server sets the type as "Bearer" as defined by RFC6750.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Use lower-case bearer type in token responses",
///  "description": "If this is on, token responses will be set the with the type \"bearer\" in lower-case. By default, the server sets the type as \"Bearer\" as defined by RFC6750.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum UseLowerCaseBearerTypeInTokenResponses {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for UseLowerCaseBearerTypeInTokenResponses {
    fn from(value: &UseLowerCaseBearerTypeInTokenResponses) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UseLowerCaseBearerTypeInTokenResponses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for UseLowerCaseBearerTypeInTokenResponses {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UseLowerCaseBearerTypeInTokenResponses {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for UseLowerCaseBearerTypeInTokenResponses {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for UseLowerCaseBearerTypeInTokenResponses {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Use refresh tokens",
///  "description": "If this is on, a refresh_token will be created and added to the token response. If this is off then no refresh_token will be generated.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum UseRefreshTokens {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for UseRefreshTokens {
    fn from(value: &UseRefreshTokens) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UseRefreshTokens {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for UseRefreshTokens {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UseRefreshTokens {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UseRefreshTokens {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UseRefreshTokens {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Use refresh tokens for client credentials grant",
///  "description": "If this is on, a refresh_token will be created and added to the token response if the client_credentials grant is used. The OAuth 2.0 RFC6749 Section 4.4.3 states that a refresh_token should not be generated when client_credentials grant is used. If this is off then no refresh_token will be generated and the associated user session will be removed.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum UseRefreshTokensForClientCredentialsGrant {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for UseRefreshTokensForClientCredentialsGrant {
    fn from(value: &UseRefreshTokensForClientCredentialsGrant) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UseRefreshTokensForClientCredentialsGrant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for UseRefreshTokensForClientCredentialsGrant {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UseRefreshTokensForClientCredentialsGrant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for UseRefreshTokensForClientCredentialsGrant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for UseRefreshTokensForClientCredentialsGrant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///UserConsentRepresentation
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "grantedClientScopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "grantedRealmRoles": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "lastUpdatedDate": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserConsentRepresentation> for UserConsentRepresentation {
    fn from(value: &UserConsentRepresentation) -> Self {
        value.clone()
    }
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
///UserFederationMapperRepresentation
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserFederationMapperRepresentation>
for UserFederationMapperRepresentation {
    fn from(value: &UserFederationMapperRepresentation) -> Self {
        value.clone()
    }
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
///UserFederationProviderRepresentation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "changedSyncPeriod": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "id": {
///      "type": "string"
///    },
///    "lastSync": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "priority": {
///      "type": "integer",
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
///    },
///    "providerName": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserFederationProviderRepresentation>
for UserFederationProviderRepresentation {
    fn from(value: &UserFederationProviderRepresentation) -> Self {
        value.clone()
    }
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
///UserManagedAccessConfig
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
impl ::std::convert::From<&UserManagedAccessConfig> for UserManagedAccessConfig {
    fn from(value: &UserManagedAccessConfig) -> Self {
        value.clone()
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
///UserProfileAttributeGroupMetadata
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserProfileAttributeGroupMetadata>
for UserProfileAttributeGroupMetadata {
    fn from(value: &UserProfileAttributeGroupMetadata) -> Self {
        value.clone()
    }
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
///UserProfileAttributeMetadata
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserProfileAttributeMetadata>
for UserProfileAttributeMetadata {
    fn from(value: &UserProfileAttributeMetadata) -> Self {
        value.clone()
    }
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
///UserProfileMetadata
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
///        "$ref": "#/$defs/UserProfileAttributeMetadata"
///      }
///    },
///    "groups": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/UserProfileAttributeGroupMetadata"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserProfileMetadata {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attributes: ::std::vec::Vec<UserProfileAttributeMetadata>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<UserProfileAttributeGroupMetadata>,
}
impl ::std::convert::From<&UserProfileMetadata> for UserProfileMetadata {
    fn from(value: &UserProfileMetadata) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for UserProfileMetadata {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            groups: Default::default(),
        }
    }
}
///UserRepresentation
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
///        "$ref": "#/$defs/UserConsentRepresentation"
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "credentials": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/CredentialRepresentation"
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
///        "$ref": "#/$defs/FederatedIdentityRepresentation"
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
///      "format": "int32",
///      "maximum": 2147483647.0,
///      "minimum": -2147483648.0
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
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SocialLinkRepresentation"
///      }
///    },
///    "totp": {
///      "type": "boolean"
///    },
///    "userProfileMetadata": {
///      "$ref": "#/$defs/UserProfileMetadata"
///    },
///    "username": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserRepresentation> for UserRepresentation {
    fn from(value: &UserRepresentation) -> Self {
        value.clone()
    }
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
///UserSessionRepresentation
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
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
///    },
///    "rememberMe": {
///      "type": "boolean"
///    },
///    "start": {
///      "type": "integer",
///      "format": "int64",
///      "maximum": 9.223372036854776e18,
///      "minimum": -9.223372036854776e18
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
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
impl ::std::convert::From<&UserSessionRepresentation> for UserSessionRepresentation {
    fn from(value: &UserSessionRepresentation) -> Self {
        value.clone()
    }
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
///Communicates to an authenticator whether to require to verify a user.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "User verification requirement",
///  "description": "Communicates to an authenticator whether to require to verify a user.",
///  "type": "string",
///  "enum": [
///    "not specified",
///    "required",
///    "preferred",
///    "discouraged"
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
pub enum UserVerificationRequirement {
    #[serde(rename = "not specified")]
    NotSpecified,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "discouraged")]
    Discouraged,
}
impl ::std::convert::From<&Self> for UserVerificationRequirement {
    fn from(value: &UserVerificationRequirement) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UserVerificationRequirement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotSpecified => write!(f, "not specified"),
            Self::Required => write!(f, "required"),
            Self::Preferred => write!(f, "preferred"),
            Self::Discouraged => write!(f, "discouraged"),
        }
    }
}
impl ::std::str::FromStr for UserVerificationRequirement {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "not specified" => Ok(Self::NotSpecified),
            "required" => Ok(Self::Required),
            "preferred" => Ok(Self::Preferred),
            "discouraged" => Ok(Self::Discouraged),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserVerificationRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UserVerificationRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserVerificationRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Enable/disable signature validation of external IDP signatures.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Validate Signatures",
///  "description": "Enable/disable signature validation of external IDP signatures.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum ValidateSignatures {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for ValidateSignatures {
    fn from(value: &ValidateSignatures) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ValidateSignatures {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for ValidateSignatures {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ValidateSignatures {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ValidateSignatures {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ValidateSignatures {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether this service provider expects an encrypted Assertion.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Want Assertions encrypted",
///  "description": "Indicates whether this service provider expects an encrypted Assertion.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum WantAssertionsEncrypted {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for WantAssertionsEncrypted {
    fn from(value: &WantAssertionsEncrypted) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WantAssertionsEncrypted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for WantAssertionsEncrypted {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WantAssertionsEncrypted {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WantAssertionsEncrypted {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WantAssertionsEncrypted {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether this service provider expects a signed Assertion.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Want Assertions signed",
///  "description": "Indicates whether this service provider expects a signed Assertion.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum WantAssertionsSigned {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for WantAssertionsSigned {
    fn from(value: &WantAssertionsSigned) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WantAssertionsSigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for WantAssertionsSigned {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WantAssertionsSigned {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WantAssertionsSigned {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WantAssertionsSigned {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether the identity provider expects a signed AuthnRequest.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Want AuthnRequests signed",
///  "description": "Indicates whether the identity provider expects a signed AuthnRequest.",
///  "type": "string",
///  "enum": [
///    "true",
///    "false",
///    ""
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
pub enum WantAuthnRequestsSigned {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for WantAuthnRequestsSigned {
    fn from(value: &WantAuthnRequestsSigned) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WantAuthnRequestsSigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::X => write!(f, ""),
        }
    }
}
impl ::std::str::FromStr for WantAuthnRequestsSigned {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WantAuthnRequestsSigned {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WantAuthnRequestsSigned {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WantAuthnRequestsSigned {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
