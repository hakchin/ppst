# Tasks: Academy Homepage

**Feature**: 001-homepage
**Input**: Design documents from `/specs/001-homepage/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Initialize Rust project with Cargo.toml and add dependencies: axum, tokio, askama, serde, serde_json, chrono, tower, tower-http
- [X] T002 Create project directory structure: src/{handlers/, models/, storage/, templates/}, static/{css/, js/}, data/contacts/, tests/{integration/, unit/}
- [X] T003 [P] Install Tailwind CLI standalone binary using system package manager (brew install tailwindcss or apt install tailwindcss)
- [X] T004 [P] Configure tailwind.config.js to scan src/templates/ directory for CSS classes
- [X] T005 [P] Download HTMX library and save to static/js/htmx.min.js (no CDN dependency)
- [X] T006 [P] Create data/contacts/.gitkeep to ensure directory exists in version control

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T007 Create main.rs with Axum server initialization, Tokio runtime setup, and server binding to localhost:3000
- [X] T008 Create src/routes.rs with Router setup and placeholder route registration
- [X] T009 Create src/handlers/mod.rs module declaration file
- [X] T010 Create src/models/mod.rs module declaration file
- [X] T011 Create src/storage/mod.rs module declaration file
- [X] T012 Create src/templates/base.html with HTML5 boilerplate, Tailwind CSS link, HTMX script tag, semantic structure (header, main, footer)
- [X] T013 [P] Configure Tower middleware for serving static files from static/ directory using ServeDir
- [X] T014 [P] Add tower-http compression layer for gzip/brotli support (optional but recommended)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - View Academy Information (Priority: P1) 🎯 MVP

**Goal**: Enable visitors to view academy information including mission, programs, and instructors on the homepage

**Independent Test**: Navigate to http://localhost:3000/ and verify all academy sections (mission, programs, instructors) are visible and readable

### Implementation for User Story 1

- [X] T015 [P] [US1] Create src/models/academy.rs with AcademyInfo, Program, and Instructor structs following data-model.md
- [X] T016 [P] [US1] Implement AcademyInfo::default() with hard-coded academy content (name, tagline, mission, programs, instructors)
- [X] T017 [US1] Create src/templates/homepage.html extending base.html with sections for mission, programs, instructors using Askama syntax
- [X] T018 [US1] Create src/templates/partials/header.html with academy logo, name, tagline, and navigation links to sections
- [X] T019 [P] [US1] Create src/templates/partials/mission.html with mission statement section using semantic HTML
- [X] T020 [P] [US1] Create src/templates/partials/programs.html with responsive CSS Grid for program cards (grid-cols-1 md:grid-cols-2 lg:grid-cols-3)
- [X] T021 [P] [US1] Create src/templates/partials/instructors.html with instructor cards including photos, names, credentials, and bios
- [X] T022 [US1] Create src/handlers/homepage.rs with GET / handler that renders HomepageTemplate with AcademyInfo data
- [X] T023 [US1] Register GET / route in src/routes.rs pointing to homepage handler
- [X] T024 [US1] Generate Tailwind CSS by running `tailwindcss -i src/input.css -o static/css/tailwind.css` (create src/input.css with @tailwind directives)
- [X] T025 [US1] Add responsive typography classes (text-base sm:text-lg md:text-xl) to all text content in templates
- [X] T026 [US1] Add WCAG 2.1 Level AA accessibility features: semantic HTML tags (header, nav, main, section), ARIA labels where needed, skip-to-content link

**Checkpoint**: At this point, User Story 1 should be fully functional - homepage displays all academy information with responsive layout

---

## Phase 4: User Story 2 - Submit Contact Inquiry (Priority: P2)

**Goal**: Enable visitors to submit contact inquiries via form with validation and success confirmation

**Independent Test**: Fill out contact form with valid data, submit, and verify success message appears and JSON file is created in data/contacts/

### Implementation for User Story 2

- [X] T027 [P] [US2] Create src/models/contact.rs with ContactInquiry and ContactFormInput structs per data-model.md
- [X] T028 [P] [US2] Implement ContactFormInput::validate() with email format validation, required field checks, and length limits
- [X] T029 [US2] Implement ContactFormInput::into_inquiry() method to convert form input to ContactInquiry with server-generated id, timestamp, IP, user-agent
- [X] T030 [US2] Create src/storage/file_store.rs with save_contact_inquiry() function that writes JSON files with timestamp-based filenames (YYYY-MM-DDTHH-MM-SS-mmmZ.json)
- [X] T031 [US2] Add helper function format_timestamp_id() to generate filesystem-safe timestamp IDs per data-model.md
- [X] T032 [US2] Add helper function is_valid_email() using regex for RFC 5322 subset email validation
- [X] T033 [US2] Create src/templates/partials/contact_form.html with HTML5 form (action="/contact", method="post") including name, email, subject, message fields with required attributes
- [X] T034 [US2] Add HTMX attributes to contact form: hx-post="/contact", hx-target="#contact-result", hx-indicator="#loading-spinner"
- [X] T035 [US2] Add client-side validation styling using Tailwind's invalid: and focus: pseudo-class utilities
- [X] T036 [US2] Create src/handlers/contact.rs with POST /contact handler that extracts Form data and HeaderMap
- [X] T037 [US2] Implement dual-mode response in contact handler: check HX-Request header to return either HTML fragment (HTMX) or 303 redirect (standard)
- [X] T038 [US2] Add validation error handling: return 400 status with error list as HTML (full page or fragment based on HX-Request)
- [X] T039 [US2] Add success response rendering: green success message div for HTMX, redirect to /?success=true for standard POST
- [X] T040 [US2] Implement in-memory rate limiting using HashMap<IpAddr, Instant> to track submissions and return 429 status if duplicate within 30 seconds
- [X] T041 [US2] Add file I/O error handling with 500 status and user-friendly error message
- [X] T042 [US2] Register POST /contact route in src/routes.rs pointing to contact handler
- [X] T043 [US2] Update homepage.html to include contact form partial and #contact-result div for HTMX responses
- [X] T044 [US2] Test form submission works without JavaScript (standard POST → redirect → homepage with success query param)
- [X] T045 [US2] Test form submission works with HTMX (AJAX POST → inline success message, no page reload)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - homepage displays info + contact form works with validation

---

## Phase 5: User Story 3 - Navigate Smoothly on Homepage (Priority: P3)

**Goal**: Ensure smooth navigation, responsive design, and accessibility across all devices

**Independent Test**: Access homepage from desktop, tablet, and mobile; test keyboard navigation; verify all sections are accessible via navigation links

### Implementation for User Story 3

- [X] T046 [P] [US3] Add anchor links to header navigation for smooth scrolling to sections (#mission, #programs, #instructors, #contact)
- [X] T047 [P] [US3] Ensure all sections have proper id attributes matching navigation anchor links
- [X] T048 [US3] Add responsive breakpoints to all layout components using Tailwind's sm:, md:, lg:, xl: prefixes
- [X] T049 [US3] Test mobile layout: single column stack, touch-friendly button sizes (min 44x44px), readable text without zoom
- [X] T050 [US3] Test tablet layout: 2-column grid for programs/instructors, appropriate spacing
- [X] T051 [US3] Test desktop layout: 3-column grid for programs/instructors, proper max-width container (max-w-7xl)
- [X] T052 [US3] Add keyboard navigation support: ensure all interactive elements (links, buttons, form inputs) are tab-accessible
- [X] T053 [US3] Add visible focus indicators using Tailwind's focus:ring-2 and focus:ring-offset-2 utilities
- [X] T054 [US3] Test keyboard form submission: Enter key submits form, Tab navigates through fields
- [X] T055 [US3] Verify screen reader compatibility: test with VoiceOver (macOS) or NVDA (Windows), ensure ARIA labels are announced correctly
- [X] T056 [US3] Add loading indicator for HTMX form submission: spinner div with hx-indicator class, shown during request
- [X] T057 [US3] Verify color contrast ratios meet WCAG 2.1 Level AA (4.5:1 for normal text, 3:1 for large text)

**Checkpoint**: All user stories should now be independently functional - homepage works on all devices with full accessibility

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T058 [P] Add HTML meta tags for SEO: description, keywords, og:tags for social sharing
- [X] T059 [P] Add favicon and touch icons for mobile devices in static/images/
- [X] T060 [P] Configure Tailwind CSS purging for production build (minimize CSS file size)
- [X] T061 [P] Add Cache-Control headers for static assets (1 hour cache, public)
- [X] T062 Code review: ensure all code follows Rust best practices, check for unwrap() usage, ensure proper error handling
- [X] T063 Add inline comments explaining HTMX attributes and Axum patterns per Educational Code Quality principle
- [X] T064 Run manual testing checklist from contracts/http-endpoints.md (all endpoints, validation, rate limiting, accessibility)
- [X] T065 Test with JavaScript disabled: verify all user stories work without JS (FR-015, SC-006)
- [X] T066 Performance testing: verify homepage loads in <3 seconds on broadband (SC-001)
- [X] T067 Browser compatibility testing: verify on Chrome, Firefox, Safari (SC-005)
- [X] T068 [P] Update quickstart.md if any implementation details changed from original design
- [X] T069 [P] Create data/contacts/.gitignore to exclude *.json files from version control (user privacy)
- [X] T070 Final constitution check: verify no Node.js/npm usage, Tailwind CLI standalone only, file-based storage, progressive enhancement

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1 (adds form to homepage) but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Enhances US1 and US2 but independently testable

### Within Each User Story

- Models before handlers/services
- Templates before handlers that use them
- Handlers before route registration
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T003-T006)
- Foundational tasks T013-T014 can run in parallel
- Within US1: T015-T016, T019-T021 can run in parallel
- Within US2: T027-T028, T036-T040 can be partially parallel (separate concerns)
- Within US3: T046-T047, all testing tasks can run in parallel
- Polish phase: T058-T061, T068-T069 can run in parallel

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T014) - CRITICAL
3. Complete Phase 3: User Story 1 (T015-T026)
4. **STOP and VALIDATE**: Test homepage loads with all academy information
5. Can deploy minimal homepage at this point

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → MVP homepage live!
3. Add User Story 2 → Test independently → Contact form added
4. Add User Story 3 → Test independently → Full responsive + accessible experience
5. Polish → Production ready

### Recommended Order

**For single developer**: Complete phases sequentially (P1 → P2 → P3)
**For team**: After Phase 2, assign US1 to Dev A, US2 to Dev B, US3 to Dev C

---

## Notes

- Tasks T001-T014 are blocking - complete before starting user stories
- Each user story is independently completable and testable
- [P] tasks can run in parallel (different files, no dependencies)
- Commit after each task or logical group
- Verify FR-016: Use Tailwind CLI standalone (brew/apt), no Node.js/npm
- Verify FR-015: Test all functionality with JavaScript disabled
- All file paths follow structure from plan.md
- Rate limiting is in-memory (transient, resets on server restart) - acceptable for promotional site
- Contact JSON files stored in data/contacts/ with timestamp-based filenames per clarifications

---

**Total Tasks**: 70
**Estimated Effort**: 4-6 hours for experienced Rust/Axum developer
**Ready for Implementation**: Yes ✅
