// src/App.js
import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { WalletProvider } from './contexts/walletcontext';
import Navbar from './components/Navbar';
import Home from './Pages/Home';
import About from './components/About';
import Contact from './components/Contact';
import RegisterLand from './Pages/RegisterLand';
import MyProperties from './components/MyProperties';
import Profile from './components/Profile';
import ProtectedRoute from './components/protectedRoute';

function App() {
  return (
    <WalletProvider>
      <Router>
        <div className="App">
          <Navbar />
          <Routes>
            <Route path="/" element={<Navigate to="/home" replace />} />
            <Route path="/home" element={<Home />} />
            <Route path="/about" element={<About />} />
            <Route path="/contact" element={<Contact />} />
            
            {/* Protected Routes - Require Wallet Connection */}
            <Route 
              path="/register-land" 
              element={
                <ProtectedRoute>
                  <RegisterLand />
                </ProtectedRoute>
              } 
            />
            <Route 
              path="/my-properties" 
              element={
                <ProtectedRoute>
                  <MyProperties />
                </ProtectedRoute>
              } 
            />
            <Route 
              path="/profile" 
              element={
                <ProtectedRoute>
                  <Profile />
                </ProtectedRoute>
              } 
            />
          </Routes>
        </div>
      </Router>
    </WalletProvider>
  );
}

export default App;