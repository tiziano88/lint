use crate::FieldType;
use crate::ObjectType;
use crate::Schema;
use crate::Type;
use maplit::btreemap;
use maplit::hashmap;

pub fn create_schema() -> Schema {
    Schema {
        root_object_type_id: 2325,
        object_types: hashmap! {
            // https://doc.rust-lang.org/cargo/reference/manifest.html
            893728943 => ObjectType {
                name: "CargoManifest".to_string(),
                fields: btreemap! {
                    0 => FieldType {
                        name: "package".to_string(),
                        type_: Type::Object(87839159),
                        repeated: false,
                    },
                    1 => FieldType {
                        name: "dependencies".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    2 => FieldType {
                        name: "dev-dependencies".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    3 => FieldType {
                        name: "build-dependencies".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    4 => FieldType {
                        name: "features".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    5 => FieldType {
                        name: "target".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    6 => FieldType {
                        name: "workspace".to_string(),
                        type_: Type::Object(893728943),
                        repeated: false,
                    },
                    7 => FieldType {
                        name: "profile".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    8 => FieldType {
                        name: "patch".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    9 => FieldType {
                        name: "replace".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    10 => FieldType {
                        name: "workspace-members".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    11 => FieldType {
                        name: "default-members".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    12 => FieldType {
                        name: "exclude".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    13 => FieldType {
                        name: "include".to_string(),
                        type_: Type::Object(893728943),
                        repeated: true,
                    },
                    14 => FieldType {
                        name: "metadata".to_string(),
                        type_: Type::Object(893728943),
                        repeated: false,
                    },
                },
            },
            // package
            87839159 => ObjectType {
                name: "Package".to_string(),
                fields: btreemap! {
                    0 => FieldType {
                        name: "name".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    1 => FieldType {
                        name: "version".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    2 => FieldType {
                        name: "authors".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    3 => FieldType {
                        name: "edition".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    4 => FieldType {
                        name: "build".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    5 => FieldType {
                        name: "links".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    6 => FieldType {
                        name: "exclude".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    7 => FieldType {
                        name: "include".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    8 => FieldType {
                        name: "publish".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    9 => FieldType {
                        name: "workspace".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    10 => FieldType {
                        name: "edition".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    11 => FieldType {
                        name: "metadata".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                },
            },

            27092 => ObjectType {
                name: "User".to_string(),
                fields: btreemap! {
                    0 => FieldType {
                        name: "name".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    1 => FieldType {
                        name: "age".to_string(),
                        type_: Type::Int,
                        repeated: false,
                    },
                    2 => FieldType {
                        name: "is_admin".to_string(),
                        type_: Type::Boolean,
                        repeated: false,
                    },
                    3 => FieldType {
                        name: "friends".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                },
            },
            2325 => ObjectType {
                name: "Post".to_string(),
                fields: btreemap! {
                    0 => FieldType {
                        name: "title".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    1 => FieldType {
                        name: "content".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    2 => FieldType {
                        name: "author".to_string(),
                        type_: Type::Object(27092),
                        repeated: true,
                    },
                    3 => FieldType {
                        name: "comments".to_string(),
                        type_: Type::Object(5528),
                        repeated: true,
                    },
                    4 => FieldType {
                        name: "cargo".to_string(),
                        type_: Type::Object(893728943),
                        repeated: false,
                    },
                },
            },
            5528 => ObjectType {
                name: "Comment".to_string(),
                fields: btreemap! {
                    0 => FieldType {
                        name: "content".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    1 => FieldType {
                        name: "things".to_string(),
                        type_: Type::Int,
                        repeated: false,
                    },
                },
            },
        },
    }
}
