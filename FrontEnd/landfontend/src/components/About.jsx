// src/components/About.jsx
import React from 'react';

const About = () => {
  return (
    <div style={styles.container}>
      <div style={styles.content}>
        <h1 style={styles.title}>About LandChain</h1>
        <p style={styles.description}>
          LandChain is a revolutionary blockchain-based land registration system built on the Stellar network. 
          We provide secure, transparent, and efficient land registration services.
        </p>
        <div style={styles.features}>
          <div style={styles.feature}>
            <h3>🔒 Secure</h3>
            <p>Blockchain-based security ensures your land records are immutable and protected.</p>
          </div>
          <div style={styles.feature}>
            <h3>🌍 Transparent</h3>
            <p>All transactions are recorded on the blockchain for complete transparency.</p>
          </div>
          <div style={styles.feature}>
            <h3>⚡ Fast</h3>
            <p>Quick processing times with instant verification capabilities.</p>
          </div>
        </div>
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
  content: {
    maxWidth: '800px',
    margin: '0 auto',
    backgroundColor: '#ffffff',
    borderRadius: '20px',
    padding: '3rem',
    boxShadow: '0 10px 40px rgba(0, 0, 0, 0.1)',
  },
  title: {
    fontSize: '2.5rem',
    fontWeight: '800',
    color: '#2c3e50',
    marginBottom: '2rem',
    textAlign: 'center',
  },
  description: {
    fontSize: '1.1rem',
    color: '#7f8c8d',
    lineHeight: '1.6',
    marginBottom: '3rem',
    textAlign: 'center',
  },
  features: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))',
    gap: '2rem',
  },
  feature: {
    textAlign: 'center',
    padding: '2rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '15px',
  },
  contactInfo: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))',
    gap: '2rem',
  },
  contactItem: {
    textAlign: 'center',
    padding: '2rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '15px',
  },
  form: {
    backgroundColor: '#f8f9fa',
    padding: '2rem',
    borderRadius: '15px',
    textAlign: 'center',
  },
  button: {
    backgroundColor: '#667eea',
    color: '#ffffff',
    border: 'none',
    padding: '1rem 2rem',
    borderRadius: '25px',
    fontSize: '1rem',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'all 0.3s ease',
    boxShadow: '0 4px 15px rgba(102, 126, 234, 0.3)',
  },
  propertiesGrid: {
    display: 'grid',
    gap: '2rem',
  },
  emptyState: {
    textAlign: 'center',
    padding: '3rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '15px',
  },
  profileCard: {
    backgroundColor: '#f8f9fa',
    padding: '2rem',
    borderRadius: '15px',
  },
  profileInfo: {
    marginBottom: '2rem',
  },
  fullAddress: {
    fontFamily: 'monospace',
    fontSize: '0.9rem',
    backgroundColor: '#e9ecef',
    padding: '0.5rem',
    borderRadius: '5px',
    wordBreak: 'break-all',
  },
  status: {
    color: '#28a745',
    fontWeight: '600',
  },
  actions: {
    textAlign: 'center',
  },
  disconnectButton: {
    backgroundColor: '#dc3545',
    color: '#ffffff',
    border: 'none',
    padding: '1rem 2rem',
    borderRadius: '25px',
    fontSize: '1rem',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'all 0.3s ease',
  },
};

export default About;


// src/components/RegisterLand.jsx
// import React from 'react';
// import { useWallet } from '../contexts/walletcontext';

// const RegisterLand = () => {
//   const { walletAddress } = useWallet();

//   return (
//     <div style={styles.container}>
//       <div style={styles.content}>
//         <h1 style={styles.title}>📝 Register Land</h1>
//         <p style={styles.description}>
//           Connected Wallet: {walletAddress}
//         </p>
//         <div style={styles.form}>
//           <h2>Land Registration Form</h2>
//           <p>Upload your land documents and complete the registration process.</p>
//           <button style={styles.button}>Start Registration Process</button>
//         </div>
//       </div>
//     </div>
//   );
// };

// export default RegisterLand;

