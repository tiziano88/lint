use leptos::*;
use maplit::{btreemap, hashmap};
use std::{
    collections::{BTreeMap, HashMap},
    hash,
};

const ESCAPE_KEY: u32 = 27;
const ENTER_KEY: u32 = 13;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Selector {
    field_id: FieldId,
    index: usize,
}

type Path = Vec<Selector>;

fn format_path(p: &Path) -> String {
    p.iter()
        .map(|selector| format!("{}[{}]", selector.field_id, selector.index))
        .collect::<Vec<_>>()
        .join(".")
}

#[derive(Clone)]
struct Schema {
    root_object_type_id: ObjectTypeId,
    object_types: HashMap<ObjectTypeId, ObjectType>,
}

type ObjectTypeId = u32;
type FieldId = u32;

#[derive(Clone, Debug)]
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

    fn is_primitive(&self) -> bool {
        match self {
            Type::String => true,
            Type::Int => true,
            Type::Number => true,
            Type::Boolean => true,
            Type::Object(_) => false,
        }
    }
}

#[derive(Clone)]
struct ObjectType {
    name: String,
    fields: BTreeMap<FieldId, FieldType>,
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
    mount_to_body(|| view! { <App/> })
}

fn create_schema() -> Schema {
    Schema {
        root_object_type_id: 2325,
        object_types: hashmap! {
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
                        repeated: false,
                    },
                    3 => FieldType {
                        name: "comments".to_string(),
                        type_: Type::Object(5528),
                        repeated: true,
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

fn single_field_value(value: Value) -> FieldValue {
    create_rw_signal(vec![create_rw_signal(value)])
}

fn create_value() -> Value {
    Value::Object(ObjectValue {
        object_type_id: 2325,
        fields: hashmap! {
            0 => single_field_value(Value::String("Hello, world!".to_string())),
            1 => single_field_value(Value::String("This is a post.".to_string())),
            2 => single_field_value(Value::Object(ObjectValue {
                    object_type_id: 27092,
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
    let selected_path = create_rw_signal(Path::default());

    let selected_element = create_memo(move |_| format_path(&selected_path.get()));

    view! {
        <div>
            sel: { move || format_path(&selected_path.get()) }
            <ObjectView
                schema=schema
                value=value
                path=vec![]
                selected=selected_path
            />
            <button on:click=move |_| {
                selected_path.set(vec![Selector { field_id: 1, index: 0 }]);
            }>Reset</button>
        </div>
    }
}

#[component]
fn ObjectView(
    schema: ReadSignal<Schema>,
    value: RwSignal<Value>,
    path: Path,
    selected: RwSignal<Path>,
) -> impl IntoView {
    let object = match value.get() {
        Value::Object(v) => v,
        _ => panic!("expected object value"),
    };
    let object_type = schema
        .get()
        .object_types
        .get(&object.object_type_id)
        .cloned();
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
                    let value = object
                        .fields
                        .get(&field_id)
                        .cloned()
                        .unwrap_or_default();
                    let more_than_one_field_value = value.get().len() > 1;
                    let path = path.clone();


                    // let expected_type = expected_type.clone();
                    let default_value = field_type.type_.default_value().clone();
                    let plus_button = if field_type.repeated || value.get().len() == 0 {
                        view! {
                            <div>
                                <button on:click=move |_| {
                                    let default_value = default_value.clone();
                                    value
                                        .update(move |v| {
                                            v.push(create_rw_signal(default_value));
                                        });
                                }>+</button>
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    };

                    let all_field_values = view! {
                        <For
                            each=move || value.get().clone().into_iter().enumerate()
                            // a unique key for each item
                            key=|(i, _)| *i
                            // renders each item to a view
                            children=move |(i, v)| {
                                let new_path = {
                                    let mut new_path = path.clone();
                                    new_path.push(Selector { field_id, index: i });
                                    new_path
                                };
                                let field_type = field_type.clone();
                                let field_type2 = field_type.clone();


                                let x_button = view! {
                                    <button on:click=move |_| {
                                        value
                                            .update(|v| {
                                                v.remove(i);
                                            });
                                    }>x</button>
                                };

                                let count_msg = " #".to_owned() + &i.to_string();
                                let field_name_with_count = field_type.clone().name + if field_type.repeated { &count_msg } else { "" } + ": ";

                                let view = match field_type.type_ {
                                    Type::Object(_) => view! {
                                        <span>
                                            {field_name_with_count}
                                            { x_button }
                                            <ObjectView
                                                schema=schema
                                                value=v
                                                path=new_path
                                                selected=selected
                                            />
                                        </span>
                                    },
                                    _ => view! {
                                        <span>
                                            {field_name_with_count}
                                            { x_button }
                                            <ValueView
                                                expected_type=field_type2
                                                value=v
                                            />
                                        </span>
                                    },
                                };

                                if more_than_one_field_value {
                                    view! {
                                        <span><li>{view}</li></span>
                                    }
                                } else {
                                    view! {
                                        <span>{view}</span>
                                    }
                                }
                            }
                        />
                    };
                    view! {
                        <li>
                            {
                                if more_than_one_field_value {
                                    view! {
                                        <span>
                                            <ul>
                                                {all_field_values}
                                            </ul>
                                        </span>
                                    }
                                } else {
                                    view! {
                                        <span>{all_field_values}</span>
                                    }
                                }
                            }
                            { plus_button }
                        </li>
                    }
                }
            />
        </ul>
    </span>
    }
}

#[component]
fn ValueView(
    expected_type: FieldType,
    value: RwSignal<Value>,
) -> impl IntoView {
    view! {
        <span>
            <input
                type="text"
                prop:value=move || { pretty_print(value.get()) }
                on:input=move |ev| {
                    let v = event_target_value(&ev);
                    logging::log!("v {:?}", v);
                    // logging::log!("expected_type {:?}", expected_type);
                    match expected_type.type_ {
                        Type::String => value.set(Value::String(v)),
                        Type::Int => {
                            let i_parsed = v.parse::<i64>();
                            logging::log!("i_parsed {:?}", i_parsed);
                            value.set(Value::Int(i_parsed.unwrap_or_default()))
                        },
                        _ => {},
                    };
                }
            />
        </span>
    }
}

fn pretty_print(value: Value) -> String {
    match value {
        Value::String(string) => string,
        Value::Int(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::Boolean(v) => v.to_string(),
        Value::Object(v) => "invalid value".to_string(),
    }
}
