// src/components/Contact.jsx
import React from 'react';

const styles = {
  container: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    minHeight: '80vh',
    backgroundColor: '#f8f9fa',
  },
  content: {
    background: '#fff',
    padding: '2rem 3rem',
    borderRadius: '12px',
    boxShadow: '0 2px 12px rgba(0,0,0,0.08)',
    maxWidth: '400px',
    width: '100%',
    textAlign: 'center',
  },
  title: {
    marginBottom: '1rem',
    color: '#2d3748',
    fontSize: '2rem',
    fontWeight: 'bold',
  },
  description: {
    marginBottom: '2rem',
    color: '#4a5568',
    fontSize: '1.1rem',
  },
  contactInfo: {
    display: 'flex',
    flexDirection: 'column',
    gap: '1.5rem',
  },
  contactItem: {
    background: '#f1f5f9',
    padding: '1rem',
    borderRadius: '8px',
  },
};

const Contact = () => {
  return (
    <div style={styles.container}>
      <div style={styles.content}>
        <h1 style={styles.title}>Contact Us</h1>
        <p style={styles.description}>
          Have questions about LandChain? We're here to help!
        </p>
        <div style={styles.contactInfo}>
          <div style={styles.contactItem}>
            <h3>📧 Email</h3>
            <p>support@landchain.com</p>
          </div>
          <div style={styles.contactItem}>
            <h3>🌐 Website</h3>
            <p>www.landchain.com</p>
          </div>
          <div style={styles.contactItem}>
            <h3>📱 Support</h3>
            <p>24/7 Customer Support</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Contact;
