 I'll also create some sample test users that you can insert into your database.


POSTMAN_API_TESTING_GUIDE.md

+234

@@ -1,0 +1,233 @@
# Postman API Testing Guide

## Base Setup

**Base URL:** `http://localhost:8888`

## Test Data Setup

First, you'll need some test users in your database. Run these SQL commands in your database:

```sql
-- Insert test users
INSERT INTO users (id, twitter_id, twitter_username, wallet_address, patron_status, selected_role, created_at) VALUES 
('550e8400-e29b-41d4-a716-446655440001', 'test_user_1', 'TestUser1', 'DjVE6JNiXqZF7wLvQKr8zQFJ2bFt9sF9N8vGjKLmNqX', 'none', 'none', NOW()),
('550e8400-e29b-41d4-a716-446655440002', 'test_user_2', 'TestUser2', NULL, 'applied', 'patron', NOW()),
('550e8400-e29b-41d4-a716-446655440003', 'test_user_3', 'TestUser3', 'FkVE6JNiXqZF7wLvQKr8zQFJ2bFt9sF9N8vGjKLmNqY', 'approved', 'staker', NOW()),
('550e8400-e29b-41d4-a716-446655440004', 'test_user_4', 'TestUser4', NULL, 'rejected', 'none', NOW()),
('550e8400-e29b-41d4-a716-446655440005', 'test_user_5', 'TestUser5', NULL, 'none', 'patron', NOW());

-- Insert some test tweets
INSERT INTO tweets (user_id, tweet_id, created_at, mining_phase) VALUES 
('550e8400-e29b-41d4-a716-446655440001', '1234567890', NOW() - INTERVAL '1 day', 1),
('550e8400-e29b-41d4-a716-446655440001', '1234567891', NOW() - INTERVAL '2 hours', 2),
('550e8400-e29b-41d4-a716-446655440002', '1234567892', NOW() - INTERVAL '1 hour', 2),
('550e8400-e29b-41d4-a716-446655440003', '1234567893', NOW() - INTERVAL '30 minutes', 1);
```

## API Endpoints Testing

### 1. Get User Profile by ID
```
GET /user/{user_id}
```

**Test Cases:**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440001`
- **Expected Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "username": "TestUser1",
  "wallet_address": "DjVE6JNiXqZF7wLvQKr8zQFJ2bFt9sF9N8vGjKLmNqX",
  "patron_status": "none",
  "selected_role": "none",
  "lock_duration_months": null,
  "locked_amount": null
}
```

### 2. Get User Mining Status
```
GET /user/{user_id}/mining_status
```

**Test Cases:**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440001/mining_status`
- **Expected Response:**
```json
{
  "phase1_mining_count": 1,
  "phase2_mining_count": 1,
  "total_mining_count": 2,
  "current_phase": "Phase2",
  "is_phase2": true
}
```

### 3. Update Patron Status
```
POST /user/{user_id}/patron_status
Content-Type: application/json
```

**Test Cases:**

**Test 1: Apply for Patron Status**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440001/patron_status`
- **Body:**
```json
{
  "patron_status": "applied"
}
```
- **Expected Response:**
```json
{
  "success": true
}
```

**Test 2: Invalid Status (should fail)**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440001/patron_status`
- **Body:**
```json
{
  "patron_status": "invalid_status"
}
```
- **Expected Response:** 400 Bad Request

### 4. Update User Role
```
POST /user/{user_id}/role
Content-Type: application/json
```

**Test Cases:**

**Test 1: Set Role to Patron**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440002/role`
- **Body:**
```json
{
  "selected_role": "patron"
}
```

**Test 2: Set Role to Staker**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440002/role`
- **Body:**
```json
{
  "selected_role": "staker"
}
```

### 5. Update Lock Details
```
POST /user/{user_id}/lock_details
Content-Type: application/json
```

**Test Cases:**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440003/lock_details`
- **Body:**
```json
{
  "lock_duration_months": 12,
  "locked_amount": 1000
}
```

### 6. Set Wallet Address
```
POST /user/{user_id}/wallet
Content-Type: application/json
```

**Test Cases:**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440005/wallet`
- **Body:**
```json
{
  "wallet_address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"
}
```

### 7. Get Phase 2 Tweets
```
GET /user/{user_id}/phase2_tweets
```

**Test Cases:**
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440001/phase2_tweets`
- **Expected Response:**
```json
[
  {
    "id": "...",
    "tweet_id": "1234567891",
    "created_at": "...",
    "phase": "Phase2"
  }
]
```

## Error Testing

### Test Invalid User ID
- **URL:** `http://localhost:8888/user/invalid-uuid`
- **Expected Response:** 400 Bad Request

