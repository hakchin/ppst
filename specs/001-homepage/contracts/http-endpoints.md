# HTTP Endpoints: Academy Homepage

**Feature**: 001-homepage
**Date**: 2025-10-30
**Protocol**: HTTP/1.1
**Base URL**: `http://localhost:3000` (development)

## Overview

This document defines the HTTP endpoints for the Academy Homepage feature. All endpoints follow RESTful conventions and support both standard browser requests and HTMX-enhanced requests.

---

## Endpoints

### 1. GET `/`

**Purpose**: Render the academy homepage

**Method**: `GET`

**URL**: `/`

**Query Parameters**: None

**Request Headers**:
- `Accept`: `text/html` (standard browser request)
- `User-Agent`: Browser identification (optional)

**Response**:

**Success (200 OK)**:
```http
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: [length]
Cache-Control: no-cache

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>PPST Academy - Home</title>
    <link rel="stylesheet" href="/static/css/tailwind.css">
</head>
<body>
    <!-- Full homepage content -->
    <header>...</header>
    <main>
        <section id="mission">...</section>
        <section id="programs">...</section>
        <section id="instructors">...</section>
        <section id="contact">
            <form action="/contact" method="post">...</form>
        </section>
    </main>
    <footer>...</footer>
    <script src="/static/js/htmx.min.js"></script>
</body>
</html>
```

**Error Responses**:
- `500 Internal Server Error`: Server error rendering template

**Functional Requirements Satisfied**:
- FR-001: Display academy name, logo, tagline
- FR-002: Include mission statement section
- FR-003: Display programs information
- FR-004: Include instructor section
- FR-005: Display contact form
- FR-012: Responsive layout
- FR-013: Navigation links to sections
- FR-014: Keyboard accessible
- FR-015: Functions without JavaScript

---

### 2. POST `/contact`

**Purpose**: Submit a contact form inquiry

**Method**: `POST`

**URL**: `/contact`

**Content-Type**: `application/x-www-form-urlencoded`

**Request Headers**:
- `Content-Type`: `application/x-www-form-urlencoded`
- `Content-Length`: [length]
- `HX-Request`: `true` (optional, present for HTMX requests)
- `User-Agent`: Browser identification (optional)

**Request Body** (form-encoded):
```
name=John+Doe&email=john%40example.com&subject=Question&message=Hello...
```

**Form Fields**:
| Field | Type | Required | Max Length | Validation |
|-------|------|----------|------------|------------|
| `name` | string | Yes | 100 chars | Non-empty after trim |
| `email` | string | Yes | 255 chars | Valid email format |
| `subject` | string | No | 200 chars | - |
| `message` | string | Yes | 5000 chars | Non-empty after trim |

**Responses**:

**Success - Standard Browser (303 See Other)**:
```http
HTTP/1.1 303 See Other
Location: /?success=true
Content-Length: 0
```

**Success - HTMX Request (200 OK)**:
```http
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: [length]

<div id="contact-result" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
    <p class="font-bold">Thank you for contacting us!</p>
    <p>We've received your message and will respond within 24 hours.</p>
</div>
```

**Validation Error - Standard Browser (400 Bad Request)**:
```http
HTTP/1.1 400 Bad Request
Content-Type: text/html; charset=utf-8
Content-Length: [length]

<!DOCTYPE html>
<html>
<!-- Full page with form pre-filled and error messages -->
<body>
    <form action="/contact" method="post">
        <div class="error">
            <ul>
                <li>Email format is invalid</li>
                <li>Message is required</li>
            </ul>
        </div>
        <input name="name" value="John Doe">
        <input name="email" value="invalid-email" class="border-red-500">
        <!-- ... -->
    </form>
</body>
</html>
```

**Validation Error - HTMX Request (400 Bad Request)**:
```http
HTTP/1.1 400 Bad Request
Content-Type: text/html; charset=utf-8
Content-Length: [length]

<div id="contact-result" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
    <p class="font-bold">Please correct the following errors:</p>
    <ul class="list-disc list-inside">
        <li>Email format is invalid</li>
        <li>Message is required</li>
    </ul>
</div>
```

**Rate Limit Error (429 Too Many Requests)**:
```http
HTTP/1.1 429 Too Many Requests
Content-Type: text/html; charset=utf-8
Retry-After: 30

<div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
    <p>You've submitted a form recently. Please wait 30 seconds before submitting again.</p>
</div>
```

