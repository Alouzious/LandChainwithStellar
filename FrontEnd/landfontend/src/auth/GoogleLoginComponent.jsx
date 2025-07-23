import React, { useState } from 'react';
import { GoogleLogin } from '@react-oauth/google';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

function GoogleLoginComponent() {
  const [isLoading, setIsLoading] = useState(false);
  const [user, setUser] = useState(null);
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleLogin = async (credentialResponse) => {
    setIsLoading(true);
    setError('');

    try {
      const id_token = credentialResponse.credential;

      const res = await axios.post('http://localhost:8000/api/users/auth/google/', {
        id_token,
      }, {
        headers: { 'Content-Type': 'application/json' },
        withCredentials: true,
      });

      localStorage.setItem('auth_token', res.data.token);

      if (res.data.user) {
        setUser(res.data.user);
      }

      console.log("✅ Google Login Successful:", res.data);
      navigate('/home');
    } catch (error) {
      console.error("❌ Login failed:", error);
      if (error.response?.data?.message) {
        setError(error.response.data.message);
      } else {
        setError("Google login failed. Please try again.");
      }
    } finally {
      setIsLoading(false);
    }
  };

  const handleLoginError = () => {
    setError("Google Login Failed. Please try again.");
  };

  const handleLogout = () => {
    localStorage.removeItem('auth_token');
    setUser(null);
    setError('');
  };

  return (
    <div style={styles.container}>
      <h3 style={styles.title}>Continue with Google</h3>
      
      {!user ? (
        <>
          <GoogleLogin
            onSuccess={handleLogin}
            onError={handleLoginError}
            useOneTap={false}
            auto_select={false}
            width="300"
            theme="outline"
            size="large"
            text="continue_with"
          />
          {isLoading && <p style={styles.loading}>Signing you in...</p>}
          {error && <p style={styles.error}>{error}</p>}
        </>
      ) : (
        <div style={styles.userInfo}>
          <p style={styles.success}>✅ Successfully logged in!</p>
          {user.name && <p>Welcome, {user.name}!</p>}
          {user.email && <p style={styles.email}>{user.email}</p>}
          <button onClick={handleLogout} style={styles.logoutButton}>Logout</button>
        </div>
      )}
    </div>
  );
}

const styles = {
  container: { textAlign: 'center', marginTop: '1rem', padding: '1rem' },
  title: { marginBottom: '1rem', color: '#333' },
  loading: { color: '#0b7285', marginTop: '1rem', fontStyle: 'italic' },
  error: { color: 'red', backgroundColor: '#f8d7da', padding: '0.5rem', borderRadius: '4px', border: '1px solid #f5c6cb' },
  success: { color: 'green' },
  userInfo: { backgroundColor: '#d4edda', padding: '1rem', borderRadius: '6px', border: '1px solid #c3e6cb' },
  email: { color: '#666', fontSize: '0.9rem' },
  logoutButton: { padding: '0.5rem 1rem', backgroundColor: '#6c757d', color: '#fff', border: 'none', borderRadius: '4px', cursor: 'pointer' },
};

export default GoogleLoginComponent;
