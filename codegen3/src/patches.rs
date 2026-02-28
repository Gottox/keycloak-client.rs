use schemars::schema::RootSchema;
use syn::visit_mut::VisitMut;

/// K8s-managed type names that need the schemars transform attribute.
const K8S_TYPES: &[&str] = &[
    "RealmRepresentation",
    "GroupRepresentation",
    "ClientRepresentation",
    "UserRepresentation",
    "ClientScopeRepresentation",
    "ProtocolMapperRepresentation",
    "AuthenticationFlowRepresentation",
    "AuthenticatorConfigRepresentation",
    "RequiredActionProviderRepresentation",
    "OrganizationRepresentation",
    "ResourceRepresentation",
    "ScopeRepresentation",
    "PolicyRepresentation",
    "RoleRepresentation",
    "IdentityProviderRepresentation",
    "IdentityProviderMapperRepresentation",
    "ComponentExportRepresentation",
    "ApplicationRepresentation",
    "OAuthClientRepresentation",
];

/// Fix known bugs in the Keycloak OpenAPI schema before feeding to typify.
///
/// - Removes `oauth2DevicePollingInterval` and `oauth2DeviceCodeLifespan`
///   from `RealmRepresentation` (keycloak/keycloak#25563).
pub fn patch_schema(schema: &mut RootSchema) {
    if let Some(schemars::schema::Schema::Object(realm)) =
        schema.definitions.get_mut("RealmRepresentation")
    {
        if let Some(obj) = &mut realm.object {
            obj.properties.remove("oauth2DevicePollingInterval");
            obj.properties.remove("oauth2DeviceCodeLifespan");
        }
    }
}

/// Walk the generated AST and add `#[cfg_attr(feature = "k8s-schema", ...)]`
/// to each K8s-managed struct.
pub fn add_k8s_attrs(file: &mut syn::File) {
    let mut visitor = K8sAttrVisitor;
    visitor.visit_file_mut(file);
}

struct K8sAttrVisitor;

impl VisitMut for K8sAttrVisitor {
    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        let name = item.ident.to_string();
        if K8S_TYPES.contains(&name.as_str()) {
            let attr: syn::Attribute = syn::parse_quote! {
                #[cfg_attr(
                    feature = "k8s-schema",
                    schemars(transform = crate::k8s::remove_k8s_managed_fields)
                )]
            };
            item.attrs.push(attr);
        }
        // Continue visiting nested items
        syn::visit_mut::visit_item_struct_mut(self, item);
    }
}
