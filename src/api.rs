use leptos::*;

use crate::models::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(prompt:Conversation) -> Result<String, ServerFnError>{
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;   

    let model = extract(|data:Data<Llama>, _connection_info:ConnectionInfo| async{
    data.into_inner()
    })
    .await.unwrap();

    use llm::KnownModel;
    let bot_name = "RustGPT";
    let user_name = "Me";
    let persona = "Human LLM  interaction";

    let mut history = format!("
    {bot_name}: Hello ! How may I assist u today\n\
    {user_name}: What is 5 times 9?\n\"
    {bot_name}: 5 times 9 is 45\n");

    for message in prompt.messages.into_iter(){
        let msg = message.text;
        let current_line = if mesage.user{
            format!("{bot_name}:{msg}\n")   
        }
        else{
            format!("{user_name}:{msg}\n")  
        };
        history.push_str(&current_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());

    session.infer(
        &model,
        &mut rng,
        &llm::InferenceRequest{
            prompt: format!("{history}\n{user_name}").as_str().into(),
            parameters : &llm::InferenceParameters::default(),
            maximum_token_count: None
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut buf, &mut res),

    )
    .unwrap_or_else(|e| panic("{e}")

    Ok(res)
)

}

fn inference_callback<'a>(
    stop_sequence: String,
    buf: &'a mut String,
    tx: tokio::sync::mpsc::Sender<String>,
    runtime: &'a mut tokio::runtime::Runtime,
) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    use llm::InferenceFeedback::Halt;
    use llm::InferenceFeedback::Continue;

    move |resp| -> Result<llm::InferenceFeedback, Infallible> {match resp {
        llm::InferenceResponse::InferredToken(t) => {
            let mut reverse_buf = buf.clone();
            reverse_buf.push_str(t.as_str());
            if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                buf.clear();
                return Ok(Halt);
            } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                buf.push_str(t.as_str());
                return Ok(Continue);
            }
            let text_to_send = if buf.is_empty() {
                t.clone()
            } else {
                reverse_buf
            };

            let tx_cloned = tx.clone();
            runtime.block_on(async move {
                tx_cloned.send(text_to_send).await.expect("issue sending on channel");
            });

            Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
        _ => Ok(Continue),
    }}
}
