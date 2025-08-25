use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::{caller, export_candid, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

// Define simple types inline
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub proposer_id: Principal,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_deadline: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub minimum_threshold: u64,
    pub executed_at: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProposalType {
    CourseApproval { course_id: String },
    InstructorVerification { instructor_id: Principal },
    PlatformUpgrade { upgrade_details: String },
    TokenomicsChange { change_details: String },
    GovernanceParameter { parameter: String, new_value: String },
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Vote {
    pub proposal_id: u64,
    pub voter_id: Principal,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub voting_duration_days: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ApiError {
    NotFound(String),
    InvalidInput(String),
    InsufficientPermissions,
    AlreadyExists(String),
    InternalError(String),
}

type Result<T> = std::result::Result<T, ApiError>;

// In-memory storage
thread_local! {
    static PROPOSALS: RefCell<HashMap<u64, Proposal>> = RefCell::new(HashMap::new());
    static VOTES: RefCell<HashMap<String, Vote>> = RefCell::new(HashMap::new());
    static USER_VOTING_POWER: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static GOVERNANCE_CONFIG: RefCell<GovernanceConfig> = RefCell::new(GovernanceConfig::default());
    static PROPOSAL_COUNTER: RefCell<u64> = RefCell::new(0);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GovernanceConfig {
    pub minimum_proposal_threshold: u64,
    pub minimum_voting_threshold: u64,
    pub voting_period_days: u64,
    pub execution_delay_days: u64,
    pub proposal_fee: u64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            minimum_proposal_threshold: 1000, // Minimum voting power to create proposal
            minimum_voting_threshold: 10000, // Minimum total votes for proposal to pass
            voting_period_days: 7,
            execution_delay_days: 2,
            proposal_fee: 10, // Fee in tokens to create proposal (prevents spam)
        }
    }
}

// Helper functions
fn get_current_time() -> u64 {
    time()
}

fn days_to_nanoseconds(days: u64) -> u64 {
    days * 24 * 60 * 60 * 1_000_000_000
}

fn vote_key(proposal_id: u64, user_id: &Principal) -> String {
    format!("{}_{}", proposal_id, user_id.to_text())
}

fn calculate_voting_power(user_id: &Principal) -> u64 {
    // In a real implementation, this would calculate voting power based on:
    // - Reputation score
    // - Time in platform
    // - Certifications earned
    // - Tokens held
    // For now, we'll use a simple default
    USER_VOTING_POWER.with(|power| {
        power.borrow().get(&user_id.to_text()).copied().unwrap_or(100)
    })
}

async fn is_user_admin(user_id: Principal) -> bool {
    // This would be an actual inter-canister call to user_management
    false
}

// Governance Functions

#[update]
#[candid_method(update)]
async fn create_proposal(request: CreateProposalRequest) -> Result<Proposal> {
    let caller_id = caller();
    
    // Validate input
    if request.title.trim().is_empty() {
        return Err(ApiError::InvalidInput("Proposal title cannot be empty".to_string()));
    }
    
    if request.description.trim().is_empty() {
        return Err(ApiError::InvalidInput("Proposal description cannot be empty".to_string()));
    }

    // Check if user has enough voting power to create proposal
    let user_voting_power = calculate_voting_power(&caller_id);
    let config = GOVERNANCE_CONFIG.with(|config| config.borrow().clone());
    
    if user_voting_power < config.minimum_proposal_threshold {
        return Err(ApiError::InsufficientPermissions);
    }

    let proposal_id = PROPOSAL_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    });

    let current_time = get_current_time();
    let voting_deadline = current_time + days_to_nanoseconds(request.voting_duration_days);
    
    let proposal = Proposal {
        id: proposal_id,
        proposer_id: caller_id,
        title: request.title,
        description: request.description,
        proposal_type: request.proposal_type,
        status: ProposalStatus::Active,
        created_at: current_time,
        voting_deadline,
        votes_for: 0,
        votes_against: 0,
        votes_abstain: 0,
        minimum_threshold: config.minimum_voting_threshold,
        executed_at: None,
    };

    PROPOSALS.with(|proposals| proposals.borrow_mut().insert(proposal_id, proposal.clone()));
    Ok(proposal)
}

#[query]
#[candid_method(query)]
fn get_proposal(proposal_id: u64) -> Result<Proposal> {
    PROPOSALS.with(|proposals| {
        proposals.borrow().get(&proposal_id).cloned()
            .ok_or_else(|| ApiError::NotFound("Proposal not found".to_string()))
    })
}

