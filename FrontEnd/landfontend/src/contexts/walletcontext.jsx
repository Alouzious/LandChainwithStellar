// src/contexts/WalletContext.jsx
import React, { createContext, useContext, useState, useEffect } from 'react';

const WalletContext = createContext();

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
};

export const WalletProvider = ({ children }) => {
  const [isWalletConnected, setIsWalletConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // Check if wallet is already connected on app load
    const savedWalletAddress = localStorage.getItem('wallet_address');
    const authToken = localStorage.getItem('auth_token');
    
    if (savedWalletAddress && authToken) {
      // Verify the connection is still valid
      checkExistingConnection(savedWalletAddress);
    }
  }, []);

  const checkExistingConnection = async (savedAddress) => {
    try {
      if (window.freighter) {
        const isAllowed = await window.freighter.isAllowed();
        if (isAllowed) {
          const currentKey = await window.freighter.getPublicKey();
          // Verify the saved address matches the current wallet
          if (currentKey === savedAddress) {
            setIsWalletConnected(true);
            setWalletAddress(savedAddress);
            return;
          }
        }
      }
      
      // If verification fails, clear stored data
      localStorage.removeItem('wallet_address');
      localStorage.removeItem('auth_token');
    } catch (error) {
      console.warn('Failed to verify existing wallet connection:', error);
      localStorage.removeItem('wallet_address');
      localStorage.removeItem('auth_token');
    }
  };

  const connectWallet = async () => {
    setIsLoading(true);
    try {
      // Check if Freighter is installed
      if (!window.freighter) {
        throw new Error('Please install Freighter wallet extension from https://www.freighter.app/');
      }

      // Request access to wallet
      const isAllowed = await window.freighter.isAllowed();
      if (!isAllowed) {
        await window.freighter.requestAccess();
      }

      // Get the user's public key
      const publicKey = await window.freighter.getPublicKey();
      
      // Store wallet info
      setIsWalletConnected(true);
      setWalletAddress(publicKey);
      localStorage.setItem('wallet_address', publicKey);
      localStorage.setItem('auth_token', 'wallet_connected');

      return { success: true, address: publicKey };
    } catch (error) {
      console.error('Wallet connection failed:', error);
      
      let errorMessage = 'Failed to connect to wallet. Please try again.';
      
      if (error.message && error.message.includes('User declined access')) {
        errorMessage = 'Please approve the connection request in your Freighter wallet.';
      } else if (error.message && error.message.includes('Freighter')) {
        errorMessage = error.message;
      }
      
      return { success: false, error: errorMessage };
    } finally {
      setIsLoading(false);
    }
  };

  const disconnectWallet = () => {
    setIsWalletConnected(false);
    setWalletAddress('');
    localStorage.removeItem('wallet_address');
    localStorage.removeItem('auth_token');
  };

  const signTransaction = async (transaction) => {
    if (!isWalletConnected) {
      throw new Error('Wallet not connected');
    }
    
    try {
      const signedTransaction = await window.freighter.signTransaction(transaction);
      return signedTransaction;
    } catch (error) {
      console.error('Transaction signing failed:', error);
      throw error;
    }
  };

  const value = {
    isWalletConnected,
    walletAddress,
    isLoading,
    connectWallet,
    disconnectWallet,
    signTransaction,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};