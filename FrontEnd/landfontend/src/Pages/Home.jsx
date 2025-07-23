// src/components/Home.jsx
import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

const Home = () => {
  const navigate = useNavigate();
  const [isWalletConnected, setIsWalletConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState('');

  useEffect(() => {
    const token = localStorage.getItem('auth_token');
    const savedWalletAddress = localStorage.getItem('wallet_address');
    
    if (token && savedWalletAddress) {
      setIsWalletConnected(true);
      setWalletAddress(savedWalletAddress);
    }
  }, []);

  const connectWallet = async () => {
    try {
      // Check if Freighter is installed
      if (!window.freighter) {
        alert('Please install Freighter wallet extension');
        return;
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

    } catch (error) {
      console.error('Wallet connection failed:', error);
      alert('Failed to connect wallet. Please try again.');
    }
  };

  const handleNavigation = (path) => {
    navigate(path);
  };

  const formatAddress = (address) => {
    if (!address) return '';
    return `${address.slice(0, 8)}...${address.slice(-8)}`;
  };

  return (
    <div style={styles.container}>
      <div style={styles.hero}>
        <h1 style={styles.title}>🏡 Welcome to LandChain</h1>
        <p style={styles.subtitle}>Secure Land Registration on the Blockchain</p>
        
        {!isWalletConnected ? (
          <div style={styles.walletSection}>
            <h2 style={styles.sectionTitle}>Get Started</h2>
            <p style={styles.description}>
              Connect your Stellar wallet to access land registration services and manage your properties securely.
            </p>
            <button style={styles.connectButton} onClick={connectWallet}>
              🔗 Connect Your Wallet to Get Started
            </button>
            <div style={styles.features}>
              <div style={styles.feature}>
                <span style={styles.featureIcon}>🔒</span>
                <p>Secure blockchain-based land records</p>
              </div>
              <div style={styles.feature}>
                <span style={styles.featureIcon}>📋</span>
                <p>Digital document verification</p>
              </div>
              <div style={styles.feature}>
                <span style={styles.featureIcon}>⚡</span>
                <p>Fast and transparent transactions</p>
              </div>
            </div>
          </div>
        ) : (
          <div style={styles.dashboardSection}>
            <h2 style={styles.sectionTitle}>Welcome Back!</h2>
            <p style={styles.description}>
              Wallet Connected: <span style={styles.walletAddress}>{formatAddress(walletAddress)}</span>
            </p>
            
            <div style={styles.actionButtons}>
              <button 
                style={styles.actionButton}
                onClick={() => handleNavigation('/register-land')}
              >
                📝 Register Land
              </button>
              <button 
                style={styles.actionButton}
                onClick={() => handleNavigation('/my-properties')}
              >
                🏠 View My Lands
              </button>
              <button 
                style={styles.actionButton}
                onClick={() => handleNavigation('/profile')}
              >
                👤 Check Profile
              </button>
            </div>

            <div style={styles.statusCards}>
              <div style={styles.statusCard}>
                <h3 style={styles.cardTitle}>🏡 Properties</h3>
                <p style={styles.cardValue}>0</p>
                <p style={styles.cardSubtext}>Registered lands</p>
              </div>
              <div style={styles.statusCard}>
                <h3 style={styles.cardTitle}>✅ Verified</h3>
                <p style={styles.cardValue}>0</p>
                <p style={styles.cardSubtext}>Verified properties</p>
              </div>
              <div style={styles.statusCard}>
                <h3 style={styles.cardTitle}>⏳ Pending</h3>
                <p style={styles.cardValue}>0</p>
                <p style={styles.cardSubtext}>Pending verification</p>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

const styles = {
  container: {
    minHeight: '100vh',
    background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)',
    padding: '2rem 1rem',
  },
  hero: {
    maxWidth: '1200px',
    margin: '0 auto',
    textAlign: 'center',
    padding: '2rem',
  },
  title: {
    fontSize: '3rem',
    fontWeight: '800',
    color: '#2c3e50',
    marginBottom: '1rem',
    textShadow: '2px 2px 4px rgba(0, 0, 0, 0.1)',
  },
  subtitle: {
    fontSize: '1.3rem',
    color: '#7f8c8d',
    marginBottom: '3rem',
    fontWeight: '400',
  },
  walletSection: {
    backgroundColor: '#ffffff',
    borderRadius: '20px',
    padding: '3rem',
    boxShadow: '0 10px 40px rgba(0, 0, 0, 0.1)',
    marginBottom: '2rem',
  },
  dashboardSection: {
    backgroundColor: '#ffffff',
    borderRadius: '20px',
    padding: '3rem',
    boxShadow: '0 10px 40px rgba(0, 0, 0, 0.1)',
    marginBottom: '2rem',
  },
  sectionTitle: {
    fontSize: '2rem',
    fontWeight: '700',
    color: '#2c3e50',
    marginBottom: '1rem',
  },
  description: {
    fontSize: '1.1rem',
    color: '#7f8c8d',
    marginBottom: '2rem',
    lineHeight: '1.6',
  },
  walletAddress: {
    fontFamily: 'monospace',
    backgroundColor: '#f8f9fa',
    padding: '0.3rem 0.8rem',
    borderRadius: '15px',
    color: '#2c3e50',
    fontWeight: '600',
  },
  connectButton: {
    backgroundColor: '#1abc9c',
    color: '#ffffff',
    border: 'none',
    padding: '1rem 2rem',
    borderRadius: '30px',
    fontSize: '1.1rem',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'all 0.3s ease',
    boxShadow: '0 6px 20px rgba(26, 188, 156, 0.3)',
    marginBottom: '3rem',
  },
  actionButtons: {
    display: 'flex',
    gap: '1rem',
    justifyContent: 'center',
    flexWrap: 'wrap',
    marginBottom: '3rem',
  },
  actionButton: {
    backgroundColor: '#667eea',
    color: '#ffffff',
    border: 'none',
    padding: '1rem 1.5rem',
    borderRadius: '25px',
    fontSize: '1rem',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'all 0.3s ease',
    boxShadow: '0 4px 15px rgba(102, 126, 234, 0.3)',
    minWidth: '180px',
  },
  features: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))',
    gap: '2rem',
    marginTop: '3rem',
  },
  feature: {
    textAlign: 'center',
    padding: '1.5rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '15px',
    transition: 'transform 0.3s ease',
  },
  featureIcon: {
    fontSize: '2rem',
    display: 'block',
    marginBottom: '1rem',
  },
  statusCards: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
    gap: '1.5rem',
    marginTop: '2rem',
  },
  statusCard: {
    backgroundColor: '#f8f9fa',
    padding: '2rem',
    borderRadius: '15px',
    textAlign: 'center',
    boxShadow: '0 4px 15px rgba(0, 0, 0, 0.05)',
  },
  cardTitle: {
    fontSize: '1.1rem',
    fontWeight: '600',
    color: '#2c3e50',
    marginBottom: '1rem',
  },
  cardValue: {
    fontSize: '2.5rem',
    fontWeight: '800',
    color: '#667eea',
    marginBottom: '0.5rem',
  },
  cardSubtext: {
    fontSize: '0.9rem',
    color: '#7f8c8d',
  },
};

export default Home;




















































// import React, { useEffect } from 'react';
// import { useNavigate } from 'react-router-dom';

// const Home = () => {
//   const navigate = useNavigate();

//   useEffect(() => {
//     const token = localStorage.getItem('auth_token');
//     if (!token) {
//       navigate('/');
//     }
//   }, [navigate]);

//   return (
//     <div>
//       <h2>🏡 Welcome to your Land App Home</h2>
//       <p>You are successfully logged in.</p>
//     </div>
//   );
// };

// export default Home;
