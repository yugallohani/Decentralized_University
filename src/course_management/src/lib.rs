use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::{caller, export_candid, query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Simple types for the demo
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ContentType {
    Video,
    Text,
    Audio,
    Interactive,
    Quiz,
    Assignment,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub instructor_id: Principal,
    pub category: String,
    pub tags: Vec<String>,
    pub difficulty_level: DifficultyLevel,
    pub estimated_duration_hours: u32,
    pub price: u64,
    pub is_published: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub difficulty_level: DifficultyLevel,
    pub estimated_duration_hours: u32,
    pub price: u64,
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
    static COURSES: RefCell<HashMap<String, Course>> = RefCell::new(HashMap::new());
    static COURSE_COUNTER: RefCell<u64> = RefCell::new(0);
}

// Helper functions
fn get_current_time() -> u64 {
    time()
}

fn generate_course_id() -> String {
    COURSE_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        format!("course_{}", *counter)
    })
}

// Course Management Functions
#[update]
#[candid_method(update)]
async fn create_course(request: CreateCourseRequest) -> Result<Course> {
    let caller_id = caller();
    
    // Validate input
    if request.title.trim().is_empty() {
        return Err(ApiError::InvalidInput("Course title cannot be empty".to_string()));
    }
    
    if request.description.trim().is_empty() {
        return Err(ApiError::InvalidInput("Course description cannot be empty".to_string()));
    }

    let course_id = generate_course_id();
    let current_time = get_current_time();

    let course = Course {
        id: course_id.clone(),
        title: request.title,
        description: request.description,
        instructor_id: caller_id,
        category: request.category,
        tags: request.tags,
        difficulty_level: request.difficulty_level,
        estimated_duration_hours: request.estimated_duration_hours,
        price: request.price,
        is_published: false,
        created_at: current_time,
        updated_at: current_time,
    };

    COURSES.with(|courses| courses.borrow_mut().insert(course_id, course.clone()));
    Ok(course)
}

#[query]
#[candid_method(query)]
fn get_course(course_id: String) -> Result<Course> {
    COURSES.with(|courses| {
        courses.borrow().get(&course_id).cloned()
            .ok_or_else(|| ApiError::NotFound("Course not found".to_string()))
    })
}

#[update]
#[candid_method(update)]
async fn publish_course(course_id: String) -> Result<Course> {
    let caller_id = caller();

    COURSES.with(|courses| {
        let mut courses = courses.borrow_mut();
        match courses.get(&course_id).cloned() {
            Some(mut course) => {
                if course.instructor_id != caller_id {
                    return Err(ApiError::InsufficientPermissions);
                }

                course.is_published = true;
                course.updated_at = get_current_time();
                courses.insert(course_id, course.clone());
                Ok(course)
            }
            None => Err(ApiError::NotFound("Course not found".to_string()))
        }
    })
}

#[query]
#[candid_method(query)]
fn get_all_courses() -> Vec<Course> {
    COURSES.with(|courses| {
        courses.borrow().values().cloned().collect()
    })
}

#[query]
#[candid_method(query)]
fn get_published_courses() -> Vec<Course> {
    COURSES.with(|courses| {
        courses.borrow()
            .values()
            .filter(|course| course.is_published)
            .cloned()
            .collect()
    })
}

#[query]
#[candid_method(query)]
fn get_instructor_courses(instructor_id: Principal) -> Vec<Course> {
    COURSES.with(|courses| {
        courses.borrow()
            .values()
            .filter(|course| course.instructor_id == instructor_id)
            .cloned()
            .collect()
    })
}

#[query]
#[candid_method(query)]
fn search_courses(query: Option<String>, category: Option<String>) -> Vec<Course> {
    COURSES.with(|courses| {
        courses.borrow()
            .values()
            .filter(|course| {
                if !course.is_published {
                    return false;
                }

                if let Some(ref q) = query {
                    let q_lower = q.to_lowercase();
                    if !course.title.to_lowercase().contains(&q_lower) &&
                       !course.description.to_lowercase().contains(&q_lower) {
                        return false;
                    }
                }

                if let Some(ref cat) = category {
                    if course.category != *cat {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    })
}

// Export candid interface
export_candid!();
