use warp::http::StatusCode;

use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::account::Session;
use crate::types::answer::NewAnswer;

pub async fn add_answer(
    session: Session,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let id = new_answer.question_id.0;
    if store.is_question_owner(id, &account_id).await? {
        let content = match check_profanity(new_answer.content).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        let answer = NewAnswer {
            content,
            question_id: new_answer.question_id,
        };
        match store.add_answer(answer).await {
            Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}
