use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::account::Session;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};
use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
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
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        return Ok(warp::reply::json(&res));
    } else {
        return Ok(warp::reply::json(&Vec::<u32>::new()));
    }
}

pub async fn update_question(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        let title = check_profanity(question.title);
        let content = check_profanity(question.content);
        let (title, content) = tokio::join!(title, content);
        if title.is_ok() && content.is_ok() {
            let question = Question {
                id: question.id,
                title: title.unwrap(),
                content: content.unwrap(),
                tags: question.tags,
            };
            match store.update_question(question, id, account_id).await {
                Ok(res) => Ok(warp::reply::json(&res)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        } else {
            Err(warp::reject::custom(
                title.expect_err("Expected API call to have failed here"),
            ))
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        if let Err(e) = store.delete_question(id).await {
            return Err(warp::reject::custom(e));
        }

        Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
