use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::{caller, export_candid, query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Simple types for the demo
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum UserRole {
    Student,
    Instructor,
    Admin,
    Moderator,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub earned_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct User {
    pub id: Principal,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: u64,
    pub updated_at: u64,
    pub reputation_score: u32,
    pub skills: Vec<String>,
    pub achievements: Vec<Achievement>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub bio: Option<String>,
    pub skills: Vec<String>,
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

// Simple in-memory storage for demo purposes
thread_local! {
    static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
    static USERNAME_TO_ID: RefCell<HashMap<String, Principal>> = RefCell::new(HashMap::new());
    static EMAIL_TO_ID: RefCell<HashMap<String, Principal>> = RefCell::new(HashMap::new());
}

// Helper functions
fn get_current_time() -> u64 {
    time()
}

fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

fn is_valid_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 50 && username.chars().all(|c| c.is_alphanumeric() || c == '_')
}

// User Management Functions

#[update]
#[candid_method(update)]
async fn create_user(request: CreateUserRequest) -> Result<User> {
    let caller_id = caller();
    
    // Validate input
    if !is_valid_username(&request.username) {
        return Err(ApiError::InvalidInput("Invalid username format".to_string()));
    }
    
    if !is_valid_email(&request.email) {
        return Err(ApiError::InvalidInput("Invalid email format".to_string()));
    }

    if request.full_name.trim().is_empty() {
        return Err(ApiError::InvalidInput("Full name cannot be empty".to_string()));
    }

    // Check if user already exists
    if USERS.with(|users| users.borrow().contains_key(&caller_id)) {
        return Err(ApiError::AlreadyExists("User already exists".to_string()));
    }

    // Check if username is taken
    if USERNAME_TO_ID.with(|map| map.borrow().contains_key(&request.username)) {
        return Err(ApiError::AlreadyExists("Username already taken".to_string()));
    }

    // Check if email is taken
    if EMAIL_TO_ID.with(|map| map.borrow().contains_key(&request.email)) {
        return Err(ApiError::AlreadyExists("Email already registered".to_string()));
    }

    let current_time = get_current_time();
    
    let user = User {
        id: caller_id,
        username: request.username.clone(),
        email: request.email.clone(),
        full_name: request.full_name,
        bio: request.bio,
        avatar_url: None,
        role: UserRole::Student,
        created_at: current_time,
        updated_at: current_time,
        reputation_score: 0,
        skills: request.skills,
        achievements: vec![],
    };

    // Store user and mappings
    USERS.with(|users| users.borrow_mut().insert(caller_id, user.clone()));
    USERNAME_TO_ID.with(|map| map.borrow_mut().insert(request.username, caller_id));
    EMAIL_TO_ID.with(|map| map.borrow_mut().insert(request.email, caller_id));

    Ok(user)
}

#[query]
#[candid_method(query)]
fn get_user(user_id: Principal) -> Result<User> {
    USERS.with(|users| {
        users.borrow().get(&user_id).cloned()
            .ok_or_else(|| ApiError::NotFound("User not found".to_string()))
    })
}

#[query]
#[candid_method(query)]
fn get_current_user() -> Result<User> {
    let caller_id = caller();
    get_user(caller_id)
}

#[query]
#[candid_method(query)]
fn get_user_by_username(username: String) -> Result<User> {
    let user_id = USERNAME_TO_ID.with(|map| {
        map.borrow().get(&username).copied()
            .ok_or_else(|| ApiError::NotFound("Username not found".to_string()))
    })?;
    
    get_user(user_id)
}

#[update]
#[candid_method(update)]
async fn update_user_profile(
    bio: Option<String>,
    avatar_url: Option<String>,
    skills: Vec<String>
) -> Result<User> {
    let caller_id = caller();
    
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        match users.get(&caller_id).cloned() {
            Some(mut user) => {
                user.bio = bio;
                user.avatar_url = avatar_url;
                user.skills = skills;
                user.updated_at = get_current_time();
                
                users.insert(caller_id, user.clone());
                Ok(user)
            }
            None => Err(ApiError::NotFound("User not found".to_string()))
        }
    })
}

