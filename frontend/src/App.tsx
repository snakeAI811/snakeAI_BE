
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import LandingPage from "./pages/landing";
import GetStartedPage from "./pages/get_started";
import Home from "./pages/home";
import TweetMiningPage from "./pages/tweet_mining";
import GenerateMeme from "./pages/meme_generation";
import Leaderboard from "./pages/leaderboard";
import OTCTrading from "./pages/otc";
import Profile from "./pages/profile";
import StakingPage from "./pages/staking";
import PatronApprovals from "./pages/admin/PatronApprovals";
import TceManagement from "./pages/admin/TceManagement";
import StakingHistoryManagement from "./pages/admin/StakingHistoryManagement";
import AdminDashboard from "./pages/admin/AdminDashboard";
import { WalletContextProvider } from "./contexts/WalletContext";
import { AuthContextProvider } from "./contexts/AuthContext";
import { AppContextProvider } from "./contexts/AppContext";
import { ToastProvider } from "./contexts/ToastContext";
import ToastContainer from "./components/ToastContainer";
import ProtectedRoute from "./components/ProtectedRoute";
import AdminRoute from "./components/AdminRoute";
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
                <Route path="/leaderboard" element={<ProtectedRoute><Leaderboard /></ProtectedRoute>} />
                {/* Admin Routes */}
                <Route path="/admin" element={<AdminRoute><AdminDashboard /></AdminRoute>} />
                <Route path="/admin/patron-approvals" element={<AdminRoute><PatronApprovals /></AdminRoute>} />
                <Route path="/admin/tce-management" element={<AdminRoute><TceManagement /></AdminRoute>} />
                <Route path="/admin/staking-history" element={<AdminRoute><StakingHistoryManagement /></AdminRoute>} />
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
