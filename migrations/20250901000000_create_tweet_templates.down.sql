-- Drop the trigger and function
DROP TRIGGER IF EXISTS update_tweet_templates_updated_at ON tweet_templates;
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop the table
DROP TABLE IF EXISTS tweet_templates;