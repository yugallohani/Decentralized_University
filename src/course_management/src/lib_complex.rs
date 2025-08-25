use candid::{candid_method, Principal};
use ic_cdk::api::time;
use ic_cdk::{caller, export_candid, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

// Import shared types
mod types {
    pub use candid::{CandidType, Deserialize, Principal};
    pub use serde::Serialize;
    pub use std::collections::HashMap;

    include!("../../types.rs");
}

use types::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;

impl Storable for Course {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::Bound = ic_stable_structures::Bound::Unbounded;
}

impl Storable for Lesson {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::Bound = ic_stable_structures::Bound::Unbounded;
}

impl Storable for Enrollment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::Bound = ic_stable_structures::Bound::Unbounded;
}

impl Storable for DiscussionThread {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::Bound = ic_stable_structures::Bound::Unbounded;
}

impl Storable for LearningPath {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::Bound = ic_stable_structures::Bound::Unbounded;
}

impl Storable for String {
    fn to_bytes(&self) -> Cow<[u8]> {
        self.as_bytes().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        String::from_utf8(bytes.into_owned()).unwrap()
    }

    const BOUND: ic_stable_structures::Bound = ic_stable_structures::Bound::Unbounded;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COURSES: RefCell<StableBTreeMap<String, Course, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static LESSONS: RefCell<StableBTreeMap<String, Lesson, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );

    static ENROLLMENTS: RefCell<StableBTreeMap<String, Enrollment, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );

    static DISCUSSIONS: RefCell<StableBTreeMap<String, DiscussionThread, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

    static LEARNING_PATHS: RefCell<StableBTreeMap<String, LearningPath, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
        )
    );

    static COURSE_COUNTER: RefCell<u64> = RefCell::new(0);
    static LESSON_COUNTER: RefCell<u64> = RefCell::new(0);
    static DISCUSSION_COUNTER: RefCell<u64> = RefCell::new(0);
    static LEARNING_PATH_COUNTER: RefCell<u64> = RefCell::new(0);
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

fn generate_lesson_id() -> String {
    LESSON_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        format!("lesson_{}", *counter)
    })
}

fn generate_discussion_id() -> String {
    DISCUSSION_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        format!("discussion_{}", *counter)
    })
}

fn generate_learning_path_id() -> String {
    LEARNING_PATH_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        format!("learning_path_{}", *counter)
    })
}

fn enrollment_key(user_id: &Principal, course_id: &str) -> String {
    format!("{}_{}", user_id.to_text(), course_id)
}

// Inter-canister call to check if user is instructor
async fn is_user_instructor(user_id: Principal) -> bool {
    // This would be an actual inter-canister call in production
    // For now, returning true for demonstration
    true
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

    // Check if user has instructor privileges
    if !is_user_instructor(caller_id).await {
        return Err(ApiError::InsufficientPermissions);
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
        thumbnail_url: None,
        lessons: vec![],
        prerequisites: request.prerequisites,
        created_at: current_time,
        updated_at: current_time,
        enrollment_count: 0,
        rating: 0.0,
        reviews: vec![],
        is_published: false,
    };

    COURSES.with(|courses| courses.borrow_mut().insert(course_id, course.clone()));
    Ok(course)
}

#[query]
#[candid_method(query)]
fn get_course(course_id: String) -> Result<Course> {
    COURSES.with(|courses| {
        courses.borrow().get(&course_id)
            .ok_or_else(|| ApiError::NotFound("Course not found".to_string()))
    })
}

#[update]
#[candid_method(update)]
async fn update_course(
    course_id: String,
    title: Option<String>,
    description: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    difficulty_level: Option<DifficultyLevel>,
    estimated_duration_hours: Option<u32>,
    price: Option<u64>,
) -> Result<Course> {
    let caller_id = caller();

    COURSES.with(|courses| {
        let mut courses = courses.borrow_mut();
        match courses.get(&course_id) {
            Some(mut course) => {
                // Check if caller is the course instructor or admin
                if course.instructor_id != caller_id {
                    return Err(ApiError::InsufficientPermissions);
                }

                // Update fields if provided
                if let Some(title) = title {
                    if title.trim().is_empty() {
                        return Err(ApiError::InvalidInput("Title cannot be empty".to_string()));
                    }
                    course.title = title;
                }
                if let Some(description) = description {
                    course.description = description;
                }
                if let Some(category) = category {
                    course.category = category;
                }
                if let Some(tags) = tags {
                    course.tags = tags;
                }
                if let Some(difficulty_level) = difficulty_level {
                    course.difficulty_level = difficulty_level;
                }
                if let Some(estimated_duration_hours) = estimated_duration_hours {
                    course.estimated_duration_hours = estimated_duration_hours;
                }
                if let Some(price) = price {
                    course.price = price;
                }

                course.updated_at = get_current_time();
                courses.insert(course_id, course.clone());
                Ok(course)
            }
            None => Err(ApiError::NotFound("Course not found".to_string()))
        }
    })
}

