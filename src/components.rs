use core::panic;
use leptos::*;
use super::Action;
use super::Selector;
use super::Value;
use super::Schema;
use super::Path;
use super::Position;
use super::ID;
use super::D;
use super::FieldType;
use super::ObjectValue;
use super::format_path;
use super::get_item;
use super::storage;
use super::*;

#[component]
pub fn App() -> impl IntoView {
    logging::log!("rendering App");
    let (schema, _set_schema) = create_signal(create_schema());

    let (api_key, set_api_key) = create_signal("api-key".to_string());

    let (fetch_queue, set_fetch_queue) = create_signal(Vec::<D>::new());

    let value = create_rw_signal(create_value());
    let _root_type = Type::Object(schema.get_untracked().root_object_type_id);

    let (debug, _set_debug) = create_signal(false);


    let (history, _set_history) = create_signal(Vec::<D>::new());

    let _store = Arc::new(LocalStorage::<Node, D>::new());

    let node = Node {
        id: 1,
        value: create_value(),
    };
    let d = set_item(&node);


    // TODO: derived signals have different types.
    // let root_digest = Signal::derive(move || history.get().last().cloned().unwrap());

    let (root_digest, set_root_digest) = create_signal(d);
    let _root_digest_memo = create_memo(move |_| root_digest.get());

    let selected_path = create_rw_signal(Path::default());
    let focus_path = create_rw_signal(Path::default());
    let focused_digest = create_memo(move |_| {
        let path = focus_path.get();
        let digest = find_value(&root_digest.get(), &path).unwrap();
        logging::log!("focused_digest {:?}", digest.to_hex());
        digest
    });
    let focus_path_memo = create_memo(move |_| focus_path.get().clone());
    let _selected_element = create_memo(move |_| format_path(&selected_path.get()));

    let (response, set_response) = create_signal("---".to_string());

    create_effect(move |_| {
        let v = storage::get_value("api_key").get();
        set_api_key(v);
    });

    create_effect(move |_| {
        logging::log!("initializing App");
        // First try to read from the hash.
        let hash = window().location().hash().unwrap();
        if hash.len() > 1 {
            let d = D::from_hex(&hash[1..]);
            logging::log!("obtained root digest from URL hash fragment: {:?}", d.to_hex());
        set_root_digest(d.clone());
        } else {
            // If the hash is empty, create a new root node.
            let node = Node {
                id: new_id(),
                value: create_value(),
            };
            let d = set_item(&node);
            logging::log!("no root digest, creating empty root node: {:?}", d.to_hex());
        set_root_digest(d.clone());
            set_root_digest_in_url_hash(&d);
        }
    });

    /* 
    create_effect(move |_| {
        let mut d = get_root();
        logging::log!("raw root {:?}", d.to_hex());
        if d.is_empty() {
            let node = Node {
                id: new_id(),
                value: create_value(),
            };
            d = set_item(&node);
            set_root(&d)
        }
        logging::log!("new root {:?}", d.to_hex());
        set_root_digest(d.clone());
        window()
            .location()
            .replace(format!("/#{}", d.to_hex()).as_str())
            .expect("failed to replace location");
    });
*/

    let on_action = move |action| {
        logging::log!("action {:?}", action);
        match action {
            Action::Noop => {}
            Action::Update(path, value) => {
                let new_d = update_node(&root_digest(), &path, value);
                // set_root(&new_d);
                set_root_digest(new_d.clone());
                set_root_digest_in_url_hash(&new_d);
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
            Action::SetFocus { path } => {
                focus_path.set(path);
            }
        }
    };

    let queue_fetch = move |digest: D| {
        if digest.is_empty() {
            logging::log!("empty digest; ignoring");
            return;
        }
        logging::log!("queue_fetch {:?}", digest.to_hex());
        // set_fetch_queue.update(|queue| {
        //     queue.push(digest);
        // });
        spawn_local_with_current_owner(async move {
            download(digest).await.unwrap();
        }).unwrap();
    };

    view! {
        <div class="">
            // <List/>
            <div>sel: {move || format_path(&selected_path.get())}</div>
            <div>root_digest: {move || root_digest.get().to_hex()}</div>
            <div>focused: {move || format_path(&focus_path.get())}</div>
            <div>hist: {move || format!("{:?}", history.get())}</div>
            <ObjectView
                schema=schema
                digest=focused_digest
                // TODO: Does not propagate correctly.
                path=focus_path_memo
                selected=selected_path
                on_action=on_action
                debug=debug
                queue_fetch=queue_fetch
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

            <button
                class="button"
                on:click=move |_| {
                    spawn_local_with_current_owner(async move {
                            upload(api_key.get(), root_digest.get()).await.unwrap();
                        })
                        .unwrap();
                }
            >

                Upload
            </button>

            <input
                class="border border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 p-2 w-full"
                type="text"
                prop:value=move || { api_key.get() }
                on:input=move |ev| {
                    let new_value = event_target_value(&ev);
                    storage::set_value("api_key", &new_value);
                    set_api_key(new_value);
                }
            />

            <div>{move || response.get()}</div>
            // { move || fetch_queue.get() }
            <div>"fetch queue:"</div>

        </div>
    }
}

#[component]
fn ObjectView(
    schema: ReadSignal<Schema>,
    digest: Memo<D>,
    #[prop(into)] on_action: Callback<Action>,
    path: Memo<Path>,
    selected: RwSignal<Path>,
    debug: ReadSignal<bool>,
    #[prop(into)] queue_fetch: Callback<D>,
) -> impl IntoView {
    logging::log!("rendering ObjectView {:?}", path.get_untracked());
    let node = create_memo(move |_| get_item(&digest.get()).get());
    let is_present = create_memo(move |_| node.get().is_some());
    let value = create_memo(move |_| node.get().unwrap().value.clone());
    let path2 = path.clone();
    let path3 = path.clone();
    let path4 = path.clone();
    let s = create_memo(move |_| path.get() == selected.get());
    fn change_value() {}
    let view_object = move |_id: Memo<ID>, v: Memo<ObjectValue>| -> HtmlElement<html::Div> {
        logging::log!("view_object {:?} {:?}", path2.get(), v.get());
        let object_type = move || {
            schema
                .get()
                .object_types
                .get(&v().object_type_id)
                .unwrap()
                .clone()
        };
        let v = v.clone();
        let v3 = v.clone();
        let path4 = path4.clone();
        let path5 = path4.clone();
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
                        <button
                            class="cursor-pointer"
                            title="focus on this element"

                            on:click=move |_| {
                                on_action(Action::SetFocus {
                                    path: path5.get(),
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
                                    d="M7.5 3.75H6A2.25 2.25 0 0 0 3.75 6v1.5M16.5 3.75H18A2.25 2.25 0 0 1 20.25 6v1.5m0 9V18A2.25 2.25 0 0 1 18 20.25h-1.5m-9 0H6A2.25 2.25 0 0 1 3.75 18v-1.5M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                                ></path>
                            </svg>
                        </button>
                    </div>
                </div>
                // Iterate over the fields of the object type.
                <For
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
                        let _path5 = path4.clone();
                        view! {
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
                                        let _field_type = field_type1.clone();
                                        let _v3 = v3.clone();
                                        let new_path = create_memo(move |_| {
                                            let mut new_path = path.get();
                                            new_path.push(Selector { field_id, index });
                                            new_path
                                        });
                                        view! {
                                            <div class="mx-4 my-2 flex">
                                                <Show when=move || debug()>
                                                    {format_path(&new_path.get())}
                                                </Show>
                                                <div
                                                    class="cursor-pointer text-red"
                                                    on:click=move |_| {
                                                        on_action(Action::Delete {
                                                            path: new_path.get(),
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
                                                        queue_fetch=queue_fetch
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
                                            path: path.get(),
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
        let _path3 = path3.clone();
        view! {
            <div class="w-full">
                <input
                    class="border border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 p-2 w-full"
                    type="text"
                    prop:value=move || { v.get() }
                    on:input=move |ev| {
                        let new_value = event_target_value(&ev);
                        on_action(Action::Update(path.get(), Value::String(new_value)));
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
    let _object_type_memo = create_memo(move |_| {
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
            <Show
                when=move || is_present()
                fallback=move || {
                    queue_fetch(digest.get());
                    view! { <div>"[" {move || digest.get().to_hex()} "]" "not found"</div> }
                }
            >

                <div
                    class=""
                    class:selected=s
                    on:click=move |ev| {
                        ev.stop_propagation();
                        selected.set(path.get());
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
            </Show>
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
