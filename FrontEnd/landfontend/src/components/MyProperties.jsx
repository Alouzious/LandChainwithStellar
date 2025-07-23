// src/components/MyProperties.jsx
import React from 'react';
import { useWallet } from '../contexts/walletcontext';

const styles = {
  container: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    minHeight: '80vh',
    background: '#f9f9f9',
  },
  content: {
    background: '#fff',
    padding: '2rem 3rem',
    borderRadius: '12px',
    boxShadow: '0 2px 16px rgba(0,0,0,0.08)',
    minWidth: '350px',
    maxWidth: '500px',
  },
  title: {
    marginBottom: '1rem',
    fontSize: '2rem',
    color: '#333',
  },
  description: {
    marginBottom: '2rem',
    color: '#666',
    fontSize: '1rem',
  },
  propertiesGrid: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    minHeight: '150px',
    justifyContent: 'center',
  },
  emptyState: {
    textAlign: 'center',
    color: '#888',
  },
  button: {
    marginTop: '1rem',
    padding: '0.75rem 1.5rem',
    background: '#0078d4',
    color: '#fff',
    border: 'none',
    borderRadius: '6px',
    cursor: 'pointer',
    fontSize: '1rem',
  },
};

const MyProperties = () => {
  const { walletAddress } = useWallet();

  return (
    <div style={styles.container}>
      <div style={styles.content}>
        <h1 style={styles.title}>🏠 My Properties</h1>
        <p style={styles.description}>
          Wallet: {walletAddress}
        </p>
        <div style={styles.propertiesGrid}>
          <div style={styles.emptyState}>
            <h3>No Properties Yet</h3>
            <p>You haven't registered any land properties yet.</p>
            <button style={styles.button}>Register Your First Land</button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MyProperties;

