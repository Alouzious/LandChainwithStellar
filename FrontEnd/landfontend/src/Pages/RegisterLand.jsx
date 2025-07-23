// src/components/RegisterLand.jsx
import React from 'react';
import { useWallet } from '../contexts/walletcontext';

const styles = {
  container: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    minHeight: '100vh',
    background: '#f5f5f5',
  },
  content: {
    background: '#fff',
    padding: '32px',
    borderRadius: '12px',
    boxShadow: '0 2px 8px rgba(0,0,0,0.1)',
    minWidth: '350px',
    maxWidth: '400px',
    width: '100%',
  },
  title: {
    marginBottom: '16px',
    fontSize: '2rem',
    textAlign: 'center',
  },
  description: {
    marginBottom: '24px',
    color: '#555',
    textAlign: 'center',
  },
  form: {
    marginTop: '16px',
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
  },
  button: {
    marginTop: '12px',
    padding: '10px 20px',
    background: '#0078d4',
    color: '#fff',
    border: 'none',
    borderRadius: '6px',
    cursor: 'pointer',
    fontSize: '1rem',
  },
};

const RegisterLand = () => {
  const { walletAddress } = useWallet();

  return (
    <div style={styles.container}>
      <div style={styles.content}>
        <h1 style={styles.title}>📝 Register Land</h1>
        <p style={styles.description}>
          Connected Wallet: {walletAddress}
        </p>
        <div style={styles.form}>
          <h2>Land Registration Form</h2>
          <p>Upload your land documents and complete the registration process.</p>
          <button style={styles.button}>Start Registration Process</button>
        </div>
      </div>
    </div>
  );
};

export default RegisterLand;
