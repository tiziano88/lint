use leptos::*;
use maplit::{btreemap, hashmap};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display, Formatter},
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

#[derive(Clone, Debug)]
enum Value {
    String(String),
    Int(i64),
    Number(f64),
    Boolean(bool),
    // Array(Vec<RwSignal<Value>>),
    Object(ObjectValue),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(string) => write!(f, "{}", string),
            Value::Int(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Object(v) => write!(f, "<OBJECT>"),
        }
    }
}

impl Value {
    fn parse(type_: Type, s: &str) -> Option<Value> {
        match type_ {
            Type::String => Some(Value::String(s.to_string())),
            Type::Int => s.parse::<i64>().map(Value::Int).ok(),
            Type::Number => s.parse::<f64>().map(Value::Number).ok(),
            Type::Boolean => s.parse::<bool>().map(Value::Boolean).ok(),
            Type::Object(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
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
                        repeated: true,
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

fn parent(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    let mut path = path.clone();
    path.pop();
    path
}

// Traverse the value to find the child at the given path.
fn child(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    let mut path = path.clone();
    let mut value = root_value.clone();
    for selector in path.iter() {
        match value {
            Value::Object(object) => {
                let field = object.fields.get(&selector.field_id).unwrap();
                value = field.get()[selector.index].get().clone();
            }
            _ => {
                logging::log!("not an object");
                break;
            }
        }
    }
    match value {
        Value::Object(o) => {
            let object_type = schema.object_types.get(&o.object_type_id).unwrap();
            for (field_id, field_type) in object_type.fields.iter() {
                let field = o.fields.get(field_id).unwrap();
                if field_type.type_.is_primitive() {
                    path.push(Selector {
                        field_id: *field_id,
                        index: 0,
                    });
                    break;
                }
            }
        }
        _ => {}
    }
    path
}

fn prev(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    // TODO
    return path.clone();
}

fn next(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    // TODO
    return path.clone();
}

fn ancestor_with_next_child(
    schema: &Schema,
    root_value: &Value,
    path: &Path,
    current_child: &Path,
) -> Option<Path> {
    // TODO
    return None;
}

fn next_leaf(
    schema: &Schema,
    root_value: &Value,
    path: &Path,
    current_child: &Path,
) -> Option<Path> {
    // TODO
    return None;
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
            sel: {move || format_path(&selected_path.get())}
            <ObjectView schema=schema value=value path=vec![] selected=selected_path/>
            <button on:click=move |_| {
                selected_path.set(parent(&schema.get(), &value.get(), &selected_path.get()));
            }>Parent</button>
            <button on:click=move |_| {
                selected_path.set(child(&schema.get(), &value.get(), &selected_path.get()));
            }>Child</button>
            <button on:click=move |_| {
                selected_path.set(prev(&schema.get(), &value.get(), &selected_path.get()));
            }>Prev</button>
            <button on:click=move |_| {
                selected_path.set(next(&schema.get(), &value.get(), &selected_path.get()));
            }>Next</button>
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
    let object = match value.get_untracked() {
        Value::Object(v) => v,
        _ => panic!("expected object value"),
    };
    let object_type = schema
        .get_untracked()
        .object_types
        .get(&object.object_type_id)
        .cloned();
    if object_type.is_none() {
        return view! { <div>Unknown</div> };
    }
    let object_type = object_type.unwrap();
    let path1 = path.clone();
    let path2 = path.clone();
    let s = create_memo(move |_| path1 == selected.get());
    view! {
        <div>
            <div
                class="block"
                class:selected=s
                on:click=move |ev| {
                    ev.stop_propagation();
                    selected.set(path.clone());
                }
            >

                <ul class="border border-gray-300 block p-2">
                    <For
                        each=move || object_type.fields.clone().into_iter()
                        // a unique key for each item
                        key=|(field_id, _)| *field_id
                        // renders each item to a view
                        children=move |(field_id, field_type)| {
                            let value = object.fields.get(&field_id).cloned().unwrap_or_default();
                            let more_than_one_field_value = value.get().len() > 1;
                            let path2 = path2.clone();
                            let default_value = field_type.type_.default_value().clone();
                            let add_button = if field_type.repeated || value.get().len() == 0 {
                                view! {
                                    <div class="inline">
                                        <button
                                            class="button"
                                            on:click=move |_| {
                                                let default_value = default_value.clone();
                                                value
                                                    .update(move |v| {
                                                        v.push(create_rw_signal(default_value));
                                                    });
                                            }
                                        >

                                            +
                                        </button>
                                    </div>
                                }
                            } else {
                                view! { <div></div> }
                            };
                            let field_type = field_type.clone();
                            let field_type2 = field_type.clone();
                            let all_field_values = view! {
                                <For
                                    each=move || value.get().clone().into_iter().enumerate()
                                    // a unique key for each item
                                    key=|(i, _)| *i
                                    // renders each item to a view
                                    children=move |(i, v)| {
                                        let new_path = {
                                            let mut new_path = path2.clone();
                                            new_path.push(Selector { field_id, index: i });
                                            new_path
                                        };
                                        let view = match field_type2.type_ {
                                            Type::Object(_) => {
                                                view! {
                                                    <div>
                                                        <ObjectView
                                                            schema=schema
                                                            value=v
                                                            path=new_path
                                                            selected=selected
                                                        />
                                                    </div>
                                                }
                                            }
                                            _ => {
                                                view! {
                                                    <div>
                                                        <ValueView
                                                            expected_type=field_type2.clone()
                                                            value=v
                                                            path=new_path
                                                            selected=selected
                                                        />
                                                    </div>
                                                }
                                            }
                                        };
                                        if more_than_one_field_value {
                                            view! {
                                                <div>
                                                    <li>
                                                        <div class="flex">
                                                            <button
                                                                class="button"
                                                                on:click=move |_| {
                                                                    value
                                                                        .update(|v| {
                                                                            v.remove(i);
                                                                        });
                                                                }
                                                            >

                                                                x
                                                            </button>
                                                            {view}
                                                        </div>
                                                    </li>
                                                </div>
                                            }
                                        } else {
                                            view! { <div>{view}</div> }
                                        }
                                    }
                                />
                            };
                            let field_type2 = field_type.clone();
                            view! {
                                // a unique key for each item
                                // renders each item to a view

                                <li class="list-disc pl-2">
                                    {if more_than_one_field_value {
                                        view! {
                                            <div>
                                                <div class="inline-block">{field_type2.clone().name} :</div>
                                                <ol class="list-decimal" start=0>
                                                    {all_field_values}
                                                </ol>
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div>
                                                <div class="inline-block">{field_type2.clone().name} :</div>
                                                {all_field_values}
                                            </div>
                                        }
                                    }}
                                    {add_button}
                                </li>
                            }
                        }
                    />

                </ul>
            </div>
        </div>
    }
}

#[component]
fn ValueView(
    expected_type: FieldType,
    value: RwSignal<Value>,
    path: Path,
    selected: RwSignal<Path>,
) -> impl IntoView {
    if !expected_type.type_.is_primitive() {
        panic!("expected primitive type")
    }
    let text_box = view! {
        <div>
            <input
                class="border border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 block"
                type="text"
                prop:value=move || { value.get().to_string() }
                on:input=move |ev| {
                    let v = event_target_value(&ev);
                    let parsed = Value::parse(expected_type.type_.clone(), &v);
                    logging::log!("parsing {} as {:?} -> {:?}", v, expected_type.type_, parsed);
                    if let Some(parsed) = parsed {
                        value.set(parsed);
                    }
                }
            />

        </div>
    };
    let path1 = path.clone();
    let s = create_memo(move |_| path1 == selected.get());
    view! {
        <div
            class="block"
            class:selected=s
            on:click=move |ev| {
                ev.stop_propagation();
                selected.set(path.clone());
            }
        >

            {text_box}
        </div>
    }
}
