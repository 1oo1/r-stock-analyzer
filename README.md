# AI-Powered Stock Analyzer Backend

This is the backend implementation for the AI-Powered Stock Analyzer, featuring user management and LLM provider management as specified in the PRD.

## Features Implemented

### 4.1 User Management
- ✅ User Registration with email and password
- ✅ User Login with JWT authentication
- ✅ Admin role support
- ✅ Secure password hashing using bcrypt
- ✅ JWT token generation and validation

### 4.4 LLM Provider Management (Admin Only)
- ✅ Add/Create LLM providers (OpenAI, Gemini, Anthropic, etc.)
- ✅ List all LLM providers
- ✅ Get specific LLM provider details
- ✅ Update LLM provider configurations
- ✅ Delete LLM providers
- ✅ API key encryption (basic implementation)
- ✅ Usage tracking structure

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login and get JWT token

### User Management
- `GET /api/user/me` - Get current user info (requires authentication)
- `GET /api/admin/users` - List all users (admin only)

### LLM Provider Management (Admin Only)
- `POST /api/admin/llm-providers` - Create a new LLM provider
- `GET /api/admin/llm-providers` - List all LLM providers
- `GET /api/admin/llm-providers/:id` - Get specific provider
- `PUT /api/admin/llm-providers/:id` - Update provider
- `DELETE /api/admin/llm-providers/:id` - Delete provider
- `GET /api/admin/llm-usage-stats` - Get usage statistics

## Technology Stack

- **Framework**: Axum (Rust web framework)
- **Database**: PostgreSQL with Diesel ORM
- **Authentication**: JWT tokens with bcrypt password hashing
- **Async Runtime**: Tokio

## Setup Instructions

### Prerequisites
- Rust (latest stable version)
- PostgreSQL database
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

### Environment Setup

1. Create a PostgreSQL database:
```sql
CREATE DATABASE r_stock_analyzer;
```

2. Copy the environment file and configure:
```bash
cp .env.example .env
```

3. Update `.env` with your database credentials:
```env
DATABASE_URL=postgresql://username:password@localhost/r_stock_analyzer
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
RUST_LOG=debug
```

### Database Setup

1. Run migrations:
```bash
diesel migration run
```

### Running the Application

1. Install dependencies and run:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## Database Schema

### Users Table
- `id` (UUID, Primary Key)
- `username` (VARCHAR, Unique)
- `email` (VARCHAR, Unique)
- `password_hash` (VARCHAR)
- `role` (VARCHAR, default: 'user')
- `is_active` (BOOLEAN, default: true)
- `created_at` (TIMESTAMP)
- `updated_at` (TIMESTAMP)

### LLM Providers Table
- `id` (UUID, Primary Key)
- `name` (VARCHAR, Unique)
- `provider_type` (VARCHAR) - 'openai', 'gemini', 'anthropic', etc.
- `api_key_encrypted` (TEXT)
- `api_endpoint` (VARCHAR, Optional)
- `model_name` (VARCHAR, Optional)
- `is_active` (BOOLEAN, default: true)
- `created_at` (TIMESTAMP)
- `updated_at` (TIMESTAMP)

### LLM Usage Table
- `id` (UUID, Primary Key)
- `provider_id` (UUID, Foreign Key)
- `user_id` (UUID, Foreign Key)
- `tokens_used` (INTEGER)
- `cost` (DECIMAL, Optional)
- `request_type` (VARCHAR)
- `created_at` (TIMESTAMP)

## Security Features

1. **Password Security**: Passwords are hashed using bcrypt with default cost
2. **JWT Authentication**: Secure token-based authentication with configurable secret
3. **API Key Protection**: LLM provider API keys are encrypted before storage
4. **Role-based Access**: Admin-only endpoints for sensitive operations
5. **Input Validation**: Request validation and sanitization

## Example API Usage

### Register a User
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "securepassword"
  }'
```

### Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "securepassword"
  }'
```

### Create LLM Provider (Admin Only)
```bash
curl -X POST http://localhost:3000/api/admin/llm-providers \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "name": "OpenAI GPT-4",
    "provider_type": "openai",
    "api_key": "sk-your-openai-key",
    "api_endpoint": "https://api.openai.com/v1",
    "model_name": "gpt-4"
  }'
```

## Development Notes

### Creating an Admin User
After registering a user, you can manually promote them to admin by updating the database:
```sql
UPDATE users SET role = 'admin' WHERE username = 'your_username';
```

### API Key Encryption
The current implementation uses basic base64 encoding. For production, implement proper encryption using libraries like `ring` or `aes-gcm`.

### Error Handling
All endpoints return appropriate HTTP status codes:
- `200` - Success
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `409` - Conflict (user already exists)
- `500` - Internal Server Error

## Next Steps

1. Implement proper API key encryption
2. Add input validation and sanitization
3. Implement rate limiting
4. Add comprehensive logging
5. Create integration with MCP server
6. Add stock analysis endpoints
7. Implement LLM provider health checks
8. Add API documentation with OpenAPI/Swagger
