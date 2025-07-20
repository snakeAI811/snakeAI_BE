
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import LandingPage from "./pages/landing";
import GetStartedPage from "./pages/get_started";
import Home from "./pages/home";
import TweetMiningPage from "./pages/tweet_mining";
import GenerateMeme from "./pages/meme_generation";
import PatronFrameworkPage from "./pages/patron";
import ClaimPage from "./pages/claim";
import { WalletContextProvider } from "./contexts/WalletContext";
import { AuthContextProvider } from "./contexts/AuthContext";

function  App() {
  return (
    <AuthContextProvider>
      <WalletContextProvider>
        <Router>
        <Routes>
          <Route path="/" element={<LandingPage />} />

          <Route path="/get-started" element={<GetStartedPage />} />

          <Route path="/home" element={<Home />} />

          <Route path="/tweet-mining" element={<TweetMiningPage />} />
          <Route path="/tweet-mining/page1" element={<TweetMiningPage page_number={1} />} />
          <Route path="/tweet-mining/page2" element={<TweetMiningPage page_number={2} />} />
          <Route path="/tweet-mining/page3" element={<TweetMiningPage page_number={3} />} />

          <Route path="/meme-generation" element={<GenerateMeme />} />
          <Route path="/meme-generation/page1" element={<GenerateMeme page_number={1} />} />
          <Route path="/meme-generation/page2" element={<GenerateMeme page_number={2} />} />
          <Route path="/meme-generation/page3" element={<GenerateMeme page_number={3} />} />

          <Route path="/patron-framework" element={<PatronFrameworkPage />} />
          
          <Route path="/claim/:id" element={<ClaimPage />} />
          <Route path="/claim" element={<ClaimPage />} />
          <Route path="/profile" element={<>profile</>} />
          <Route path="/staking" element={<>staking</>} />
          <Route path="/swap" element={<>swap</>} />
          <Route path="/dao" element={<>dao</>} />

        </Routes>
        </Router>
      </WalletContextProvider>
    </AuthContextProvider>
  );
}

export default App;
