// src/components/ProtectedRoute.jsx
import React from 'react';
import { Navigate } from 'react-router-dom';
import { useWallet } from '../contexts/walletcontext';

const ProtectedRoute = ({ children }) => {
  const { isWalletConnected } = useWallet();

  if (!isWalletConnected) {
    return <Navigate to="/home" replace />;
  }

  return children;
};

export default ProtectedRoute;