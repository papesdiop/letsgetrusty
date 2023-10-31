use rocket::{serde::json::Json, State};

use crate::models::*;

use crate::persistance::answers_dao::AnswersDao;
use crate::persistance::questions_dao::QuestionsDao;

mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Json<QuestionDetail> {
    Json (
        handlers_inner::create_question(question.into_inner(), questions_dao).await.unwrap()
    )
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>, // add the appropriate type annotation
) -> Json<Vec<QuestionDetail>> {
    Json (
        handlers_inner::read_questions(questions_dao).await.unwrap()
    )
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>, // add the appropriate type annotation
) {
    handlers_inner::delete_question(question_uuid.into_inner(), questions_dao).await;
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Json<AnswerDetail> {
    Json (
        handlers_inner::create_answer(answer.into_inner(), answers_dao).await.unwrap()
    )
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>, // add the appropriate type annotation
) -> Json<Vec<AnswerDetail>> {
    Json (
        handlers_inner::read_answers(question_uuid.into_inner(), answers_dao).await.unwrap()
    )
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao +Sync + Send>>, // add the appropriate type annotation
) {
    handlers_inner::delete_answer(answer_uuid.into_inner(), answers_dao).await;
}