import { html, render } from 'lit-html';
import { user_management } from 'declarations/user_management';
import { course_management } from 'declarations/course_management';
import { certification_system } from 'declarations/certification_system';
import { governance } from 'declarations/governance';
import logo from './logo2.svg';

class App {
  userCount = 0;
  courseCount = 0;
  certificationCount = 0;
  proposalCount = 0;
  users = [];
  courses = [];
  certifications = [];
  proposals = [];
  status = 'Loading platform statistics...';
  currentView = 'dashboard';
  showModal = false;
  modalContent = '';

  constructor() {
    this.#loadStats();
    this.#render();
    this.#setupEventListeners();
  }

  #loadStats = async () => {
    try {
      // Load user statistics
      this.userCount = await user_management.get_user_count();
      
      // Load course statistics
      this.courses = await course_management.get_all_courses();
      this.courseCount = this.courses.length;
      
      // Load certification statistics
      this.certifications = await certification_system.get_all_certifications();
      this.certificationCount = this.certifications.length;
      
      // Load governance statistics (using active proposals since get_governance_stats doesn't exist)
      this.proposals = await governance.get_active_proposals(null);
      this.proposalCount = this.proposals.length;
      
      this.status = 'Platform statistics loaded successfully!';
    } catch (error) {
      console.error('Error loading stats:', error);
      this.status = `Error loading statistics: ${error.message}`;
    }
    this.#render();
  };

  #setupEventListeners = () => {
    // Event delegation for dynamic elements
    document.addEventListener('click', (e) => {
      if (e.target.matches('[data-view]')) {
        e.preventDefault();
        this.currentView = e.target.dataset.view;
        this.#render();
      }
      if (e.target.matches('[data-action]')) {
        e.preventDefault();
        this.#handleAction(e.target.dataset.action, e.target);
      }
      if (e.target.matches('.modal-backdrop, .close-modal')) {
        this.showModal = false;
        this.#render();
      }
    });

    // Form submission handler
    document.addEventListener('submit', (e) => {
      if (e.target.matches('.modal-form')) {
        e.preventDefault();
        this.#handleFormSubmission(e.target);
      }
    });
  };

  #handleFormSubmission = async (form) => {
    const formData = new FormData(form);
    const data = Object.fromEntries(formData.entries());
    
    try {
      if (this.modalContent === 'create-user') {
        await this.#createUser(data);
      } else if (this.modalContent === 'create-course') {
        await this.#createCourse(data);
      }
    } catch (error) {
      console.error('Form submission error:', error);
      this.status = `Error: ${error.message}`;
      this.#render();
    }
  };

  #createUser = async (data) => {
    this.status = 'Creating new user...';
    this.#render();
    
    const userRequest = {
      username: data.username,
      email: data.email,
      full_name: data.fullname,
      bio: data.bio ? [data.bio] : [],
      skills: data.skills ? data.skills.split(',').map(s => s.trim()) : []
    };
    
    const result = await user_management.create_user(userRequest);
    
    if ('Ok' in result) {
      this.status = `User "${userRequest.username}" created successfully!`;
      this.showModal = false;
      await this.#loadStats(); // Refresh stats
    } else {
      this.status = `Failed to create user: ${Object.keys(result.Err)[0]} - ${Object.values(result.Err)[0]}`;
    }
    
    this.#render();
  };

  #createCourse = async (data) => {
    this.status = 'Creating new course...';
    this.#render();
    
    const courseRequest = {
      title: data.title,
      description: data.description,
      category: data.category,
      tags: data.tags ? data.tags.split(',').map(s => s.trim()) : [],
      difficulty_level: { [data.difficulty]: null },
      estimated_duration_hours: parseInt(data.duration) || 1,
      price: 0
    };
    
    const result = await course_management.create_course(courseRequest);
    
    if ('Ok' in result) {
      this.status = `Course "${courseRequest.title}" created successfully!`;
      this.showModal = false;
      await this.#loadStats(); // Refresh stats
    } else {
      this.status = `Failed to create course: ${Object.keys(result.Err)[0]} - ${Object.values(result.Err)[0]}`;
    }
    
    this.#render();
  };

  #handleAction = async (action, element) => {
    switch (action) {
      case 'create-user':
        this.#showCreateUserModal();
        break;
      case 'create-course':
        this.#showCreateCourseModal();
        break;
      case 'refresh-stats':
        this.status = 'Refreshing statistics...';
        this.#render();
        await this.#loadStats();
        break;
      default:
        console.log('Unknown action:', action);
    }
  };

  #showCreateUserModal = () => {
    this.modalContent = 'create-user';
    this.showModal = true;
    this.#render();
  };

  #showCreateCourseModal = () => {
    this.modalContent = 'create-course';
    this.showModal = true;
    this.#render();
  };

  #renderDashboard() {
    return html`
      <div class="dashboard">
        <!-- Stats Cards -->
        <div class="stats-grid">
          <div class="stat-card users">
            <div class="stat-icon">👥</div>
            <div class="stat-info">
              <div class="stat-number">${this.userCount}</div>
              <div class="stat-label">Active Users</div>
            </div>
            <div class="stat-trend">+12% this month</div>
          </div>
          
          <div class="stat-card courses">
            <div class="stat-icon">📚</div>
            <div class="stat-info">
              <div class="stat-number">${this.courseCount}</div>
              <div class="stat-label">Live Courses</div>
            </div>
            <div class="stat-trend">+3 new today</div>
          </div>
          
          <div class="stat-card certificates">
            <div class="stat-icon">🏆</div>
            <div class="stat-info">
              <div class="stat-number">${this.certificationCount}</div>
              <div class="stat-label">Certificates Issued</div>
            </div>
            <div class="stat-trend">+${this.certificationCount} total</div>
          </div>
          
          <div class="stat-card proposals">
            <div class="stat-icon">🗳️</div>
            <div class="stat-info">
              <div class="stat-number">${this.proposalCount}</div>
              <div class="stat-label">Active Proposals</div>
            </div>
            <div class="stat-trend">Community voting</div>
          </div>
        </div>

        <!-- Quick Actions -->
        <div class="quick-actions">
          <h2>Quick Actions</h2>
          <div class="action-buttons">
            <button class="action-btn primary" data-action="create-user">
              <span class="btn-icon">👤</span>
              Create User Account
            </button>
            <button class="action-btn secondary" data-action="create-course">
              <span class="btn-icon">📖</span>
              Create New Course
            </button>
            <button class="action-btn tertiary" data-action="refresh-stats">
              <span class="btn-icon">🔄</span>
              Refresh Statistics
            </button>
          </div>
        </div>

        <!-- Recent Activity -->
        <div class="recent-activity">
          <h2>Recent Activity</h2>
          <div class="activity-list">
            <div class="activity-item">
              <div class="activity-icon user-activity">👤</div>
              <div class="activity-content">
                <div class="activity-title">New user registered</div>
                <div class="activity-time">2 minutes ago</div>
              </div>
            </div>
            <div class="activity-item">
              <div class="activity-icon course-activity">📚</div>
              <div class="activity-content">
                <div class="activity-title">Introduction to Blockchain course created</div>
                <div class="activity-time">5 minutes ago</div>
              </div>
            </div>
            <div class="activity-item">
              <div class="activity-icon system-activity">⚡</div>
              <div class="activity-content">
                <div class="activity-title">System initialization complete</div>
                <div class="activity-time">10 minutes ago</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    `;
  }

  #renderCourses() {
    return html`
      <div class="courses-view">
        <div class="view-header">
          <h2>Course Catalog</h2>
          <button class="btn primary" data-action="create-course">
            <span class="btn-icon">➕</span>
            Create Course
          </button>
        </div>
        
        ${this.courses.length === 0 ? html`
          <div class="empty-state">
            <div class="empty-icon">📚</div>
            <h3>No courses yet</h3>
            <p>Start building the future of education by creating the first course!</p>
            <button class="btn primary" data-action="create-course">Create First Course</button>
          </div>
        ` : html`
          <div class="courses-grid">
            ${this.courses.map(course => html`
              <div class="course-card">
                <div class="course-header">
                  <h3>${course.title || 'Untitled Course'}</h3>
                  <div class="course-difficulty ${course.difficulty_level?.Beginner !== undefined ? 'beginner' : 'intermediate'}">
                    ${course.difficulty_level?.Beginner !== undefined ? 'Beginner' : 
                      course.difficulty_level?.Intermediate !== undefined ? 'Intermediate' :
                      course.difficulty_level?.Advanced !== undefined ? 'Advanced' : 'Expert'}
                  </div>
                </div>
                <p class="course-description">${course.description || 'No description available'}</p>
                <div class="course-meta">
                  <span class="course-category">${course.category || 'General'}</span>
                  <span class="course-duration">${course.estimated_duration_hours || 0}h</span>
                </div>
                <div class="course-tags">
                  ${(course.tags || []).map(tag => html`<span class="tag">${tag}</span>`)}
                </div>
                <div class="course-status">
                  <span class="status-badge ${course.is_published ? 'published' : 'draft'}">
                    ${course.is_published ? '🟢 Published' : '🟡 Draft'}
                  </span>
                </div>
              </div>
            `)}
          </div>
        `}
      </div>
    `;
  }

  #renderUsers() {
    return html`
      <div class="users-view">
        <div class="view-header">
          <h2>User Management</h2>
          <button class="btn primary" data-action="create-user">
            <span class="btn-icon">👤</span>
            Add User
          </button>
        </div>
        
        <div class="users-stats">
          <div class="user-stat">
            <div class="stat-value">${this.userCount}</div>
            <div class="stat-label">Total Users</div>
          </div>
        </div>
        
        <div class="empty-state">
          <div class="empty-icon">👥</div>
          <h3>User directory coming soon</h3>
          <p>Detailed user management features will be available in the next update.</p>
        </div>
      </div>
    `;
  }

  #renderGovernance() {
    return html`
      <div class="governance-view">
        <div class="view-header">
          <h2>DAO Governance</h2>
          <button class="btn primary" disabled>
            <span class="btn-icon">📝</span>
            Create Proposal
          </button>
        </div>
        
        ${this.proposals.length === 0 ? html`
          <div class="empty-state">
            <div class="empty-icon">🗳️</div>
            <h3>No active proposals</h3>
            <p>The community hasn't created any proposals yet. Be the first to shape the future of our platform!</p>
          </div>
        ` : html`
          <div class="proposals-list">
            ${this.proposals.map(proposal => html`
              <div class="proposal-card">
                <h3>${proposal.title}</h3>
                <p>${proposal.description}</p>
                <div class="proposal-meta">
                  <span class="status ${proposal.status}">${proposal.status}</span>
                  <span class="votes">👍 ${proposal.votes_for} 👎 ${proposal.votes_against}</span>
                </div>
              </div>
            `)}
          </div>
        `}
      </div>
    `;
  }

  #renderModal() {
    if (!this.showModal) return '';
    
    let modalBody = '';
    if (this.modalContent === 'create-user') {
      modalBody = html`
        <h3>Create New User</h3>
        <form class="modal-form">
          <input type="text" name="username" placeholder="Username" required>
          <input type="email" name="email" placeholder="Email" required>
          <input type="text" name="fullname" placeholder="Full Name" required>
          <textarea name="bio" placeholder="Bio (optional)"></textarea>
          <input type="text" name="skills" placeholder="Skills (comma separated)">
          <button type="submit" class="btn primary">Create User</button>
        </form>
      `;
    } else if (this.modalContent === 'create-course') {
      modalBody = html`
        <h3>Create New Course</h3>
        <form class="modal-form">
          <input type="text" name="title" placeholder="Course Title" required>
          <textarea name="description" placeholder="Course Description" required></textarea>
          <input type="text" name="category" placeholder="Category" required>
          <input type="text" name="tags" placeholder="Tags (comma separated)">
          <select name="difficulty" required>
            <option value="">Select Difficulty Level</option>
            <option value="Beginner">Beginner</option>
            <option value="Intermediate">Intermediate</option>
            <option value="Advanced">Advanced</option>
            <option value="Expert">Expert</option>
          </select>
          <input type="number" name="duration" placeholder="Duration (hours)" min="1" required>
          <button type="submit" class="btn primary">Create Course</button>
        </form>
      `;
    }

    return html`
      <div class="modal-backdrop">
        <div class="modal">
          <button class="close-modal">×</button>
          ${modalBody}
        </div>
      </div>
    `;
  }

  #render() {
    const currentViewContent = () => {
      switch (this.currentView) {
        case 'courses': return this.#renderCourses();
        case 'users': return this.#renderUsers();
        case 'governance': return this.#renderGovernance();
        default: return this.#renderDashboard();
      }
    };

    let body = html`
      <div class="app">
        <!-- Navigation Header -->
        <nav class="navbar">
          <div class="nav-brand">
            <div class="logo">🎓</div>
            <div class="brand-text">
              <div class="brand-title">DAU Platform</div>
              <div class="brand-subtitle">Decentralized Autonomous University</div>
            </div>
          </div>
          
          <div class="nav-links">
            <a href="#" class="nav-link ${this.currentView === 'dashboard' ? 'active' : ''}" data-view="dashboard">
              <span class="nav-icon">📊</span>
              Dashboard
            </a>
            <a href="#" class="nav-link ${this.currentView === 'courses' ? 'active' : ''}" data-view="courses">
              <span class="nav-icon">📚</span>
              Courses
            </a>
            <a href="#" class="nav-link ${this.currentView === 'users' ? 'active' : ''}" data-view="users">
              <span class="nav-icon">👥</span>
              Users
            </a>
            <a href="#" class="nav-link ${this.currentView === 'governance' ? 'active' : ''}" data-view="governance">
              <span class="nav-icon">🗳️</span>
              Governance
            </a>
          </div>
          
          <div class="nav-status">
            <div class="status-indicator online"></div>
            <span class="status-text">Online</span>
          </div>
        </nav>

        <!-- Main Content -->
        <main class="main-content">
          ${currentViewContent()}
        </main>

        <!-- Status Bar -->
        <div class="status-bar">
          <div class="status-message">
            <span class="status-icon">ℹ️</span>
            ${this.status}
          </div>
          <div class="platform-info">
            <span class="platform-badge">Internet Computer</span>
            <span class="version">v1.0.0</span>
          </div>
        </div>

        <!-- Modal -->
        ${this.#renderModal()}
      </div>
    `;
    render(body, document.getElementById('root'));
  }
}

export default App;
