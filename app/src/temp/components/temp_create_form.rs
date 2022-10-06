use dioxus::{events::MouseEvent, prelude::*};

// use crate::chatroom::components::ChatroomCard;

#[derive(Props)]
pub struct TempCreateFormProps<'a> {
    // chatroom_list: &'a im_rc::HashMap<u32, ChatroomCard>,
    // chatroom_id: &'a u32,
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn TempCreateForm<'a>(cx: Scope<'a, TempCreateFormProps<'a>>) -> Element {
    let draft = use_state(&cx, || "".to_string());

    let view = rsx! {
        div {
            class: "p-2 flex justify-around",
            input {
                placeholder: "Type here",
                class: "input w-full max-w-xs m-2",
                value: "{draft}",
                oninput: move |evt| draft.set(evt.value.clone()),
            }
            button {
                onclick: move |evt| {
                    if !draft.is_empty(){
                        cx.props.onclick.call(evt);
                        draft.set(String::from(""));
                    }
                },
                h2 {"create"}
            }
        }
    };

    cx.render(view)
}
