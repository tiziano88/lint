use core::panic;
use leptos::*;
use maplit::btreemap;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display, Formatter},
    process::id,
    sync::Arc,
};

mod schema;
mod storage;

use schema::*;
use storage::*;

const ESCAPE_KEY: u32 = 27;
const ENTER_KEY: u32 = 13;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Selector {
    field_id: ID,
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
    root_object_type_id: ID,
    object_types: HashMap<ID, ObjectType>,
}

type ID = u32;

fn new_id() -> ID {
    rand::random()
}

#[derive(Clone, Debug, PartialEq)]
enum Type {
    String,
    Int,
    Number,
    Boolean,
    // Array(Box<Type>),
    Object(ID),
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
struct Node {
    id: ID,
    value: Value,
}

impl Node {
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn deserialize(s: &str) -> Option<Node> {
        serde_json::from_str(s).ok()
    }
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
                fields: btreemap! {},
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

#[derive(Debug, Clone, PartialEq)]
struct ObjectType {
    name: String,
    fields: BTreeMap<ID, FieldType>,
}

#[derive(Debug, Clone, PartialEq)]
struct FieldType {
    name: String,
    type_: Type,
    repeated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
enum Value {
    #[default]
    Empty,
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
            Value::Empty => write!(f, "<EMPTY>"),
            Value::String(string) => write!(f, "{}", string),
            Value::Int(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Object(_v) => write!(f, "<OBJECT>"),
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ObjectValue {
    object_type_id: ID,
    fields: BTreeMap<ID, Vec<D>>,
}

impl ObjectValue {
    fn append(&mut self, field_id: ID, value: D) {
        let field = self.fields.entry(field_id).or_default();
        field.push(value);
    }
    fn delete(&mut self, field_id: ID, index: usize) {
        self.fields.get_mut(&field_id).map(|v| v.remove(index));
    }
    fn set(&mut self, field_id: ID, index: usize, value: D) {
        let field = self.fields.entry(field_id).or_default();
        field.get_mut(index).map(|v| *v = value);
    }
    fn get(&mut self, field_id: ID, index: usize) -> Option<&D> {
        self.fields.get(&field_id).and_then(|v| v.get(index))
    }
}

type FieldValue = RwSignal<Vec<RwSignal<Value>>>;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

fn single_field_value(value: Value) -> FieldValue {
    create_rw_signal(vec![create_rw_signal(value)])
}

fn create_value() -> Value {
    Value::Object(ObjectValue {
        object_type_id: 2325,
        fields: btreemap! {
            0 => vec![],
            1 => vec![],
            2 => vec![],
        },
    })
}

#[allow(unused)]
fn parent(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    let mut path = path.clone();
    path.pop();
    path
}

// Traverse the value to find the child at the given path.
#[allow(unused)]
fn child(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    todo!()
    // let value = find_value(root_value, path).unwrap();
    // let mut path = path.clone();
    // match value {
    //     Value::Object(o) => {
    //         let object_type = schema.object_types.get(&o.object_type_id).unwrap();
    //         for (field_id, field_type) in object_type.fields.iter() {
    //             let field = o.fields.get(field_id).unwrap();
    //             if field_type.type_.is_primitive() {
    //                 path.push(Selector {
    //                     field_id: *field_id,
    //                     index: 0,
    //                 });
    //                 break;
    //             }
    //         }
    //     }
    //     _ => {}
    // }
    // path
}

#[allow(unused)]
fn prev(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    // TODO
    return path.clone();
}

#[allow(unused)]
fn next(schema: &Schema, root_value: &Value, path: &Path) -> Path {
    todo!()
    // let starting_ancestor = match find_value(root_value, path).unwrap() {
    //     Value::Object(_) => Some(path.clone()),
    //     _ => ancestor_with_next_child(schema, root_value, path),
    // };
    // logging::log!("starting_ancestor {:?}", starting_ancestor);
    // match starting_ancestor {
    //     Some(ancestor) => first_leaf(schema, root_value, &ancestor)
    //         .get_or_insert(path.clone())
    //         .to_vec(),
    //     None => path.clone(),
    // }
}

#[allow(unused)]
fn ancestor_with_next_child(schema: &Schema, root_value: &Value, path: &Path) -> Option<Path> {
    todo!()
    // let parent_path = parent(schema, root_value, path);
    // let parent_value = find_value(root_value, &parent_path).unwrap();
    // match parent_value {
    //     Value::Object(parent_object) => {
    //         let mut parent_type_fields = schema.object_types[&parent_object.object_type_id]
    //             .fields
    //             .clone();
    //         let current_path_field_id = match path.last() {
    //             Some(selector) => selector.field_id,
    //             None => panic!("no values"),
    //         };
    //         let subsequent_parent_type_fields =
    //             parent_type_fields.split_off(&current_path_field_id);
    //         let mut iter = subsequent_parent_type_fields.iter();
    //         let (current_field_id, current_field_type) = iter.next().unwrap();
    //         if current_field_type.repeated {
    //             let field_entries_size = parent_object.fields[current_field_id].len();
    //             let current_field_index = path.last().unwrap().index;
    //             if current_field_index < field_entries_size - 1 {
    //                 let mut next_leaf_path = parent_path.clone();
    //                 next_leaf_path.push(Selector {
    //                     field_id: *current_field_id,
    //                     index: current_field_index + 1,
    //                 });
    //                 return Some(next_leaf_path);
    //             }
    //         }
    //         for (field_id, field_type) in iter {
    //             if parent_object.fields.contains_key(field_id) {
    //                 let mut next_leaf_path = parent_path.clone();
    //                 next_leaf_path.push(Selector {
    //                     field_id: *field_id,
    //                     index: 0,
    //                 });
    //                 return Some(next_leaf_path);
    //             }
    //         }
    //     }
    //     _ => panic!("not an object"),
    // }
    // if parent_path.len() == 0 {
    //     return None;
    // }
    // return ancestor_with_next_child(schema, root_value, &parent_path);
}

fn first_leaf(schema: &Schema, root_digest: &D, path: &Path) -> Option<Path> {
    let digest = find_value(root_digest, path).unwrap();
    let value = get_item(&digest).get_untracked().unwrap().value.clone();
    let mut first_leaf_path = path.clone();
    match value {
        Value::Object(object_value) => {
            let type_fields = schema.object_types[&object_value.object_type_id]
                .fields
                .clone();
            match type_fields.first_key_value() {
                Some((field_id, _field_type)) => {
                    first_leaf_path.push(Selector {
                        field_id: *field_id,
                        index: 0,
                    });
                    first_leaf(schema, root_digest, &first_leaf_path)
                }
                None => panic!("no values"),
            }
        }
        _ => Some(first_leaf_path),
    }
}

fn find_value(root_digest: &D, path: &Path) -> Option<D> {
    if path.is_empty() {
        return Some(root_digest.clone());
    } else {
        let node = get_item(root_digest).get_untracked().unwrap();
        let object = match node.value {
            Value::Object(v) => v,
            _ => panic!("expected object value"),
        };
        let selector = path.first().unwrap();
        let field = object.fields.get(&selector.field_id).unwrap();
        let next_path = path[1..].to_vec();
        let next_selector = next_path.first().unwrap();
        let next_index = next_selector.index;
        let next_digest = field.get(next_index).unwrap();
        return find_value(&next_digest, &next_path);
    }
}

trait HasDigest {
    type Digest;
    fn digest(&self) -> Self::Digest;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct D {
    sha2_256: [u8; 32],
}

impl D {
    pub fn to_hex(&self) -> String {
        format!("sha2-256:{}", hex::encode(self.sha2_256))
    }

    pub fn from_hex(s: &str) -> Self {
        let sha2_256 = hex::decode(&s[9..]).unwrap();
        D {
            sha2_256: sha2_256.try_into().unwrap(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sha2_256.iter().all(|&b| b == 0)
    }
}

// test for from_hex
#[test]
fn test_d_from_hex() {
    let d =
        D::from_hex("sha2-256:883ba22861ceac0617e6d08c25d6c6868a5cc1757f44f41b70845d23b667323e");
}

impl HasDigest for Node {
    type Digest = D;
    fn digest(&self) -> D {
        let json = self.serialize();
        let sha2_256 = sha2::Sha256::digest(json.as_bytes());
        D {
            sha2_256: sha2_256.into(),
        }
    }
}

#[derive(Clone, Debug)]
enum Position {
    First,
    Last,
    Before(usize),
    After(usize),
}

#[derive(Clone, Debug)]
enum Action {
    Noop,
    Update(Path, Value),
    // Path of the parent, ID of the field.
    Append {
        path: Path,
        field_id: ID,
        position: Position,
        value: Value,
    },
    Delete {
        path: Path,
    },
}

#[derive(Default)]
struct State {
    history: Vec<D>,
}

fn update_node(base: &D, path: &Path, value: Value) -> D {
    update_node_value(base, path, |_| value)
}

fn update_node_value<F: FnOnce(Value) -> Value>(base: &D, path: &Path, update_fn: F) -> D {
    logging::log!("update_node {:?} -> (fn)", path);
    if path.is_empty() {
        let mut node = get_item(base).get_untracked().unwrap();
        node.value = update_fn(node.value);
        set_item(&node)
    } else {
        let mut node = get_item(base).get_untracked().unwrap();
        logging::log!("node {:?}", node);
        let mut object = match node.value {
            Value::Object(v) => v,
            _ => panic!("expected object value"),
        };
        logging::log!("object {:?}", object);
        let selector = path.first().unwrap();
        logging::log!("selector {:?}", selector);
        let child = object.get(selector.field_id, selector.index).unwrap();
        let new_next_digest = update_node_value(child, &path[1..].to_vec(), update_fn);
        object.set(selector.field_id, selector.index, new_next_digest);
        node.value = Value::Object(object);
        set_item(&node)
    }
}

#[component]
fn List() -> impl IntoView {
    let (v, set_v) = create_signal(vec![1, 2, 3]);
    let (other, set_other) = create_signal(123);
    view! {
        <div>
            {move || other.get()}
            <For
                each=move || 0..v.with(Vec::len)
                key=|i| i.clone()
                children=move |i| {
                    let vv = create_memo(move |_| { v.with(|v| v[i].clone()) });
                    view! {
                        <div>
                            <Thing v=vv/>
                            {move || other.get()}
                        </div>
                    }
                }
            />
            <button
                class="block"
                on:click=move |_| {
                    set_v.update(|x| x[1] += 1);
                }
            >

                Inc
            </button>
            <button on:click=move |_| {
                set_other.update(|other| *other += 1);
            }>other</button>
        </div>
    }
}

#[component]
fn Thing(#[prop(into)] v: Memo<i32>) -> impl IntoView {
    logging::log!("Thing {:?}", v.get_untracked());
    view! { <div>{move || v.get()}</div> }
}

#[component]
fn App() -> impl IntoView {
    logging::log!("rendering App");
    let (schema, _set_schema) = create_signal(create_schema());
    let value = create_rw_signal(create_value());
    let _root_type = Type::Object(schema.get_untracked().root_object_type_id);

    let (debug, _set_debug) = create_signal(false);

    let selected_path = create_rw_signal(Path::default());

    let (history, _set_history) = create_signal(Vec::<D>::new());

    let store = Arc::new(LocalStorage::<Node, D>::new());

    let node = Node {
        id: 1,
        value: create_value(),
    };
    let d = set_item(&node);

    // let storage = window().local_storage().unwrap().unwrap();
    // storage.set_item("c", "v").unwrap();
    // logging::log!("storage {}", storage.get_item("c").unwrap().unwrap());
    // let d = store.put(Node {
    //     id: 1,
    //     value: value.get_untracked(),
    // });
    // logging::log!("node {:?}", store.get(&d));

    let selected_element = create_memo(move |_| format_path(&selected_path.get()));

    // TODO: derived signals have different types.
    // let root_digest = Signal::derive(move || history.get().last().cloned().unwrap());

    let (root_digest, set_root_digest) = create_signal(d);
    let root_digest_memo = create_memo(move |_| root_digest.get());

    create_effect(move |_| {
        let d = get_root();
        logging::log!("root {:?}", d);
        set_root_digest(d);
    });

    let on_action = move |action| {
        logging::log!("action {:?}", action);
        match action {
            Action::Noop => {}
            Action::Update(path, value) => {
                let new_d = update_node(&root_digest(), &path, value);
                set_root(&new_d);
                set_root_digest(new_d);
            }
            Action::Append {
                path,
                field_id,
                position,
                value,
            } => {
                // TODO: handle position
                match position {
                    Position::Last => {}
                    _ => {}
                };
                let new_d = update_node_value(&root_digest(), &path, |v| match v {
                    Value::Object(mut object) => {
                        object.append(
                            field_id,
                            set_item(&Node {
                                id: new_id(),
                                value,
                            }),
                        );
                        Value::Object(object)
                    }
                    _ => panic!("expected object value"),
                });
                set_root(&new_d);
                set_root_digest(new_d);
            }
            Action::Delete { path } => {
                let (parent_path, selector) = path.split_at(path.len() - 1);
                let selector = selector.first().unwrap();
                let new_d = update_node_value(&root_digest(), &parent_path.to_vec(), |v| match v {
                    Value::Object(mut object) => {
                        object.delete(selector.field_id, selector.index);
                        Value::Object(object)
                    }
                    _ => panic!("expected object value"),
                });
                set_root(&new_d);
                set_root_digest(new_d);
            }
        }
    };

    view! {
        <div class="">
            // <List/>
            <div>sel: {move || format_path(&selected_path.get())}</div>
            <div>root_digest: {move || root_digest.get().to_hex()}</div>
            <div>hist: {move || format!("{:?}", history.get())}</div>
            <ObjectView
                schema=schema
                digest=root_digest_memo
                path=vec![]
                selected=selected_path
                on_action=on_action
                debug=debug
            />
            <button
                class="button"
                on:click=move |_| {
                    selected_path.set(parent(&schema.get(), &value.get(), &selected_path.get()));
                }
            >

                Parent
            </button>
            <button
                class="button"
                on:click=move |_| {
                    selected_path.set(child(&schema.get(), &value.get(), &selected_path.get()));
                }
            >

                Child
            </button>
            <button
                class="button"
                on:click=move |_| {
                    selected_path.set(prev(&schema.get(), &value.get(), &selected_path.get()));
                }
            >

                Prev
            </button>
            <button
                class="button"
                on:click=move |_| {
                    selected_path.set(next(&schema.get(), &value.get(), &selected_path.get()));
                }
            >

                Next
            </button>
        </div>
    }
}

#[component]
fn ObjectView(
    schema: ReadSignal<Schema>,
    digest: Memo<D>,
    #[prop(into)] on_action: Callback<Action>,
    path: Path,
    selected: RwSignal<Path>,
    debug: ReadSignal<bool>,
) -> impl IntoView {
    logging::log!("rendering ObjectView {:?}", path);
    let node = create_memo(move |_| get_item_untracked(&digest.get()));
    let value = create_memo(move |_| node.get().unwrap().value.clone());
    let path1 = path.clone();
    let path2 = path.clone();
    let path3 = path.clone();
    let path4 = path.clone();
    let path5 = path.clone();
    let path6 = path.clone();
    let path7 = path.clone();
    let path8 = path.clone();
    let s = create_memo(move |_| path1 == selected.get());
    fn change_value() {}
    let view_object = move |id: Memo<ID>, v: Memo<ObjectValue>| -> HtmlElement<html::Div> {
        logging::log!("view_object {:?} {:?}", path2, v);
        let object_type = move || {
            schema
                .get()
                .object_types
                .get(&v().object_type_id)
                .unwrap()
                .clone()
        };
        let v = v.clone();
        let v1 = v.clone();
        let v2 = v.clone();
        let v3 = v.clone();
        let path4 = path4.clone();
        let field_ids = move || {
            object_type()
                .clone()
                .fields
                .clone()
                .keys()
                .cloned()
                .collect::<Vec<_>>()
        };
        view! {
            <div class="rounded border-solid border-2 border-blue divide-y">
                <div class="">
                    <div class="bg-blue flex p-2 space-x-2">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-6 h-6"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9"
                            ></path>
                        </svg>
                        <div class="">{move || object_type().name}</div>
                        <button class="cursor-pointer" title="focus on this element">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="w-6 h-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M7.5 3.75H6A2.25 2.25 0 0 0 3.75 6v1.5M16.5 3.75H18A2.25 2.25 0 0 1 20.25 6v1.5m0 9V18A2.25 2.25 0 0 1 18 20.25h-1.5m-9 0H6A2.25 2.25 0 0 1 3.75 18v-1.5M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                                ></path>
                            </svg>
                        </button>
                    </div>
                </div>
                // Iterate over the fields of the object type.
                <For
                    // each=move || object_type.fields.clone().into_iter()
                    // each=move || vec![(0, FieldType {
                    // name: "name".to_string(),
                    // type_: Type::String,
                    // repeated: false,
                    // })]
                    each=move || field_ids()
                    // a unique key for each item
                    key=|field_id| *field_id
                    // renders each item to a view
                    children=move |field_id| {
                        let field_type = move || {
                            object_type().fields.get(&field_id).unwrap().clone()
                        };
                        let v3 = v3.clone();
                        let fields = move || v().fields.clone();
                        let fields1 = move || v().fields.clone();
                        let field_type = field_type.clone();
                        let field_type1 = field_type.clone();
                        let path4 = path4.clone();
                        let path5 = path4.clone();
                        view! {
                            // logging::log!("for field_id {:?}", field_id);
                            // let v1 = v.clone();
                            // let v2 = v2.clone();
                            // let fields2 = v.fields.clone();
                            // let it: Vec<(usize, D)> = fields
                            // .get(&field_id)
                            // .cloned()
                            // .unwrap_or_default()
                            // .into_iter()
                            // .enumerate()
                            // .collect();
                            <div class="p-2">
                                {move || field_type().name}
                                <Show when=move || debug()>"(#" {field_id} ")"</Show>
                                // Iterate over the field values.
                                <For
                                    each=move || {
                                        0..fields().get(&field_id).map(Vec::len).unwrap_or_default()
                                    }

                                    key=|i| i.clone()
                                    // key=|(i, _)| i.clone()
                                    children=move |index| {
                                        let fields1 = fields1.clone();
                                        let read_d = create_memo(move |_| {
                                            fields1()
                                                .get(&field_id)
                                                .cloned()
                                                .unwrap_or_default()[index]
                                                .clone()
                                        });
                                        let field_type = field_type1.clone();
                                        let v3 = v3.clone();
                                        let new_path = {
                                            let mut new_path = path4.clone();
                                            new_path.push(Selector { field_id, index });
                                            new_path
                                        };
                                        let new_path_2 = new_path.clone();
                                        let new_path_3 = new_path.clone();
                                        view! {
                                            <div class="mx-4 my-2 flex">
                                                <Show when=move || debug()>{format_path(&new_path_2)}</Show>
                                                <div
                                                    class="cursor-pointer text-red"
                                                    on:click=move |_| {
                                                        on_action(Action::Delete {
                                                            path: new_path_3.clone(),
                                                        })
                                                    }
                                                >

                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        fill="none"
                                                        viewBox="0 0 24 24"
                                                        stroke-width="1.5"
                                                        stroke="currentColor"
                                                        class="w-6 h-6"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                                                        ></path>
                                                    </svg>
                                                </div>
                                                <div class="grow">
                                                    <ObjectView
                                                        schema=schema
                                                        digest=read_d
                                                        path=new_path
                                                        selected=selected
                                                        on_action=on_action.clone()
                                                        debug=debug
                                                    />
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                                <button
                                    class="cursor-pointer text-green"
                                    on:click=move |_| {
                                        let new_value = move || field_type().type_.default_value();
                                        on_action(Action::Append {
                                            path: path5.clone(),
                                            field_id: field_id.clone(),
                                            position: Position::Last,
                                            value: new_value(),
                                        })
                                    }
                                >

                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-6 h-6"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                                        ></path>
                                    </svg>

                                </button>
                            </div>
                        }
                    }
                />

            </div>
        }
    };
    let view_string = move |v: Memo<String>| -> HtmlElement<html::Div> {
        // let vv = v.to_string();
        let path3 = path3.clone();
        view! {
            <div class="w-full">
                <input
                    class="border border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 p-2 w-full"
                    type="text"
                    prop:value=move || { v.get() }
                    on:input=move |ev| {
                        let new_value = event_target_value(&ev);
                        on_action(Action::Update(path3.clone(), Value::String(new_value)));
                    }
                />

            </div>
        }
    };
    let is_object = create_memo(move |_| match value.get() {
        Value::Object(_) => true,
        _ => false,
    });
    let is_string = create_memo(move |_| match value.get() {
        Value::String(_) => true,
        _ => false,
    });
    let object_type_memo = create_memo(move |_| {
        logging::log!("object_type_memo {:?}", value.get());
        let object_value = match value.get() {
            Value::Object(value) => value,
            _ => panic!("expected object value"),
        };
        schema
            .get()
            .object_types
            .get(&object_value.object_type_id)
            .unwrap()
            .clone()
    });
    view! {
        <div>
            <Show when=move || debug()>
                <div>
                    digest: {move || digest.get().to_hex()} value:
                    {move || format!("{:?}", node.get().unwrap().value)}
                </div>
            </Show>
            <div
                class=""
                class:selected=s
                on:click=move |ev| {
                    ev.stop_propagation();
                    selected.set(path.clone());
                }
            >

                <Show when=move || is_object()>

                    {
                        let object_value = create_memo(move |_| match value.get() {
                            Value::Object(value) => value,
                            _ => panic!("expected object value"),
                        });
                        let object_id = create_memo(move |_| object_value().object_type_id);
                        view_object(object_id, object_value)
                    }

                </Show>

                <Show when=move || {
                    is_string.get()
                }>

                    {
                        let string_value = create_memo(move |_| match value.get() {
                            Value::String(value) => value,
                            _ => panic!("expected string value"),
                        });
                        view_string(string_value)
                    }

                </Show>

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
