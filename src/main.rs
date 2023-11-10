use leptos::*;
use maplit::hashmap;
use std::{collections::HashMap, hash};

const ESCAPE_KEY: u32 = 27;
const ENTER_KEY: u32 = 13;

#[derive(Clone)]
struct Schema {
    root_object_type_id: ObjectTypeId,
    object_types: HashMap<ObjectTypeId, ObjectType>,
}

type ObjectTypeId = u32;
type FieldId = u32;

#[derive(Clone)]
enum Type {
    String,
    Int,
    Number,
    Boolean,
    // Array(Box<Type>),
    Object(ObjectTypeId),
}

impl Type {
    fn default_value(&self) -> Value {
        match self {
            Type::String => Value::String("".to_string()),
            Type::Int => Value::Int(0),
            Type::Number => Value::Number(0.0),
            Type::Boolean => Value::Boolean(false),
            Type::Object(object_type_id) => Value::Object(ObjectValue {
                object_type_id: *object_type_id,
                fields: hashmap! {},
            }),
        }
    }
}

#[derive(Clone)]
struct ObjectType {
    name: String,
    fields: HashMap<FieldId, FieldType>,
}

#[derive(Clone)]
struct FieldType {
    name: String,
    type_: Type,
    repeated: bool,
}

#[derive(Clone)]
enum Value {
    String(String),
    Int(i64),
    Number(f64),
    Boolean(bool),
    // Array(Vec<RwSignal<Value>>),
    Object(ObjectValue),
}

#[derive(Clone)]
struct ObjectValue {
    object_type_id: ObjectTypeId,
    fields: HashMap<FieldId, FieldValue>,
}

type FieldValue = RwSignal<Vec<RwSignal<Value>>>;

fn main() {
    mount_to_body(|| view! { <App /> })
}

fn create_schema() -> Schema {
    Schema {
        root_object_type_id: 1,
        object_types: hashmap! {
            0 => ObjectType {
                name: "User".to_string(),
                fields: hashmap! {
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
            1 => ObjectType {
                name: "Post".to_string(),
                fields: hashmap! {
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
                        type_: Type::Object(0),
                        repeated: false,
                    },
                    3 => FieldType {
                        name: "comments".to_string(),
                        type_: Type::Object(2),
                        repeated: true,
                    },
                },
            },
            2 => ObjectType {
                name: "Comment".to_string(),
                fields: hashmap! {
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

fn single_field_value(value: Value) -> FieldValue {
    create_rw_signal(vec![create_rw_signal(value)])
}

fn create_value() -> Value {
    Value::Object(ObjectValue {
        object_type_id: 1,
        fields: hashmap! {
            0 => single_field_value(Value::String("Hello, world!".to_string())),
            1 => single_field_value(Value::String("This is a post.".to_string())),
            2 => single_field_value(Value::Object(ObjectValue {
                    object_type_id: 0,
                    fields: hashmap! {
                        0 => single_field_value(Value::String("John Doe".to_string())),
                        1 => single_field_value(Value::Int(42)),
                        2 => single_field_value(Value::Boolean(false)),
                        3 => create_rw_signal(vec![
                                create_rw_signal(Value::String("Jane Doe".to_string())),
                                create_rw_signal(Value::String("Jack Doe".to_string())),
                            ]),
                    },
                })),
        },
    })
}

#[component]
fn App() -> impl IntoView {
    let (schema, set_schema) = create_signal(create_schema());
    let value = create_rw_signal(create_value());
    let root_type = Type::Object(schema.get_untracked().root_object_type_id);

    view! {
        <div>
            <ValueView schema=schema expected_type=root_type value=value />
        </div>
    }
}

#[component]
fn FieldView(
    schema: ReadSignal<Schema>,
    expected_type: FieldType,
    field: FieldValue,
) -> impl IntoView {
    view! {
        { move || {
                let default_value = expected_type.type_.default_value().clone();
                let plus_button = if expected_type.repeated || field.get().len() == 0 {
                    view! {
                        <div>
                        <button
                            on:click=move |_| {
                                let default_value = default_value.clone();
                                field.update(move |v| {
                                    v.push(create_rw_signal(default_value));
                                });
                            } >
                            +
                        </button>
                        </div>
                    }
                } else {
                    view! {
                        <div></div>
                    }
                };
                view! {
                    <div>
                        { expected_type.name.clone()}:
                                <ul>
                                <For
                                    each=move || field.get().clone().into_iter().enumerate()
                                    // a unique key for each item
                                    key=|(i,_)| *i
                                    // renders each item to a view
                                    children=move |(i,v)| {
                                        view! {
                                            <li>
                                                <button
                                                    on:click=move |_| {
                                                        field.update(|v| { v.remove(i); });
                                                    } >
                                                    x
                                                    </button>
                                                <ValueView schema=schema expected_type=Type::Boolean value=v />
                                            </li>
                                        }
                                    }
                                    />
                                    { plus_button }
                                </ul>
                    </div>
                }
            }
        }
    }
}

// Display a value.
#[component]
fn ValueView(
    schema: ReadSignal<Schema>,
    expected_type: Type,
    value: RwSignal<Value>,
) -> impl IntoView {
    view! {
        { move || match value.get() {
                Value::String(string) => view! { <span>
                <input
                    prop:value=string
                    on:keyup=move |ev| {
                        let key_code = ev.key_code();
                        if key_code == ENTER_KEY {
                            value.set(Value::String(event_target_value(&ev)));
                        }
                    } />
                </span> },
                Value::Int(int) => view! { <span>{int}</span> },
                Value::Number(number) => view! { <span>{number}</span> },
                Value::Boolean(boolean) => view! { <span>{boolean}</span> },
                Value::Object(object) => {
                    let object_type = schema.get().object_types.get(&object.object_type_id).cloned();
                    if object_type.is_none() {
                        return view! { <span>Unknown</span> };
                    }
                    let object_type = object_type.unwrap();
                    view! {
                        <span>
                            <ul>
                            <For
                                each=move || object_type.fields.clone().into_iter()
                                // a unique key for each item
                                key=|(field_id, _)| *field_id
                                // renders each item to a view
                                children=move |(field_id, field_type)| {
                                    let value = object.fields.get(&field_id).cloned().unwrap_or_default();
                                    view! {
                                        <span>
                                            <FieldView schema=schema expected_type=field_type.clone() field=value />
                                        </span>
                                    }
                                }
                                />
                            </ul>
                        </span>
                    }
                },
                _ => view! { <span>Unknown</span> },
            }
        }
    }
}
