use crate::store::Store;
use crate::types::pagination::Pagination;
use crate::types::question::{NewQuestion, Question, QuestionId};
use handle_errors::Error;
use warp::reject::custom;
use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "practical_rust_book",Level::INFO,"querying questions");
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
        info!(pagination = false);
        let res: Vec<Question> = match store
            .get_questions(pagination.limit, pagination.offset)
        .await  {
            Ok(res) => res,
            Err(e) => {
                return Err(warp::reject::custom(Error::DatabaseQueryError(e)
                )),
            },
        };
        Ok(warp::reply::json(&res))
    }

}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question,id).await {
        Ok(res) => res,
Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e)))
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => (),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(custom(Error::DatabaseQueryError(e)));
    }
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
