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
                        type_: Type::Object(7218972),
                        repeated: true,
                    },
                    2 => FieldType {
                        name: "dev-dependencies".to_string(),
                        type_: Type::Object(7218972),
                        repeated: true,
                    },
                    3 => FieldType {
                        name: "build-dependencies".to_string(),
                        type_: Type::Object(7218972),
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
            7218972 => ObjectType {
                name: "Dependency".to_string(),
                fields: btreemap! {
                    12093 => FieldType {
                        name: "name".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    912831 => FieldType {
                        name: "version".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    12987123 => FieldType {
                        name: "registry".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    2098713 => FieldType {
                        name: "git".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    912783 => FieldType {
                        name: "branch".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    341231 => FieldType {
                        name: "tag".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    123901 => FieldType {
                        name: "rev".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    198293 => FieldType {
                        name: "path".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                },
            },

            // Dockerfile
            213792873 => ObjectType {
                name: "Dockerfile".to_string(),
                fields: btreemap! {
                    2871232 => FieldType {
                        name: "commands".to_string(),
                        type_: Type::Object(28973111),
                        repeated: true,
                    },
                }
            },
            28973111 => ObjectType {
                name: "Dockerfile Command".to_string(),
                fields: btreemap! {
                    128371 => FieldType {
                        name: "from".to_string(),
                        type_: Type::Object(29187312),
                        repeated: false,
                    },
                    89723 => FieldType {
                        name: "run".to_string(),
                        type_: Type::Object(273819273),
                        repeated: true,
                    },
                    1987312 => FieldType {
                        name: "cmd".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    129837 => FieldType {
                        name: "label".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    129837 => FieldType {
                        name: "expose".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    129837 => FieldType {
                        name: "env".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    1927131 => FieldType {
                        name: "add".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    23812319 => FieldType {
                        name: "copy".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    6412128 => FieldType {
                        name: "entrypoint".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    21897312 => FieldType {
                        name: "volume".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    1298371 => FieldType {
                        name: "user".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    119283 => FieldType {
                        name: "workdir".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    1122811 => FieldType {
                        name: "arg".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    11929911 => FieldType {
                        name: "onbuild".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    9989231 => FieldType {
                        name: "stopsignal".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    1287311 => FieldType {
                        name: "healthcheck".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    199198 => FieldType {
                        name: "shell".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                }
            },

            // https://docs.docker.com/reference/dockerfile/#from
            29187312 => ObjectType {
                name: "Dockerfile FROM".to_string(),
                fields: btreemap! {
                    281731 => FieldType {
                        name: "image".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    976981231 => FieldType {
                        name: "tag".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    8912731 => FieldType {
                        name: "digest".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    8197129 => FieldType {
                        name: "as".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                }
            },

            // https://docs.docker.com/reference/dockerfile/#run
            273819273 => ObjectType {
                name: "Dockerfile RUN".to_string(),
                fields: btreemap! {
                    1279811 => FieldType {
                        name: "command".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    9002137 => FieldType {
                        name: "network".to_string(),
                        type_: Type::Object(87123897),
                        repeated: false,
                    },
                }
            },
            87123897 => ObjectType {
                name: "Dockerfile RUN network".to_string(),
                fields: btreemap! {
                    29187312 => FieldType {
                        name: "default".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    98123981 => FieldType {
                        name: "none".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    9128232 => FieldType {
                        name: "host".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                }
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
                    2287312 => FieldType {
                        name: "docker".to_string(),
                        type_: Type::Object(213792873),
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