**Server Error (500 Internal Server Error)**:
```http
HTTP/1.1 500 Internal Server Error
Content-Type: text/html; charset=utf-8

<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
    <p>An error occurred while processing your submission. Please try again later.</p>
</div>
```

**Functional Requirements Satisfied**:
- FR-006: Validate email format
- FR-007: Validate required fields
- FR-008: Display user-friendly error messages
- FR-009: Display success confirmation
- FR-010: Store submission data
- FR-011: Prevent duplicate submissions (rate limiting)
- FR-015: Works without JavaScript (standard POST)

---

### 3. GET `/static/*`

**Purpose**: Serve static assets (CSS, JS, images)

**Method**: `GET`

**URL Pattern**: `/static/{path}`

**Examples**:
- `/static/css/tailwind.css`
- `/static/js/htmx.min.js`
- `/static/images/logo.png`
- `/static/images/instructors/jane.jpg`

**Request Headers**:
- `Accept`: Varies by file type
- `If-None-Match`: ETag value (for caching)
- `If-Modified-Since`: Timestamp (for caching)

**Response**:

**Success (200 OK)**:
```http
HTTP/1.1 200 OK
Content-Type: [mime-type]
Content-Length: [length]
Cache-Control: public, max-age=3600
ETag: "[etag-value]"

[file contents]
```

**Not Found (404 Not Found)**:
```http
HTTP/1.1 404 Not Found
Content-Type: text/html

<h1>404 Not Found</h1>
```

**Supported MIME Types**:
| Extension | MIME Type |
|-----------|-----------|
| `.css` | `text/css` |
| `.js` | `application/javascript` |
| `.png` | `image/png` |
| `.jpg`, `.jpeg` | `image/jpeg` |
| `.svg` | `image/svg+xml` |
| `.ico` | `image/x-icon` |

---

## Request/Response Examples

### Example 1: Loading the Homepage

**Request**:
```http
GET / HTTP/1.1
Host: localhost:3000
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)
Accept: text/html,application/xhtml+xml
Accept-Language: en-US,en;q=0.9
```

**Response**:
```http
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 8423
Date: Wed, 30 Oct 2025 14:23:45 GMT

<!DOCTYPE html>
<html lang="en">
...
</html>
```

### Example 2: Successful Form Submission (Standard Browser)

**Request**:
```http
POST /contact HTTP/1.1
Host: localhost:3000
Content-Type: application/x-www-form-urlencoded
Content-Length: 98

name=Jane+Smith&email=jane%40example.com&subject=Enrollment+Question&message=I+am+interested+in...
```

**Response**:
```http
HTTP/1.1 303 See Other
Location: /?success=true
Date: Wed, 30 Oct 2025 14:25:10 GMT
Content-Length: 0
```

### Example 3: Form Submission with HTMX

**Request**:
```http
POST /contact HTTP/1.1
Host: localhost:3000
Content-Type: application/x-www-form-urlencoded
HX-Request: true
HX-Target: contact-result
Content-Length: 98

name=Jane+Smith&email=jane%40example.com&subject=Enrollment+Question&message=I+am+interested+in...
```

**Response**:
```http
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 245

<div id="contact-result" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
    <p class="font-bold">Thank you for contacting us!</p>
    <p>We've received your message and will respond within 24 hours.</p>
</div>
```

### Example 4: Validation Error (HTMX)

**Request**:
```http
POST /contact HTTP/1.1
Host: localhost:3000
Content-Type: application/x-www-form-urlencoded
HX-Request: true
Content-Length: 45

name=&email=invalid-email&message=
```

**Response**:
```http
HTTP/1.1 400 Bad Request
Content-Type: text/html; charset=utf-8
Content-Length: 312

<div id="contact-result" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
    <p class="font-bold">Please correct the following errors:</p>
    <ul class="list-disc list-inside">
        <li>Name is required</li>
        <li>Email format is invalid</li>
        <li>Message is required</li>
    </ul>
</div>
```

---

## Progressive Enhancement Strategy

### Base Layer (No JavaScript)

1. **Homepage rendering**: Standard HTTP GET request
2. **Form submission**: Standard HTTP POST → 303 redirect
3. **Navigation**: Anchor links with fragment identifiers (`#mission`, `#programs`, etc.)
4. **Validation**: HTML5 attributes (`required`, `type="email"`) + server-side