#[update]
#[candid_method(update)]
async fn update_user_role(user_id: Principal, new_role: UserRole) -> Result<User> {
    let caller_id = caller();
    
    // Check if caller has admin privileges
    let caller_user = get_user(caller_id)?;
    match caller_user.role {
        UserRole::Admin => {},
        _ => return Err(ApiError::InsufficientPermissions),
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        match users.get(&user_id).cloned() {
            Some(mut user) => {
                user.role = new_role;
                user.updated_at = get_current_time();
                
                users.insert(user_id, user.clone());
                Ok(user)
            }
            None => Err(ApiError::NotFound("User not found".to_string()))
        }
    })
}

#[update]
#[candid_method(update)]
async fn add_achievement(user_id: Principal, achievement: Achievement) -> Result<User> {
    let caller_id = caller();
    
    // Check if caller has permission to add achievements (admin or instructor)
    let caller_user = get_user(caller_id)?;
    match caller_user.role {
        UserRole::Admin | UserRole::Instructor => {},
        _ => return Err(ApiError::InsufficientPermissions),
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        match users.get(&user_id).cloned() {
            Some(mut user) => {
                user.achievements.push(achievement);
                user.updated_at = get_current_time();
                
                users.insert(user_id, user.clone());
                Ok(user)
            }
            None => Err(ApiError::NotFound("User not found".to_string()))
        }
    })
}

#[update]
#[candid_method(update)]
async fn update_reputation_score(user_id: Principal, score_delta: i32) -> Result<User> {
    let caller_id = caller();
    
    // Check if caller has permission (admin or system)
    let caller_user = get_user(caller_id)?;
    match caller_user.role {
        UserRole::Admin => {},
        _ => return Err(ApiError::InsufficientPermissions),
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        match users.get(&user_id).cloned() {
            Some(mut user) => {
                if score_delta < 0 && user.reputation_score < (-score_delta) as u32 {
                    user.reputation_score = 0;
                } else {
                    user.reputation_score = ((user.reputation_score as i32) + score_delta).max(0) as u32;
                }
                user.updated_at = get_current_time();
                
                users.insert(user_id, user.clone());
                Ok(user)
            }
            None => Err(ApiError::NotFound("User not found".to_string()))
        }
    })
}

#[query]
#[candid_method(query)]
fn get_users_by_role(role: UserRole) -> Vec<User> {
    USERS.with(|users| {
        users.borrow()
            .iter()
            .filter_map(|(_, user)| {
                if user.role == role {
                    Some(user.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
#[candid_method(query)]
fn get_user_count() -> u64 {
    USERS.with(|users| users.borrow().len() as u64)
}

#[query]
#[candid_method(query)]
fn search_users(query: String, limit: Option<u32>) -> Vec<User> {
    let limit = limit.unwrap_or(10).min(100) as usize;
    let query_lower = query.to_lowercase();
    
    USERS.with(|users| {
        users.borrow()
            .iter()
            .filter_map(|(_, user)| {
                if user.username.to_lowercase().contains(&query_lower) ||
                   user.full_name.to_lowercase().contains(&query_lower) ||
                   user.skills.iter().any(|skill| skill.to_lowercase().contains(&query_lower)) {
                    Some(user.clone())
                } else {
                    None
                }
            })
            .take(limit)
            .collect()
    })
}

#[query]
#[candid_method(query)]
fn get_leaderboard(limit: Option<u32>) -> Vec<User> {
    let limit = limit.unwrap_or(10).min(100) as usize;
    
    USERS.with(|users| {
        let mut user_list: Vec<User> = users.borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();
        
        user_list.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score));
        user_list.truncate(limit);
        user_list
    })
}

// System functions
#[query]
#[candid_method(query)]
fn is_admin(user_id: Principal) -> bool {
    USERS.with(|users| {
        users.borrow().get(&user_id)
            .map(|user| matches!(user.role, UserRole::Admin))
            .unwrap_or(false)
    })
}

#[query]
#[candid_method(query)]
fn is_instructor(user_id: Principal) -> bool {
    USERS.with(|users| {
        users.borrow().get(&user_id)
            .map(|user| matches!(user.role, UserRole::Instructor | UserRole::Admin))
            .unwrap_or(false)
    })
}

// Export candid interface
export_candid!();
