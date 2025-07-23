// src/components/Profile.jsx
import React from 'react';
import { useWallet } from '../contexts/walletcontext';

const styles = {
  container: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    minHeight: '100vh',
    background: '#f5f6fa',
  },
  content: {
    background: '#fff',
    padding: '32px',
    borderRadius: '12px',
    boxShadow: '0 2px 12px rgba(0,0,0,0.08)',
    minWidth: '350px',
  },
  title: {
    marginBottom: '24px',
    fontSize: '2rem',
    color: '#222',
    textAlign: 'center',
  },
  profileCard: {
    border: '1px solid #e1e1e1',
    borderRadius: '8px',
    padding: '20px',
    background: '#fafbfc',
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
  },
  profileInfo: {
    marginBottom: '12px',
  },
  fullAddress: {
    fontFamily: 'monospace',
    fontSize: '0.95em',
    color: '#555',
  },
  status: {
    color: 'green',
    fontWeight: 'bold',
  },
  actions: {
    display: 'flex',
    justifyContent: 'flex-end',
  },
  disconnectButton: {
    background: '#e74c3c',
    color: '#fff',
    border: 'none',
    borderRadius: '6px',
    padding: '8px 18px',
    cursor: 'pointer',
    fontWeight: 'bold',
    fontSize: '1em',
    transition: 'background 0.2s',
  },
};

const Profile = () => {
  const { walletAddress, disconnectWallet } = useWallet();

  const formatAddress = (address) => {
    if (!address) return '';
    return `${address.slice(0, 8)}...${address.slice(-8)}`;
  };

  return (
    <div style={styles.container}>
      <div style={styles.content}>
        <h1 style={styles.title}>👤 Profile</h1>
        <div style={styles.profileCard}>
          <div style={styles.profileInfo}>
            <h3>Wallet Information</h3>
            <p><strong>Address:</strong> {formatAddress(walletAddress)}</p>
            <p><strong>Full Address:</strong> <span style={styles.fullAddress}>{walletAddress}</span></p>
            <p><strong>Status:</strong> <span style={styles.status}>Connected</span></p>
          </div>
          <div style={styles.actions}>
            <button style={styles.disconnectButton} onClick={disconnectWallet}>
              Disconnect Wallet
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Profile;