use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::{caller, export_candid, query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Simple types for demo
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Certification {
    pub id: String,
    pub user_id: Principal,
    pub course_id: String,
    pub title: String,
    pub description: String,
    pub issued_at: u64,
    pub skills_acquired: Vec<String>,
    pub final_score: u8,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    InvalidInput(String),
    AlreadyExists(String),
    InsufficientPermissions,
}

pub type Result<T> = std::result::Result<T, ApiError>;

// Simple in-memory storage for demo
thread_local! {
    static CERTIFICATIONS: RefCell<HashMap<String, Certification>> = RefCell::new(HashMap::new());
    static CERTIFICATION_COUNTER: RefCell<u64> = RefCell::new(0);
}

// Helper functions
fn get_current_time() -> u64 {
    time()
}

fn generate_certification_id() -> String {
    CERTIFICATION_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        format!("cert_{}", *counter)
    })
}

// Certification System Functions

#[update]
#[candid_method(update)]
async fn issue_certification(
    user_id: Principal,
    course_id: String,
    final_score: u8,
) -> Result<Certification> {
    let _caller_id = caller();
    
    let certification_id = generate_certification_id();
    let current_time = get_current_time();
    
    let certification = Certification {
        id: certification_id.clone(),
        user_id,
        course_id,
        title: "Certificate of Completion".to_string(),
        description: "This certifies successful course completion".to_string(),
        issued_at: current_time,
        skills_acquired: vec!["General Knowledge".to_string()],
        final_score,
    };
    
    // Store certification
    CERTIFICATIONS.with(|certs| certs.borrow_mut().insert(certification_id, certification.clone()));
    
    Ok(certification)
}

#[query]
#[candid_method(query)]
fn get_certification(certification_id: String) -> Result<Certification> {
    CERTIFICATIONS.with(|certs| {
        certs.borrow().get(&certification_id).cloned()
            .ok_or_else(|| ApiError::NotFound("Certification not found".to_string()))
    })
}

#[query]
#[candid_method(query)]
fn get_user_certifications(user_id: Principal) -> Vec<Certification> {
    CERTIFICATIONS.with(|certs| {
        certs.borrow()
            .values()
            .filter(|cert| cert.user_id == user_id)
            .cloned()
            .collect()
    })
}

#[query]
#[candid_method(query)]
fn get_all_certifications() -> Vec<Certification> {
    CERTIFICATIONS.with(|certs| {
        certs.borrow().values().cloned().collect()
    })
}

#[query]
#[candid_method(query)]
fn verify_certification(certification_id: String) -> Result<bool> {
    match get_certification(certification_id) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

export_candid!();
