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