### Test Non-existent User
- **URL:** `http://localhost:8888/user/550e8400-e29b-41d4-a716-446655440999`
- **Expected Response:** 404 Not Found

## Postman Collection Setup

1. **Create a new Collection** called "Snake AI Patron Framework"

2. **Set Collection Variables:**
   - `base_url`: `http://localhost:8888`
   - `test_user_1`: `550e8400-e29b-41d4-a716-446655440001`
   - `test_user_2`: `550e8400-e29b-41d4-a716-446655440002`
   - `test_user_3`: `550e8400-e29b-41d4-a716-446655440003`

3. **Add requests** for each endpoint using the collection variables:
   - `{{base_url}}/user/{{test_user_1}}`
   - `{{base_url}}/user/{{test_user_1}}/mining_status`
   - etc.

## Testing Workflow

1. **Setup Phase:**
   - Run the SQL commands to insert test data
   - Verify backend is running on localhost:8888

2. **Basic Tests:**
   - Test GET endpoints first
   - Verify data structure and content

3. **Update Tests:**
   - Test POST endpoints to modify data
   - Verify changes with subsequent GET requests

4. **Error Tests:**
   - Test with invalid UUIDs
   - Test with invalid request bodies
   - Test with non-existent users

5. **Integration Tests:**
   - Update patron status, then check profile
   - Set role, then verify in mining status
   - Add lock details, then verify in profile

## Verification

After each POST request, run the corresponding GET request to verify the changes were applied correctly.

Example workflow:
1. `GET /user/{user_id}` - Check initial state
2. `POST /user/{user_id}/patron_status` - Update patron status
3. `GET /user/{user_id}` - Verify update was applied
Now let me create a Postman collection file that you can import directly:


Snake_AI_Patron_Framework.postman_collection.json

+260

@@ -1,0 +1,259 @@
{
  "info": {
    "name": "Snake AI Patron Framework",
    "description": "API endpoints for testing the Patron Framework",
    "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
  },
  "variable": [
    {
      "key": "base_url",
      "value": "http://localhost:8888",
      "type": "string"
    },
    {
      "key": "test_user_1",
      "value": "550e8400-e29b-41d4-a716-446655440001",
      "type": "string"
    },
    {
      "key": "test_user_2",
      "value": "550e8400-e29b-41d4-a716-446655440002",
      "type": "string"
    },
    {
      "key": "test_user_3",
      "value": "550e8400-e29b-41d4-a716-446655440003",
      "type": "string"
    }
  ],
  "item": [
    {
      "name": "Get User Profile",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}"]
        }
      }
    },
    {
      "name": "Get User Mining Status",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/mining_status",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "mining_status"]
        }
      }
    },
    {
      "name": "Update Patron Status - Apply",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"patron_status\": \"applied\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/patron_status",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "patron_status"]
        }
      }
    },
    {
      "name": "Update Patron Status - Approve",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"patron_status\": \"approved\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_2}}/patron_status",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_2}}", "patron_status"]
        }
      }
    },
    {
      "name": "Update Patron Status - Reject",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"patron_status\": \"rejected\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_3}}/patron_status",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_3}}", "patron_status"]
        }
      }
    },
    {
      "name": "Update User Role - Patron",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"selected_role\": \"patron\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/role",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "role"]
        }
      }
    },
    {
      "name": "Update User Role - Staker",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"selected_role\": \"staker\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_2}}/role",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_2}}", "role"]
        }
      }
    },
    {
      "name": "Update Lock Details",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"lock_duration_months\": 12,\n  \"locked_amount\": 1000\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/lock_details",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "lock_details"]
        }
      }
    },
    {
      "name": "Set Wallet Address",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"wallet_address\": \"9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/wallet",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "wallet"]
        }
      }
    },
    {
      "name": "Get Phase 2 Tweets",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/phase2_tweets",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "phase2_tweets"]
        }
      }
    },
    {
      "name": "Error Test - Invalid UUID",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "{{base_url}}/user/invalid-uuid",
          "host": ["{{base_url}}"],
          "path": ["user", "invalid-uuid"]
        }
      }
    },
    {
      "name": "Error Test - Non-existent User",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "{{base_url}}/user/550e8400-e29b-41d4-a716-446655440999",
          "host": ["{{base_url}}"],
          "path": ["user", "550e8400-e29b-41d4-a716-446655440999"]
        }
      }
    },
    {
      "name": "Error Test - Invalid Patron Status",
      "request": {
        "method": "POST",
        "header": [
          {
            "key": "Content-Type",
            "value": "application/json"
          }
        ],
        "body": {
          "mode": "raw",
          "raw": "{\n  \"patron_status\": \"invalid_status\"\n}"
        },
        "url": {
          "raw": "{{base_url}}/user/{{test_user_1}}/patron_status",
          "host": ["{{base_url}}"],
          "path": ["user", "{{test_user_1}}", "patron_status"]
        }
      }
    }
  ]
}
Now let me create a SQL script to set up test data:


setup_test_data.sql

+35

@@ -1,0 +1,34 @@
-- Setup Test Data for Patron Framework API Testing

-- Insert test users with various states
INSERT INTO users (id, twitter_id, twitter_username, wallet_address, patron_status, selected_role, lock_duration_months, locked_amount, created_at) VALUES 
('550e8400-e29b-41d4-a716-446655440001', 'test_user_1', 'TestUser1', 'DjVE6JNiXqZF7wLvQKr8zQFJ2bFt9sF9N8vGjKLmNqX', 'none', 'none', NULL, NULL, NOW()),
('550e8400-e29b-41d4-a716-446655440002', 'test_user_2', 'TestUser2', NULL, 'applied', 'patron', 6, 500, NOW()),
('550e8400-e29b-41d4-a716-446655440003', 'test_user_3', 'TestUser3', 'FkVE6JNiXqZF7wLvQKr8zQFJ2bFt9sF9N8vGjKLmNqY', 'approved', 'staker', 12, 1000, NOW()),
('550e8400-e29b-41d4-a716-446655440004', 'test_user_4', 'TestUser4', NULL, 'rejected', 'none', NULL, NULL, NOW()),
('550e8400-e29b-41d4-a716-446655440005', 'test_user_5', 'TestUser5', NULL, 'none', 'patron', NULL, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- Insert some test tweets with different mining phases
INSERT INTO tweets (user_id, tweet_id, created_at, mining_phase) VALUES 
('550e8400-e29b-41d4-a716-446655440001', '1234567890', NOW() - INTERVAL '1 day', 1),
('550e8400-e29b-41d4-a716-446655440001', '1234567891', NOW() - INTERVAL '2 hours', 2),
('550e8400-e29b-41d4-a716-446655440001', '1234567892', NOW() - INTERVAL '1 hour', 2),
('550e8400-e29b-41d4-a716-446655440002', '1234567893', NOW() - INTERVAL '3 hours', 2),
('550e8400-e29b-41d4-a716-446655440002', '1234567894', NOW() - INTERVAL '1 day', 1),
('550e8400-e29b-41d4-a716-446655440003', '1234567895', NOW() - INTERVAL '30 minutes', 1),
('550e8400-e29b-41d4-a716-446655440003', '1234567896', NOW() - INTERVAL '45 minutes', 2)
ON CONFLICT (tweet_id) DO NOTHING;

-- Display the test data
SELECT 'Test Users Created:' as info;
SELECT id, twitter_username, patron_status, selected_role, lock_duration_months, locked_amount 
FROM users 
WHERE id IN ('550e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440005');

SELECT 'Test Tweets Created:' as info;
SELECT t.tweet_id, u.twitter_username, t.mining_phase, t.created_at 
FROM tweets t 
JOIN users u ON t.user_id = u.id 
WHERE u.id IN ('550e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440003')
ORDER BY t.created_at DESC;
Perfect! I've created a complete testing setup for you. Here's what you need to do:

Step-by-Step Setup:
1. Set up test data in your database:
# Connect to your database and run the setup script
# You'll need to use your actual database connection details
psql YOUR_DATABASE_CONNECTION_STRING -f setup_test_data.sql
2. Import the Postman Collection:
Open Postman
Click "Import"
Select the file: Snake_AI_Patron_Framework.postman_collection.json
The collection will be imported with all endpoints and test cases
3. Start Testing:
Quick Test Sequence:

Test User Profile: GET /user/550e8400-e29b-41d4-a716-446655440001
Test Mining Status: GET /user/550e8400-e29b-41d4-a716-446655440001/mining_status
Update Patron Status: POST /user/550e8400-e29b-41d4-a716-446655440001/patron_status
Verify Changes: GET /user/550e8400-e29b-41d4-a716-446655440001 (check the patron_status changed)
Files Created:
✅ POSTMAN_API_TESTING_GUIDE.md - Complete testing documentation
✅ Snake_AI_Patron_Framework.postman_collection.json - Ready-to-import Postman collection
✅ setup_test_data.sql - SQL script to create test users and tweets

Test Users Available:
User ID (last 4 digits)	Username	Patron Status	Role	Has Wallet
...0001	TestUser1	none	none	✅
...0002	TestUser2	applied	patron	❌
...0003	TestUser3	approved	staker	✅
...0004	TestUser4	rejected	none	❌
...0005	TestUser5	none	patron	❌
Would you like me to help you with anything specific about testing these endpoints, or do you need help setting up the test data?