#[update]
#[candid_method(update)]
async fn publish_course(course_id: String) -> Result<Course> {
    let caller_id = caller();

    COURSES.with(|courses| {
        let mut courses = courses.borrow_mut();
        match courses.get(&course_id) {
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

#[update]
#[candid_method(update)]
async fn add_lesson(request: CreateLessonRequest) -> Result<Lesson> {
    let caller_id = caller();

    // Check if course exists and caller is instructor
    let course = get_course(request.course_id.clone())?;
    if course.instructor_id != caller_id {
        return Err(ApiError::InsufficientPermissions);
    }

    if request.title.trim().is_empty() {
        return Err(ApiError::InvalidInput("Lesson title cannot be empty".to_string()));
    }

    let lesson_id = generate_lesson_id();
    let current_time = get_current_time();

    let lesson = Lesson {
        id: lesson_id.clone(),
        course_id: request.course_id.clone(),
        title: request.title,
        description: request.description,
        content_type: request.content_type,
        content_url: request.content_url,
        duration_minutes: request.duration_minutes,
        order_index: 0, // Will be updated based on course lessons
        prerequisites: request.prerequisites,
        learning_objectives: request.learning_objectives,
        created_at: current_time,
        updated_at: current_time,
    };

    // Add lesson to course
    COURSES.with(|courses| {
        let mut courses = courses.borrow_mut();
        if let Some(mut course) = courses.get(&request.course_id) {
            course.lessons.push(lesson_id.clone());
            course.updated_at = current_time;
            courses.insert(request.course_id, course);
        }
    });

    LESSONS.with(|lessons| lessons.borrow_mut().insert(lesson_id, lesson.clone()));
    Ok(lesson)
}

#[query]
#[candid_method(query)]
fn get_lesson(lesson_id: String) -> Result<Lesson> {
    LESSONS.with(|lessons| {
        lessons.borrow().get(&lesson_id)
            .ok_or_else(|| ApiError::NotFound("Lesson not found".to_string()))
    })
}

#[query]
#[candid_method(query)]
fn get_course_lessons(course_id: String) -> Vec<Lesson> {
    LESSONS.with(|lessons| {
        lessons.borrow()
            .iter()
            .filter_map(|(_, lesson)| {
                if lesson.course_id == course_id {
                    Some(lesson)
                } else {
                    None
                }
            })
            .collect()
    })
}

#[update]
#[candid_method(update)]
async fn enroll_in_course(course_id: String) -> Result<Enrollment> {
    let caller_id = caller();

    // Check if course exists and is published
    let course = get_course(course_id.clone())?;
    if !course.is_published {
        return Err(ApiError::InvalidInput("Course is not published".to_string()));
    }

    let enrollment_id = enrollment_key(&caller_id, &course_id);

    // Check if already enrolled
    if ENROLLMENTS.with(|enrollments| enrollments.borrow().contains_key(&enrollment_id)) {
        return Err(ApiError::AlreadyExists("Already enrolled in this course".to_string()));
    }

    let current_time = get_current_time();
    let enrollment = Enrollment {
        user_id: caller_id,
        course_id: course_id.clone(),
        enrolled_at: current_time,
        progress: CourseProgress {
            completed_lessons: vec![],
            quiz_scores: HashMap::new(),
            assignment_submissions: HashMap::new(),
            time_spent_minutes: 0,
        },
        completion_percentage: 0.0,
        last_accessed: current_time,
    };

    // Update course enrollment count
    COURSES.with(|courses| {
        let mut courses = courses.borrow_mut();
        if let Some(mut course) = courses.get(&course_id) {
            course.enrollment_count += 1;
            course.updated_at = current_time;
            courses.insert(course_id, course);
        }
    });

    ENROLLMENTS.with(|enrollments| enrollments.borrow_mut().insert(enrollment_id, enrollment.clone()));
    Ok(enrollment)
}

#[query]
#[candid_method(query)]
fn get_user_enrollment(user_id: Principal, course_id: String) -> Result<Enrollment> {
    let enrollment_id = enrollment_key(&user_id, &course_id);
    ENROLLMENTS.with(|enrollments| {
        enrollments.borrow().get(&enrollment_id)
            .ok_or_else(|| ApiError::NotFound("Enrollment not found".to_string()))
    })
}

#[query]
#[candid_method(query)]
fn get_user_enrollments(user_id: Principal) -> Vec<Enrollment> {
    ENROLLMENTS.with(|enrollments| {
        enrollments.borrow()
            .iter()
            .filter_map(|(_, enrollment)| {
                if enrollment.user_id == user_id {
                    Some(enrollment)
                } else {
                    None
                }
            })
            .collect()
    })
}

#[update]
#[candid_method(update)]
async fn mark_lesson_complete(course_id: String, lesson_id: String) -> Result<Enrollment> {
    let caller_id = caller();
    let enrollment_id = enrollment_key(&caller_id, &course_id);

    ENROLLMENTS.with(|enrollments| {
        let mut enrollments = enrollments.borrow_mut();
        match enrollments.get(&enrollment_id) {
            Some(mut enrollment) => {
                if !enrollment.progress.completed_lessons.contains(&lesson_id) {
                    enrollment.progress.completed_lessons.push(lesson_id);
                    enrollment.last_accessed = get_current_time();
                    
                    // Calculate completion percentage
                    let course = get_course(course_id)?;
                    if !course.lessons.is_empty() {
                        enrollment.completion_percentage = 
                            (enrollment.progress.completed_lessons.len() as f32) / 
                            (course.lessons.len() as f32) * 100.0;
                    }
                }
                enrollments.insert(enrollment_id, enrollment.clone());
                Ok(enrollment)
            }
            None => Err(ApiError::NotFound("Enrollment not found".to_string()))
        }
    })
}

#[query]
#[candid_method(query)]
fn search_courses(
    query: Option<String>,
    category: Option<String>,
    difficulty: Option<DifficultyLevel>,
    limit: Option<u32>,
) -> Vec<Course> {
    let limit = limit.unwrap_or(10).min(100) as usize;
    
    COURSES.with(|courses| {
        let mut filtered_courses: Vec<Course> = courses.borrow()
            .iter()
            .filter_map(|(_, course)| {
                if !course.is_published {
                    return None;
                }

                if let Some(ref q) = query {
                    let q_lower = q.to_lowercase();
                    if !course.title.to_lowercase().contains(&q_lower) &&
                       !course.description.to_lowercase().contains(&q_lower) &&
                       !course.tags.iter().any(|tag| tag.to_lowercase().contains(&q_lower)) {
                        return None;
                    }
                }

                if let Some(ref cat) = category {
                    if course.category != *cat {
                        return None;
                    }
                }

                if let Some(ref diff) = difficulty {
                    if course.difficulty_level != *diff {
                        return None;
                    }
                }

                Some(course)
            })
            .collect();

        // Sort by rating and enrollment count
        filtered_courses.sort_by(|a, b| {
            b.rating.partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.enrollment_count.cmp(&a.enrollment_count))
        });

        filtered_courses.truncate(limit);
        filtered_courses
    })
}

#[update]
#[candid_method(update)]
async fn add_course_review(course_id: String, rating: u8, comment: String) -> Result<Course> {
    let caller_id = caller();

    if rating < 1 || rating > 5 {
        return Err(ApiError::InvalidInput("Rating must be between 1 and 5".to_string()));
    }

    // Check if user is enrolled
    let enrollment_id = enrollment_key(&caller_id, &course_id);
    if !ENROLLMENTS.with(|enrollments| enrollments.borrow().contains_key(&enrollment_id)) {
        return Err(ApiError::InsufficientPermissions);
    }

    COURSES.with(|courses| {
        let mut courses = courses.borrow_mut();
        match courses.get(&course_id) {
            Some(mut course) => {
                // Check if user already reviewed
                if course.reviews.iter().any(|review| review.user_id == caller_id) {
                    return Err(ApiError::AlreadyExists("Already reviewed this course".to_string()));
                }

                let review = Review {
                    id: format!("review_{}_{}", caller_id.to_text(), get_current_time()),
                    user_id: caller_id,
                    rating,
                    comment,
                    created_at: get_current_time(),
                    helpful_votes: 0,
                };

                course.reviews.push(review);
                
                // Recalculate average rating
                let total_rating: u32 = course.reviews.iter().map(|r| r.rating as u32).sum();
                course.rating = total_rating as f32 / course.reviews.len() as f32;
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
fn get_popular_courses(limit: Option<u32>) -> Vec<Course> {
    let limit = limit.unwrap_or(10).min(100) as usize;
    
    COURSES.with(|courses| {
        let mut published_courses: Vec<Course> = courses.borrow()
            .iter()
            .filter_map(|(_, course)| {
                if course.is_published {
                    Some(course)
                } else {
                    None
                }
            })
            .collect();

        published_courses.sort_by(|a, b| {
            b.enrollment_count.cmp(&a.enrollment_count)
                .then(b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal))
        });

        published_courses.truncate(limit);
        published_courses
    })
}

#[query]
#[candid_method(query)]
fn get_instructor_courses(instructor_id: Principal) -> Vec<Course> {
    COURSES.with(|courses| {
        courses.borrow()
            .iter()
            .filter_map(|(_, course)| {
                if course.instructor_id == instructor_id {
                    Some(course)
                } else {
                    None
                }
            })
            .collect()
    })
}

export_candid!();