#[update]
#[candid_method(update)]
async fn vote_on_proposal(
    proposal_id: u64,
    vote_type: VoteType,
) -> Result<Vote> {
    let caller_id = caller();
    
    // Get proposal and check if it exists and is active
    let mut proposal = get_proposal(proposal_id)?;
    if !matches!(proposal.status, ProposalStatus::Active) {
        return Err(ApiError::InvalidInput("Proposal is not active".to_string()));
    }
    
    // Check if voting period has ended
    let current_time = get_current_time();
    if current_time > proposal.voting_deadline {
        // Update proposal status if deadline passed
        proposal.status = if proposal.votes_for > proposal.votes_against && 
                             proposal.votes_for >= proposal.minimum_threshold {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };
        
        PROPOSALS.with(|proposals| proposals.borrow_mut().insert(proposal_id, proposal));
        return Err(ApiError::InvalidInput("Voting period has ended".to_string()));
    }

    let vote_key = vote_key(proposal_id, &caller_id);
    
    // Check if user already voted
    if VOTES.with(|votes| votes.borrow().contains_key(&vote_key)) {
        return Err(ApiError::AlreadyExists("User has already voted on this proposal".to_string()));
    }

    let voting_power = calculate_voting_power(&caller_id);
    
    let vote = Vote {
        proposal_id,
        voter_id: caller_id,
        vote_type: vote_type.clone(),
        voting_power,
        timestamp: current_time,
    };

    // Update proposal vote counts
    match vote_type {
        VoteType::For => proposal.votes_for += voting_power,
        VoteType::Against => proposal.votes_against += voting_power,
        VoteType::Abstain => proposal.votes_abstain += voting_power,
    }

    // Store vote and updated proposal
    VOTES.with(|votes| votes.borrow_mut().insert(vote_key, vote.clone()));
    PROPOSALS.with(|proposals| proposals.borrow_mut().insert(proposal_id, proposal));
    
    Ok(vote)
}

#[query]
#[candid_method(query)]
fn get_user_vote(proposal_id: u64, user_id: Principal) -> Option<Vote> {
    let vote_key = vote_key(proposal_id, &user_id);
    VOTES.with(|votes| votes.borrow().get(&vote_key).cloned())
}

#[update]
#[candid_method(update)]
async fn execute_proposal(proposal_id: u64) -> Result<bool> {
    let caller_id = caller();
    
    let mut proposal = get_proposal(proposal_id)?;
    
    // Check if proposal has passed
    if !matches!(proposal.status, ProposalStatus::Passed) {
        return Err(ApiError::InvalidInput("Proposal has not passed".to_string()));
    }

    // Check if execution delay has passed
    let current_time = get_current_time();
    let config = GOVERNANCE_CONFIG.with(|config| config.borrow().clone());
    let execution_delay = days_to_nanoseconds(config.execution_delay_days);
    if current_time < (proposal.voting_deadline + execution_delay) {
        return Err(ApiError::InvalidInput("Execution delay period has not passed".to_string()));
    }

    // Execute the proposal based on its type
    let execution_successful = match &proposal.proposal_type {
        ProposalType::CourseApproval { course_id } => {
            // Inter-canister call to approve course
            execute_course_approval(course_id.clone()).await
        },
        ProposalType::InstructorVerification { instructor_id } => {
            // Inter-canister call to verify instructor
            execute_instructor_verification(*instructor_id).await
        },
        ProposalType::PlatformUpgrade { upgrade_details: _ } => {
            // This would trigger a platform upgrade
            // For now, we'll just mark it as executed
            true
        },
        ProposalType::TokenomicsChange { change_details: _ } => {
            // This would update tokenomics parameters
            // For now, we'll just mark it as executed
            true
        },
        ProposalType::GovernanceParameter { parameter, new_value } => {
            execute_governance_parameter_change(parameter.clone(), new_value.clone()).await
        },
    };

    if execution_successful {
        proposal.status = ProposalStatus::Executed;
        proposal.executed_at = Some(current_time);
    }

    PROPOSALS.with(|proposals| proposals.borrow_mut().insert(proposal_id, proposal));
    Ok(execution_successful)
}

async fn execute_course_approval(course_id: String) -> bool {
    // Inter-canister call to course_management to approve/publish course
    true
}

async fn execute_instructor_verification(instructor_id: Principal) -> bool {
    // Inter-canister call to user_management to update user role to instructor
    true
}

