-- Create tweet templates table
CREATE TABLE tweet_templates (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    category VARCHAR(50) DEFAULT 'general',
    is_active BOOLEAN DEFAULT true,
    usage_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster queries
CREATE INDEX idx_tweet_templates_active ON tweet_templates(is_active);
CREATE INDEX idx_tweet_templates_category ON tweet_templates(category);

-- Insert initial tweet templates
INSERT INTO tweet_templates (content, category) VALUES
-- Gaming & AI themed tweets
('Just discovered @playSnakeAI! 🐍 This AI-powered snake game is revolutionizing GameFi! #MineTheSnake #GameFi #AI', 'gaming'),
('The future of gaming is here with @playSnakeAI! 🚀 AI meets blockchain in the most addictive way possible! #MineTheSnake #Web3Gaming', 'gaming'),
('Can''t stop playing @playSnakeAI! 🎮 The AI mechanics are incredible and the rewards are real! #MineTheSnake #PlayToEarn', 'gaming'),
('@playSnakeAI is changing the game! 🐍 Smart contracts, AI gameplay, and actual rewards? Count me in! #MineTheSnake #DeFi', 'gaming'),
('Who said snakes can''t be smart? @playSnakeAI proves otherwise! 🧠🐍 #MineTheSnake #ArtificialIntelligence', 'gaming'),

-- Community & Social themed tweets
('Joined the @playSnakeAI community and loving every moment! 🌟 The most welcoming GameFi community out there! #MineTheSnake #Community', 'community'),
('The @playSnakeAI community is growing fast! 📈 Join us before it''s too late! #MineTheSnake #EarlyAdopter', 'community'),
('Shoutout to the amazing @playSnakeAI team! 👏 Building the future of gaming one block at a time! #MineTheSnake #TeamWork', 'community'),
('@playSnakeAI bringing people together through gaming! 🤝 This is what Web3 should be about! #MineTheSnake #Web3Community', 'community'),
('The @playSnakeAI ecosystem is thriving! 🌱 Proud to be part of this revolutionary project! #MineTheSnake #Innovation', 'community'),

-- Technology & Innovation themed tweets
('@playSnakeAI is pushing the boundaries of what''s possible in GameFi! 🔬 AI + Blockchain = Future! #MineTheSnake #Innovation', 'technology'),
('The technology behind @playSnakeAI is mind-blowing! 🤯 Smart contracts that actually make sense! #MineTheSnake #SmartContracts', 'technology'),
('@playSnakeAI proves that gaming and DeFi can work together seamlessly! ⚡ #MineTheSnake #DeFiGaming', 'technology'),
('Blockchain gaming done right! @playSnakeAI shows how it should be! 💎 #MineTheSnake #BlockchainGaming', 'technology'),
('@playSnakeAI is the perfect example of utility meeting entertainment! 🎯 #MineTheSnake #UtilityToken', 'technology'),

-- Investment & Financial themed tweets
('@playSnakeAI isn''t just a game, it''s an investment in the future! 💰 #MineTheSnake #Investment', 'financial'),
('Smart money is flowing into @playSnakeAI! 📊 Don''t miss this opportunity! #MineTheSnake #SmartMoney', 'financial'),
('@playSnakeAI tokenomics are designed for long-term success! 📈 #MineTheSnake #Tokenomics', 'financial'),
('The ROI potential of @playSnakeAI is incredible! 🚀 Gaming meets investing! #MineTheSnake #ROI', 'financial'),
('@playSnakeAI rewards are real and sustainable! 💎 This is how GameFi should work! #MineTheSnake #SustainableRewards', 'financial'),

-- Fun & Casual themed tweets
('Having a blast with @playSnakeAI! 🎉 Who knew earning could be this fun? #MineTheSnake #FunToEarn', 'casual'),
('@playSnakeAI got me addicted! 😅 Send help... or more SNAKE tokens! #MineTheSnake #Addicted', 'casual'),
('Just beat my high score on @playSnakeAI! 🏆 This game never gets old! #MineTheSnake #HighScore', 'casual'),
('@playSnakeAI is my new favorite way to spend time! ⏰ Productive procrastination at its finest! #MineTheSnake #Productivity', 'casual'),
('Coffee ☕ + @playSnakeAI = Perfect morning! Who else is mining today? #MineTheSnake #MorningVibes', 'casual'),

-- Educational & Informative themed tweets
('Did you know @playSnakeAI uses advanced AI algorithms for gameplay? 🤖 Learn while you earn! #MineTheSnake #Education', 'educational'),
('@playSnakeAI is teaching me about DeFi in the most fun way possible! 📚 #MineTheSnake #DeFiEducation', 'educational'),
('Understanding blockchain has never been easier thanks to @playSnakeAI! 🧠 #MineTheSnake #BlockchainEducation', 'educational'),
('@playSnakeAI shows how gaming can be educational AND profitable! 🎓 #MineTheSnake #EdTech', 'educational'),
('Learning about tokenomics through @playSnakeAI gameplay! 📖 This is the future of education! #MineTheSnake #Learning', 'educational'),

-- Motivational & Inspirational themed tweets
('@playSnakeAI proves that innovation never stops! 🌟 Keep building, keep growing! #MineTheSnake #Innovation', 'motivational'),
('Every game of @playSnakeAI is a step towards financial freedom! 💪 #MineTheSnake #FinancialFreedom', 'motivational'),
('@playSnakeAI reminds us that the future is what we make it! 🚀 #MineTheSnake #FutureIsNow', 'motivational'),
('Success in @playSnakeAI comes to those who persist! 🎯 Keep mining, keep winning! #MineTheSnake #Persistence', 'motivational'),
('@playSnakeAI is proof that great things come to those who innovate! ⚡ #MineTheSnake #GreatThings', 'motivational'),

-- Trending & Viral themed tweets
('@playSnakeAI is trending for all the right reasons! 📈 Join the movement! #MineTheSnake #Trending', 'viral'),
('Everyone''s talking about @playSnakeAI! 🗣️ Don''t get left behind! #MineTheSnake #EveryonesTalking', 'viral'),
('@playSnakeAI is going viral! 🌪️ Be part of the phenomenon! #MineTheSnake #Viral', 'viral'),
('The @playSnakeAI hype is real! 🔥 And it''s just getting started! #MineTheSnake #Hype', 'viral'),
('@playSnakeAI is breaking the internet! 💥 In the best way possible! #MineTheSnake #BreakingTheInternet', 'viral'),

-- Achievement & Success themed tweets
('Just reached a new milestone in @playSnakeAI! 🏅 The journey continues! #MineTheSnake #Milestone', 'achievement'),
('@playSnakeAI rewards are paying off! 💰 Consistency is key! #MineTheSnake #PayingOff', 'achievement'),
('Leveled up in @playSnakeAI today! 📈 Progress feels amazing! #MineTheSnake #LevelUp', 'achievement'),
('@playSnakeAI achievements unlocked! 🔓 What''s next on the list? #MineTheSnake #Unlocked', 'achievement'),
('Celebrating another successful session with @playSnakeAI! 🎊 #MineTheSnake #Success', 'achievement'),

-- Future & Vision themed tweets
('@playSnakeAI is building the future of entertainment! 🔮 Excited to be part of it! #MineTheSnake #FutureOfEntertainment', 'future'),
('The vision behind @playSnakeAI is inspiring! 👁️ Gaming will never be the same! #MineTheSnake #Vision', 'future'),
('@playSnakeAI is paving the way for next-gen gaming! 🛤️ The future is bright! #MineTheSnake #NextGen', 'future'),
('Imagine a world where @playSnakeAI is everywhere! 🌍 That future is closer than you think! #MineTheSnake #Everywhere', 'future'),
('@playSnakeAI today, mainstream adoption tomorrow! 📅 The timeline is accelerating! #MineTheSnake #MainstreamAdoption', 'future'),

-- General enthusiasm tweets
('@playSnakeAI never fails to amaze me! ✨ Pure innovation in action! #MineTheSnake #Amazing', 'general'),
('Grateful to be part of the @playSnakeAI journey! 🙏 This is just the beginning! #MineTheSnake #Grateful', 'general'),
('@playSnakeAI is more than a game, it''s a revolution! 🔄 Join the revolution! #MineTheSnake #Revolution', 'general'),
('The energy around @playSnakeAI is infectious! ⚡ Catch the fever! #MineTheSnake #Infectious', 'general'),
('@playSnakeAI continues to exceed expectations! 📊 Bullish on the future! #MineTheSnake #Bullish', 'general');

-- Update the updated_at timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tweet_templates_updated_at 
    BEFORE UPDATE ON tweet_templates 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();