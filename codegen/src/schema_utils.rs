use schemars::schema::{
    ArrayValidation, ObjectValidation, Schema, SingleOrVec,
};
use std::ops::{Deref, DerefMut};

fn cleanup_schema_array(array: &mut ArrayValidation) {
    match &mut array.items {
        Some(SingleOrVec::Single(schema)) => cleanup_schema(schema.deref_mut()),
        Some(SingleOrVec::Vec(schemas)) => {
            for schema in schemas {
                cleanup_schema(schema);
            }
        }
        _ => (),
    }
}

fn cleanup_schema_object(object: &mut ObjectValidation) {
    // We need to do that because Kubernetes has a very different
    // understanding of what additionalProperties means. We decide based
    // the types of the properties if we should use additionalProperties
    // and remove all properties or use properties and disallow additionalProperties.
    //
    // Ref: https://github.com/kubernetes/kubernetes/issues/94593
    let additional_properties_type =
        object.additional_properties.as_ref().and_then(|schema| {
            if let Schema::Object(object) = schema.deref() {
                object.instance_type.as_ref()
            } else {
                None
            }
        });

    let all_addition_type = object.properties.values().all(|schema| {
        if let Schema::Object(object) = schema {
            object.instance_type.as_ref() == additional_properties_type
        } else {
            false
        }
    });

    if all_addition_type {
        object.properties = Default::default();
    } else {
        object.additional_properties = None;
    }

    // TODO Remove this.
    // See: https://github.com/jirutka/keycloak-json-schema/issues/2
    object.properties.remove("bruteForceDetection");

    for schema in object.properties.values_mut() {
        cleanup_schema(schema);
    }
    if let Some(schema) = &mut object.additional_properties {
        cleanup_schema(schema);
    }
}

pub fn cleanup_schema(schema: &mut Schema) {
    let Schema::Object(object) = schema else {
        return;
    };

    if let Some(array) = &mut object.array {
        cleanup_schema_array(array);
    }

    if let Some(object) = &mut object.object {
        cleanup_schema_object(object);
    }
}
