#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, Map};

// =======================
// STRUCT
// =======================

// Data soal
#[contracttype]
#[derive(Clone, Debug)]
pub struct Question {
    id: u64,
    question: String,
    answer: String,
}

// =======================
// STORAGE KEY
// =======================

const QUESTION_DATA: Symbol = symbol_short!("Q_DATA");
const SCORE_DATA: Symbol = symbol_short!("SCORE");

// =======================
// CONTRACT
// =======================

#[contract]
pub struct QuizContract;

#[contractimpl]
impl QuizContract {

    // =======================
    // GET ALL QUESTIONS
    // =======================
    pub fn get_questions(env: Env) -> Vec<Question> {
        env.storage()
            .instance()
            .get(&QUESTION_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // =======================
    // CREATE QUESTION
    // =======================
    pub fn create_question(env: Env, question: String, answer: String) -> String {
        let mut questions: Vec<Question> = env
            .storage()
            .instance()
            .get(&QUESTION_DATA)
            .unwrap_or(Vec::new(&env));

        let new_q = Question {
            id: env.prng().gen::<u64>(),
            question,
            answer,
        };

        questions.push_back(new_q);

        env.storage().instance().set(&QUESTION_DATA, &questions);

        String::from_str(&env, "Soal berhasil ditambahkan")
    }

    // =======================
    // ANSWER QUESTION
    // =======================
    pub fn answer_question(env: Env, id: u64, user_answer: String, user: String) -> String {
        let questions: Vec<Question> = env
            .storage()
            .instance()
            .get(&QUESTION_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..questions.len() {
            let q = questions.get(i).unwrap();

            if q.id == id {
                if q.answer == user_answer {
                    // ambil score user
                    let mut scores: Map<String, u64> = env
                        .storage()
                        .instance()
                        .get(&SCORE_DATA)
                        .unwrap_or(Map::new(&env));

                    let current = scores.get(user.clone()).unwrap_or(0);

                    scores.set(user.clone(), current + 1);

                    env.storage().instance().set(&SCORE_DATA, &scores);

                    return String::from_str(&env, "Jawaban benar! +1 poin");
                } else {
                    return String::from_str(&env, "Jawaban salah!");
                }
            }
        }

        String::from_str(&env, "Soal tidak ditemukan")
    }

    // =======================
    // GET SCORE USER
    // =======================
    pub fn get_score(env: Env, user: String) -> u64 {
        let scores: Map<String, u64> = env
            .storage()
            .instance()
            .get(&SCORE_DATA)
            .unwrap_or(Map::new(&env));

        scores.get(user).unwrap_or(0)
    }
}

mod test;