# 🎓 Decentralized Autonomous University (DAU)

[![Internet Computer](https://img.shields.io/badge/Internet%20Computer-Protocol-blue)](https://internetcomputer.org/)
[![Rust](https://img.shields.io/badge/Rust-Backend-orange)](https://www.rust-lang.org/)
[![JavaScript](https://img.shields.io/badge/JavaScript-Frontend-yellow)](https://developer.mozilla.org/en-US/docs/Web/JavaScript)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

> An open, decentralized platform on the Internet Computer Protocol (ICP) for education, where courses are created and taught by the community, leveraging blockchain for certifications and DAO governance.

## 🌐 Live Demo
**Frontend**: Local development at `http://{canister-id}.localhost:4943/`

## ✨ Features

### 🚀 **Completed Implementation**
- **4 Rust Backend Canisters**: Fully functional modular architecture
- **Modern Responsive Frontend**: Beautiful glassmorphism design with animations
- **Real-time Statistics**: Live updating dashboard with platform metrics
- **Interactive UI**: Navigation, forms, modals, and course catalog

### 🏗 **Core Platform Components**
- **👥 User Management**: Registration, profiles, roles, and reputation tracking
- **📚 Course Management**: Course creation, publishing, catalog, and search
- **🏆 Certification System**: Blockchain certificate issuance and verification
- **🗳️ DAO Governance**: Proposal creation, community voting, and execution

## 🏗 Architecture

The platform is built as a collection of interconnected canisters on the Internet Computer:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  User Management│    │Course Management│    │ Certification   │
│                 │    │                 │    │    System       │
│ • Registration  │    │ • Course CRUD   │    │ • Issue Certs   │
│ • Profiles      │◄──►│ • Enrollments   │◄──►│ • Verification  │
│ • Roles & Perms │    │ • Progress      │    │ • NFT-like      │
│ • Reputation    │    │ • Reviews       │    │ • Blockchain    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         ▲                        ▲                        ▲
         │                        │                        │
         └────────────────────────┼────────────────────────┘
                                  │
                    ┌─────────────────┐
                    │   Governance    │
                    │                 │
                    │ • Proposals     │
                    │ • Voting        │
                    │ • Execution     │
                    │ • DAO Logic     │
                    └─────────────────┘
```

## 🚀 Getting Started

### Prerequisites
- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/) (DFINITY SDK)
- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (v16+)
- [npm](https://www.npmjs.com/) (v7+)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yugallohani/Decentralized_University.git
   cd Decentralized_University
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Start the local replica**
   ```bash
   dfx start --background
   ```

4. **Deploy the canisters**
   ```bash
   dfx deploy
   ```

5. **Access the application**
   - Frontend: `http://{canister-id}.localhost:4943/`
   - Backend Candid interfaces available via DFX

### 🔧 Development Commands

```bash
# Development workflow
dfx start --background    # Start local replica
dfx deploy               # Deploy all canisters
dfx canister status --all # Check canister status

# Frontend development
npm start               # Start development server
npm run build          # Build frontend assets

# Backend development
dfx generate           # Generate Candid interfaces
cargo test            # Run Rust tests
```

## 🎨 Frontend Features

### Modern Design System
- **Glassmorphism UI**: Translucent cards with backdrop blur effects
- **Gradient Backgrounds**: Beautiful color transitions and animations
- **Responsive Layout**: Mobile-first design approach
- **Smooth Animations**: Hover effects and micro-interactions

### Interactive Components
- **Multi-tab Navigation**: Dashboard, Courses, Users, Governance
- **Modal Forms**: User and course creation with validation
- **Course Catalog**: Interactive cards with status badges
- **Real-time Statistics**: Live updating counters and metrics

## 🔧 Backend Architecture

### Canister Overview

#### User Management (`user_management`)
```rust
// Core API functions
create_user(request: CreateUserRequest) -> Result<User>
get_user_count() -> u64
get_user(user_id: Principal) -> Result<User>
update_user_profile(...) -> Result<User>
```

#### Course Management (`course_management`)
```rust
// Core API functions  
create_course(request: CreateCourseRequest) -> Result<Course>
get_all_courses() -> Vec<Course>
publish_course(course_id: String) -> Result<Course>
search_courses(...) -> Vec<Course>
```

#### Certification System (`certification_system`)
```rust
// Core API functions
issue_certification(...) -> Result<Certification>
verify_certification(cert_id: String) -> Result<bool>
get_user_certifications(user_id: Principal) -> Vec<Certification>
```

#### Governance (`governance`)
```rust
// Core API functions
create_proposal(request: CreateProposalRequest) -> Result<Proposal>
vote_on_proposal(proposal_id: u64, vote_type: VoteType) -> Result<Vote>
get_active_proposals(limit: Option<u32>) -> Vec<Proposal>
```

## 📊 Current Platform Stats

- **👥 Users**: 1 registered user
- **📚 Courses**: 2 courses (1 published, 1 draft)
  - "Introduction to Web3" (Published, Beginner, 15h)
  - "Smart Contract Development" (Draft, Advanced, 25h)
- **🏆 Certificates**: 0 issued (system ready)
- **🗳️ Proposals**: 0 active (governance ready)

## 🛣️ Roadmap

### ✅ Phase 1: Foundation (Complete)
- [x] 4 core Rust canisters implemented
- [x] Modern responsive frontend
- [x] User registration and management
- [x] Course creation and catalog
- [x] Basic governance structure
- [x] Real-time statistics dashboard

### 🚧 Phase 2: Enhancement (Next)
- [ ] Course content system (lessons, quizzes)
- [ ] Certificate verification portal  
- [ ] Advanced user profiles and achievements
- [ ] Enhanced governance features

### 📋 Phase 3: Scale (Future)
- [ ] AI-powered learning recommendations
- [ ] Mobile application
- [ ] Multi-language support
- [ ] Integration with external platforms

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

**Developer**: Yugal Lohani  
**Repository**: [https://github.com/yugallohani/Decentralized_University](https://github.com/yugallohani/Decentralized_University)

---

<div align="center">

**Built with ❤️ for the decentralized future of education**

[Internet Computer](https://internetcomputer.org/) | [DFX Documentation](https://internetcomputer.org/docs/current/references/cli-reference/dfx-parent) | [Rust Lang](https://www.rust-lang.org/)

</div>
