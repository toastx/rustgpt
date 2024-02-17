use std::thread::Scope;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::models::conversation::Conversation;

#[component]
pub fn App() -> impl IntoView {
    
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String|){
        let user_message = Message{
            msg = new_message.clone(),
            sender:true
    };
    set_conversation.update(move|c|{
        c.message.push(user_message.clone());
    })
    };


    view! {
        <Stylesheet id="leptos" href="/pkg/chatbot.css"/>
        <Title text="RustGPT"/>
        <ChatBox conversation/>
        <PromptBox send/>

    }
}