### Enhancement Layer (HTMX Enabled)

1. **Form submission**: AJAX POST with `hx-post` → inline HTML response (no page reload)
2. **Smooth scrolling**: Can be added with minimal Alpine.js if desired (optional)
3. **Loading indicators**: `hx-indicator` shows spinner during submission

**Detection**:
- Server detects HTMX via `HX-Request: true` header
- Response format changes based on presence of header

---

## Rate Limiting

**Strategy**: Simple in-memory rate limiter

**Implementation**:
- Track submissions by IP address
- Allow 1 submission per 30 seconds per IP
- Return 429 status code when rate limit exceeded

**Headers**:
- `Retry-After: 30` (seconds until retry allowed)

**Note**: This is a simple approach. For production, consider more sophisticated rate limiting (Redis-based, token bucket, etc.)

---

## Error Handling Summary

| Status Code | Scenario | Response Format |
|-------------|----------|-----------------|
| 200 OK | Successful GET / | Full HTML page |
| 200 OK | Successful POST /contact (HTMX) | HTML fragment (success message) |
| 303 See Other | Successful POST /contact (standard) | Redirect to /?success=true |
| 400 Bad Request | Validation errors | HTML with errors (full page or fragment) |
| 404 Not Found | Static asset not found | Simple error page |
| 429 Too Many Requests | Rate limit exceeded | HTML fragment with retry message |
| 500 Internal Server Error | Server error | HTML fragment with generic error message |

---

## Security Considerations

### Input Validation

- **Server-side validation**: Never trust client input
- **Email validation**: Basic regex, reject obvious invalid formats
- **Length limits**: Enforce maximum lengths on all fields
- **Sanitization**: Escape HTML in user input when displaying errors

### Rate Limiting

- **IP-based limiting**: Prevent form spam
- **30-second cooldown**: Balance user experience with abuse prevention

### Data Storage

- **File permissions**: Ensure contact JSON files are not web-accessible
- **No sensitive data**: Don't store passwords or payment info
- **Privacy**: Consider GDPR/privacy implications of storing email addresses

### CORS

- **Not applicable**: Single-origin application (no cross-origin requests expected)

### CSRF Protection

- **Optional for this feature**: Since there's no authentication/sessions, CSRF risk is minimal
- **Future consideration**: Add CSRF tokens when adding admin interface

---

## Testing Checklist

### Manual Testing

- [ ] GET / returns homepage with all sections
- [ ] Form submission works without JavaScript (standard POST → redirect)
- [ ] Form submission works with HTMX (AJAX POST → inline response)
- [ ] Validation errors display correctly (both standard and HTMX modes)
- [ ] Rate limiting prevents duplicate submissions
- [ ] Static assets load correctly (CSS, JS, images)
- [ ] Page works with JavaScript disabled
- [ ] Keyboard navigation works (tab through form, submit with Enter)
- [ ] Screen reader announces form errors correctly

### Automated Testing

- [ ] Integration test: GET / returns 200
- [ ] Integration test: Valid POST /contact returns 303 or 200
- [ ] Integration test: Invalid POST /contact returns 400 with errors
- [ ] Integration test: Rate limiting returns 429
- [ ] Unit test: Email validation function
- [ ] Unit test: Contact form validation
- [ ] Unit test: File storage operations

---

## Dependencies

**Axum**:
- `axum::Router` for routing
- `axum::extract::Form` for form data extraction
- `axum::response::{Html, Redirect, IntoResponse}` for responses
- `axum::http::StatusCode` for status codes

**Tower**:
- `tower_http::services::ServeDir` for static file serving
- `tower_http::compression::CompressionLayer` (optional)

**Askama**:
- Template rendering for HTML responses

---

## Summary

The HTTP endpoint design follows RESTful principles and supports progressive enhancement:

- **Base functionality**: Standard HTTP GET/POST work without JavaScript
- **Enhanced functionality**: HTMX provides better UX but is not required
- **Clear contracts**: Well-defined request/response formats
- **Error handling**: Comprehensive error responses with user-friendly messages
- **Security**: Input validation, rate limiting, safe file storage

All endpoints satisfy the functional requirements (FR-001 through FR-015) and constitutional principles (Server-Side First, Progressive Enhancement).
