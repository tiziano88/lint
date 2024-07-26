#![feature(async_closure)]
#![feature(async_fn_traits)]

use core::panic;
use leptos::*;
use maplit::btreemap;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display, Formatter},
    sync::Arc,
};

mod schema;
mod storage;
mod components;

use schema::*;
use storage::*;
use components::*;

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
    logging::log!("find_value {:?} {:?}", root_digest.to_hex(), path);
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
        let next_digest = field.get(selector.index).unwrap();
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
        if !s.starts_with("sha2-256:") {
            return D::default();
        }
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
    SetFocus {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}

pub fn set_root_digest_in_url_hash(d: &D) {
        window()
            .location()
            .replace(format!("/#{}", d.to_hex()).as_str())
            .expect("failed to replace location");
}

async fn upload_node(api_key: &str, node: Node) -> anyhow::Result<()> {
    let s3_url = "https://api.static.space/v1/upload";
    // let content = get_item(&digest).get_untracked().unwrap().serialize();
    let content = node.serialize();
    // logging::log!("uploading {:?}", digest.to_hex());
    logging::log!("uploading {:?}", content);
    let res = reqwasm::http::Request::post(&format!("{s3_url}"))
        .header("Content-Type", "application/json")
        .header("bucket-key", &api_key)
        .body(content)
        .send()
        .await?;
    logging::log!("upload res {:?}", res);
    Ok(())
}

async fn upload(api_key: String, digest: D) -> leptos::error::Result<()> {
    traverse_async(digest, move |node: Node| {
        let api_key = api_key.clone();
      async move { 
        upload_node(&api_key.clone(), node.clone()).await.unwrap();
    }
}).await?;
    Ok(())
}

const STATIC_SPACE_API_URL: &str = "https://api.static.space";

async fn download(digest: D) -> anyhow::Result<()> {
    logging::log!("downloading {:?}", digest.to_hex());
    let digest_hex = digest.to_hex();
    let res = reqwasm::http::Request::get(&format!("{STATIC_SPACE_API_URL}/v1/raw/{digest_hex}"))
        .send()
        .await?;
    logging::log!("download res {:?}", res);
    if res.status() != 200 {
        return Err(anyhow::anyhow!("failed to download").into());
    }
    let body = res.text().await?;
    // parse node
    logging::log!("body {:?}", body);
    let node = Node::deserialize(&body).unwrap();
    logging::log!("node {:?}", node);
    let d = set_item(&node);
    logging::log!("new digest {:?}", d.to_hex());
    Ok(())
}

async fn traverse_async<F: std::ops::AsyncFn(Node) + Clone>(digest: D, f : F) -> leptos::error::Result<()> {
    let node = get_item(&digest).get_untracked().unwrap();
    f.clone()(node.clone()).await;
    match node.value {
        Value::Object(object) => {
            for (field_id, field) in object.fields.iter() {
                for child_digest in field {
                    Box::pin(traverse_async(child_digest.clone(), f.clone())).await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
