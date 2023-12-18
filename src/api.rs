use leptos::*;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(cx: Scope, promt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;    

    let model = extract(cx, |data: Data<Llama>, connection: ConnectionInfo| async {
        data.into_inner();
    }).await.unwrap();

    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let perona = "A chat between a human and an assistant";
    let mut history = format!(
    )
}