async fn execute_governance_parameter_change(parameter: String, new_value: String) -> bool {
    match parameter.as_str() {
        "minimum_proposal_threshold" => {
            if let Ok(value) = new_value.parse::<u64>() {
                GOVERNANCE_CONFIG.with(|config| {
                    config.borrow_mut().minimum_proposal_threshold = value;
                });
                true
            } else {
                false
            }
        },
        "minimum_voting_threshold" => {
            if let Ok(value) = new_value.parse::<u64>() {
                GOVERNANCE_CONFIG.with(|config| {
                    config.borrow_mut().minimum_voting_threshold = value;
                });
                true
            } else {
                false
            }
        },
        "voting_period_days" => {
            if let Ok(value) = new_value.parse::<u64>() {
                GOVERNANCE_CONFIG.with(|config| {
                    config.borrow_mut().voting_period_days = value;
                });
                true
            } else {
                false
            }
        },
        _ => false,
    }
}

#[query]
#[candid_method(query)]
fn get_active_proposals(limit: Option<u32>) -> Vec<Proposal> {
    let limit = limit.unwrap_or(10).min(100) as usize;
    
    PROPOSALS.with(|proposals| {
        let mut active_proposals: Vec<Proposal> = proposals.borrow()
            .iter()
            .filter_map(|(_, proposal)| {
                if matches!(proposal.status, ProposalStatus::Active) {
                    Some(proposal.clone())
                } else {
                    None
                }
            })
            .collect();

        // Sort by creation time (newest first)
        active_proposals.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        active_proposals.truncate(limit);
        active_proposals
    })
}

#[query]
#[candid_method(query)]
fn get_proposal_history(
    status: Option<ProposalStatus>,
    limit: Option<u32>,
) -> Vec<Proposal> {
    let limit = limit.unwrap_or(10).min(100) as usize;
    
    PROPOSALS.with(|proposals| {
        let mut filtered_proposals: Vec<Proposal> = proposals.borrow()
            .iter()
            .filter_map(|(_, proposal)| {
                if let Some(ref filter_status) = status {
                    if proposal.status != *filter_status {
                        return None;
                    }
                }
                Some(proposal.clone())
            })
            .collect();

        // Sort by creation time (newest first)
        filtered_proposals.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        filtered_proposals.truncate(limit);
        filtered_proposals
    })
}

#[query]
#[candid_method(query)]
fn get_user_proposals(user_id: Principal) -> Vec<Proposal> {
    PROPOSALS.with(|proposals| {
        proposals.borrow()
            .iter()
            .filter_map(|(_, proposal)| {
                if proposal.proposer_id == user_id {
                    Some(proposal.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
#[candid_method(query)]
fn get_proposal_votes(proposal_id: u64) -> Vec<Vote> {
    VOTES.with(|votes| {
        votes.borrow()
            .iter()
            .filter_map(|(_, vote)| {
                if vote.proposal_id == proposal_id {
                    Some(vote.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[update]
#[candid_method(update)]
async fn update_user_voting_power(user_id: Principal, new_power: u64) -> Result<bool> {
    let caller_id = caller();
    
    // Check if caller is admin
    if !is_user_admin(caller_id).await {
        return Err(ApiError::InsufficientPermissions);
    }

    USER_VOTING_POWER.with(|power| {
        power.borrow_mut().insert(user_id.to_text(), new_power);
    });

    Ok(true)
}

#[query]
#[candid_method(query)]
fn get_user_voting_power(user_id: Principal) -> u64 {
    calculate_voting_power(&user_id)
}

#[query]
#[candid_method(query)]
fn get_governance_stats() -> GovernanceStats {
    let (total_proposals, active_proposals, executed_proposals) = 
        PROPOSALS.with(|proposals| {
            let mut total = 0u64;
            let mut active = 0u64;
            let mut executed = 0u64;

            for (_, proposal) in proposals.borrow().iter() {
                total += 1;
                match proposal.status {
                    ProposalStatus::Active => active += 1,
                    ProposalStatus::Executed => executed += 1,
                    _ => {}
                }
            }

            (total, active, executed)
        });

    let total_votes = VOTES.with(|votes| votes.borrow().len());
    
    let config = GOVERNANCE_CONFIG.with(|config| config.borrow().clone());

    GovernanceStats {
        total_proposals,
        active_proposals,
        executed_proposals,
        total_votes: total_votes as u64,
        governance_config: config,
    }
}

#[update]
#[candid_method(update)]
async fn update_governance_config(new_config: GovernanceConfig) -> Result<bool> {
    let caller_id = caller();
    
    // Check if caller is admin
    if !is_user_admin(caller_id).await {
        return Err(ApiError::InsufficientPermissions);
    }

    GOVERNANCE_CONFIG.with(|config| {
        *config.borrow_mut() = new_config;
    });

    Ok(true)
}

// Additional types
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GovernanceStats {
    pub total_proposals: u64,
    pub active_proposals: u64,
    pub executed_proposals: u64,
    pub total_votes: u64,
    pub governance_config: GovernanceConfig,
}


export_candid!();
