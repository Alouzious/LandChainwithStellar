import React, { useState, useEffect } from 'react';
import GoogleLoginComponent from '../auth/GoogleLoginComponent';
import { useNavigate } from 'react-router-dom';

function RegisterLand() {
  const navigate = useNavigate();
  
  useEffect(() => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      navigate('/home');
    }
  }, [navigate]);

  const [formData, setFormData] = useState({ firstName: '', lastName: '', email: '' });
  const [errors, setErrors] = useState({});
  const [apiError, setApiError] = useState('');
  const [loading, setLoading] = useState(false);

  const sanitize = (value) => value.replace(/<\/?[^>]+(>|$)/g, "").trim();

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: sanitize(value) }));
  };

  const validate = () => {
    const newErrors = {};
    if (!formData.firstName) newErrors.firstName = 'First name is required';
    if (!formData.lastName) newErrors.lastName = 'Last name is required';
    if (!formData.email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      newErrors.email = 'Valid email is required';
    }
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!validate()) return;

    setLoading(true);
    try {
      const res = await fetch('http://localhost:8000/api/users/register/', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          first_name: formData.firstName,
          last_name: formData.lastName,
          email: formData.email,
          username: formData.email,
        }),
      });

      const result = await res.json();

      if (res.ok) {
        alert('Registration successful!');
        setFormData({ firstName: '', lastName: '', email: '' });
        navigate('/home');
      } else {
        setApiError(JSON.stringify(result));
      }
    } catch (err) {
      setApiError('Network error: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={styles.container}>
      <h2 style={styles.title}>Register to use the Land App</h2>

      <form onSubmit={handleSubmit} style={styles.form}>
        <input type="text" name="firstName" placeholder="First Name" value={formData.firstName} onChange={handleChange} style={errors.firstName ? {...styles.input, borderColor: 'red'} : styles.input} />
        {errors.firstName && <div style={styles.error}>{errors.firstName}</div>}

        <input type="text" name="lastName" placeholder="Last Name" value={formData.lastName} onChange={handleChange} style={errors.lastName ? {...styles.input, borderColor: 'red'} : styles.input} />
        {errors.lastName && <div style={styles.error}>{errors.lastName}</div>}

        <input type="email" name="email" placeholder="Email Address" value={formData.email} onChange={handleChange} style={errors.email ? {...styles.input, borderColor: 'red'} : styles.input} />
        {errors.email && <div style={styles.error}>{errors.email}</div>}

        <button type="submit" style={styles.submitButton} disabled={loading}>{loading ? 'Registering...' : 'Register'}</button>
      </form>

      {apiError && <div style={styles.apiError}>Error: {apiError}</div>}

      <div style={styles.divider}>OR</div>
      <GoogleLoginComponent />
    </div>
  );
}

const styles = {
  container: { maxWidth: '400px', margin: '2rem auto', padding: '2rem', boxShadow: '0 4px 10px rgba(0,0,0,0.1)', borderRadius: '8px', backgroundColor: '#fff' },
  title: { textAlign: 'center', marginBottom: '1.5rem' },
  form: { display: 'flex', flexDirection: 'column', gap: '0.5rem' },
  input: { padding: '0.75rem', fontSize: '1rem', borderRadius: '4px', border: '1px solid #ccc' },
  error: { color: 'red', fontSize: '0.85rem', marginBottom: '0.5rem' },
  apiError: { color: 'red', fontSize: '0.9rem', marginTop: '1rem', textAlign: 'center' },
  submitButton: { marginTop: '1rem', padding: '0.75rem', fontSize: '1rem', borderRadius: '4px', backgroundColor: '#2c3e50', color: '#fff', border: 'none', cursor: 'pointer' },
  divider: { margin: '1.5rem 0', textAlign: 'center', color: '#999', fontWeight: 'bold' }
};

export default RegisterLand;
