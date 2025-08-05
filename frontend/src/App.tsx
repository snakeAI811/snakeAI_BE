
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import LandingPage from "./pages/landing";
import GetStartedPage from "./pages/get_started";
import Home from "./pages/home";
import TweetMiningPage from "./pages/tweet_mining";
import GenerateMeme from "./pages/meme_generation";
import PatronFrameworkPage from "./pages/patron";
import ApplicationPage from "./pages/dao";
// import ApplicationPage from "./pages/patron/application";
import OTCTrading from "./pages/otc";
import ClaimPage from "./pages/claim";
import Profile from "./pages/profile";
import StakingPage from "./pages/staking";
import { WalletContextProvider } from "./contexts/WalletContext";
import { AuthContextProvider } from "./contexts/AuthContext";
import { AppContextProvider } from "./contexts/AppContext";
import { ToastProvider } from "./contexts/ToastContext";
import ToastContainer from "./components/ToastContainer";
import ProtectedRoute from "./components/ProtectedRoute";
import { BrowserRouter } from 'react-router-dom'

function App() {
  return (
    <Router>
      <AuthContextProvider>
        <WalletContextProvider>
          <AppContextProvider>
            <ToastProvider>
              <Routes>
                <Route path="/" element={<LandingPage />} />
                <Route path="/get-started" element={<GetStartedPage />} />
                <Route path="/home" element={<ProtectedRoute><Home /></ProtectedRoute>} />
                <Route path="/tweet-mining" element={<ProtectedRoute><TweetMiningPage /></ProtectedRoute>} />
                <Route path="/meme-generation" element={<ProtectedRoute><GenerateMeme /></ProtectedRoute>} />
                <Route path="/claim/:id" element={<ProtectedRoute><TweetMiningPage /></ProtectedRoute>} />
                <Route path="/claim" element={<ProtectedRoute><TweetMiningPage /></ProtectedRoute>} />
                <Route path="/profile" element={<ProtectedRoute><Profile /></ProtectedRoute>} />
                <Route path="/staking" element={<ProtectedRoute><StakingPage /></ProtectedRoute>} />
                <Route path="/swap" element={<ProtectedRoute><OTCTrading /></ProtectedRoute>} />
                <Route path="/dao" element={<ProtectedRoute><ApplicationPage /></ProtectedRoute>} />
              </Routes>
              <ToastContainer />
            </ToastProvider>
          </AppContextProvider>
        </WalletContextProvider>
      </AuthContextProvider>
    </Router>
  );
}

export default App;
