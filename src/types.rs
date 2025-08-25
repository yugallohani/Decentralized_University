// Types will be imported by each canister individually

pub type UserId = Principal;
pub type CourseId = String;
pub type LessonId = String;
pub type CertificationId = String;
pub type ProposalId = u64;
pub type Timestamp = u64;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum UserRole {
    Student,
    Instructor,
    Admin,
    Moderator,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub reputation_score: u32,
    pub skills: Vec<String>,
    pub achievements: Vec<Achievement>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub earned_at: Timestamp,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Course {
    pub id: CourseId,
    pub title: String,
    pub description: String,
    pub instructor_id: UserId,
    pub category: String,
    pub tags: Vec<String>,
    pub difficulty_level: DifficultyLevel,
    pub estimated_duration_hours: u32,
    pub price: u64, // in tokens
    pub thumbnail_url: Option<String>,
    pub lessons: Vec<LessonId>,
    pub prerequisites: Vec<CourseId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub enrollment_count: u32,
    pub rating: f32,
    pub reviews: Vec<Review>,
    pub is_published: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Lesson {
    pub id: LessonId,
    pub course_id: CourseId,
    pub title: String,
    pub description: String,
    pub content_type: ContentType,
    pub content_url: String,
    pub duration_minutes: u32,
    pub order_index: u32,
    pub prerequisites: Vec<LessonId>,
    pub learning_objectives: Vec<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
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
pub struct Review {
    pub id: String,
    pub user_id: UserId,
    pub rating: u8, // 1-5
    pub comment: String,
    pub created_at: Timestamp,
    pub helpful_votes: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Enrollment {
    pub user_id: UserId,
    pub course_id: CourseId,
    pub enrolled_at: Timestamp,
    pub progress: CourseProgress,
    pub completion_percentage: f32,
    pub last_accessed: Timestamp,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CourseProgress {
    pub completed_lessons: Vec<LessonId>,
    pub quiz_scores: HashMap<LessonId, u8>,
    pub assignment_submissions: HashMap<LessonId, AssignmentSubmission>,
    pub time_spent_minutes: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AssignmentSubmission {
    pub content: String,
    pub submitted_at: Timestamp,
    pub grade: Option<u8>,
    pub feedback: Option<String>,
    pub graded_by: Option<UserId>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct LearningPath {
    pub id: String,
    pub user_id: UserId,
    pub title: String,
    pub description: String,
    pub recommended_courses: Vec<CourseRecommendation>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub ai_generated: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CourseRecommendation {
    pub course_id: CourseId,
    pub reason: String,
    pub priority_score: f32,
    pub estimated_completion_time: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Certification {
    pub id: CertificationId,
    pub user_id: UserId,
    pub course_id: CourseId,
    pub title: String,
    pub description: String,
    pub issuer: String,
    pub issued_at: Timestamp,
    pub expires_at: Option<Timestamp>,
    pub verification_hash: String,
    pub metadata: CertificationMetadata,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CertificationMetadata {
    pub skills_acquired: Vec<String>,
    pub final_score: u8,
    pub completion_time_hours: u32,
    pub blockchain_proof: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DiscussionThread {
    pub id: String,
    pub course_id: Option<CourseId>,
    pub lesson_id: Option<LessonId>,
    pub author_id: UserId,
    pub title: String,
    pub content: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub replies: Vec<ThreadReply>,
    pub tags: Vec<String>,
    pub upvotes: u32,
    pub downvotes: u32,
    pub is_pinned: bool,
    pub is_locked: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ThreadReply {
    pub id: String,
    pub author_id: UserId,
    pub content: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub upvotes: u32,
    pub downvotes: u32,
    pub parent_reply_id: Option<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Proposal {
    pub id: ProposalId,
    pub proposer_id: UserId,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: Timestamp,
    pub voting_deadline: Timestamp,
    pub execution_delay: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub minimum_threshold: u64,
    pub executed_at: Option<Timestamp>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ProposalType {
    CourseApproval { course_id: CourseId },
    InstructorVerification { instructor_id: UserId },
    PlatformUpgrade { upgrade_details: String },
    TokenomicsChange { change_details: String },
    GovernanceParameter { parameter: String, new_value: String },
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Vote {
    pub proposal_id: ProposalId,
    pub voter_id: UserId,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

// API Result types
pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    InvalidInput(String),
    InternalError(String),
    AlreadyExists(String),
    InsufficientPermissions,
    QuotaExceeded,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ApiError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            ApiError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            ApiError::QuotaExceeded => write!(f, "Quota exceeded"),
        }
    }
}

// Request/Response types for API calls
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub bio: Option<String>,
    pub skills: Vec<String>,
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
    pub prerequisites: Vec<CourseId>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateLessonRequest {
    pub course_id: CourseId,
    pub title: String,
    pub description: String,
    pub content_type: ContentType,
    pub content_url: String,
    pub duration_minutes: u32,
    pub prerequisites: Vec<LessonId>,
    pub learning_objectives: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub voting_duration_days: u64,
}
