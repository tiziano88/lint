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
            298732 => ObjectType {
                name: "FlatArticle".to_string(),
                fields: btreemap! {
                    37438 => FieldType {
                        name: "blocks".to_string(),
                        type_: Type::Object(28398),
                        repeated: true,
                    },
                },
            },
            28398 => ObjectType {
                name: "FlatArticleBlock".to_string(),
                fields: btreemap! {
                    29382 => FieldType {
                        name: "h1".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    129382 => FieldType {
                        name: "h2".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    093483 => FieldType {
                        name: "h3".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    238239 => FieldType {
                        name: "paragrah".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    9834734 => FieldType {
                        name: "quote".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    389734 => FieldType {
                        name: "code".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    834734 => FieldType {
                        name: "ordered list".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                    756347 => FieldType {
                        name: "unordered list".to_string(),
                        type_: Type::String,
                        repeated: true,
                    },
                },
            },

            9823923 => ObjectType {
                name: "TreeArticle".to_string(),
                fields: btreemap! {
                    34837 => FieldType {
                        name: "sections".to_string(),
                        type_: Type::Object(8734289),
                        repeated: false,
                    },
                },
            },

            8734289 => ObjectType {
                name: "TreeArticleSection".to_string(),
                fields: btreemap! {
                    21837 => FieldType {
                        name: "title".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    298734 => FieldType {
                        name: "blocks".to_string(),
                        type_: Type::Object(93847373),
                        repeated: true,
                    },
                },
            },

            93847373 => ObjectType {
                name: "TreeArticleBlock".to_string(),
                fields: btreemap! {
                    387439 => FieldType {
                        name: "text".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    38743 => FieldType {
                        name: "quote".to_string(),
                        type_: Type::String,
                        repeated: false,
                    },
                    39843 => FieldType {
                        name: "code".to_string(),
                        type_: Type::String,
                        repeated: false,
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
                    329873 => FieldType {
                        name: "flat_article".to_string(),
                        type_: Type::Object(298732),
                        repeated: false,
                    },
                    734837 => FieldType {
                        name: "tree_article".to_string(),
                        type_: Type::Object(9823923),
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
