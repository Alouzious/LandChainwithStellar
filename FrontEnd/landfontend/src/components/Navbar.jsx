import React, { useState, useEffect } from "react";
import { NavLink } from "react-router-dom";

function Navbar() {
  const [isWalletConnected, setIsWalletConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState("");
  const [showDropdown, setShowDropdown] = useState(false);

  // Helper function to check if Freighter is installed
  const isFreighterInstalled = () => {
    return typeof window.freighter !== 'undefined';
  };

  // Helper function to wait for Freighter to load
  const waitForFreighter = (timeout = 5000) => {
    return new Promise((resolve) => {
      if (isFreighterInstalled()) {
        resolve(true);
        return;
      }

      const checkInterval = 100;
      let elapsed = 0;

      const interval = setInterval(() => {
        elapsed += checkInterval;
        
        if (isFreighterInstalled()) {
          clearInterval(interval);
          resolve(true);
        } else if (elapsed >= timeout) {
          clearInterval(interval);
          resolve(false);
        }
      }, checkInterval);
    });
  };

  useEffect(() => {
    async function checkWallet() {
      try {
        // Debug info
        console.log('=== Freighter Debug Info ===');
        console.log('window.freighter:', window.freighter);
        console.log('window.freighterApi:', window.freighterApi);
        
        // Wait for Freighter to be available
        const freighterAvailable = await waitForFreighter(3000);
        if (!freighterAvailable) {
          console.warn("Freighter wallet extension not found");
          return;
        }

        // Check if already connected
        const isAllowed = await window.freighter.isAllowed();
        if (isAllowed) {
          const publicKey = await window.freighter.getPublicKey();
          setWalletAddress(publicKey);
          setIsWalletConnected(true);
          localStorage.setItem("wallet_address", publicKey);
        }
      } catch (error) {
        console.warn("Error checking wallet:", error.message);
      }
    }
    checkWallet();
  }, []);

  const connectWallet = async () => {
    try {
      // Check if Freighter is installed
      const freighterAvailable = await waitForFreighter(1000);
      if (!freighterAvailable) {
        alert("Freighter wallet extension is not installed.\n\nPlease install it from: https://www.freighter.app/");
        window.open("https://www.freighter.app/", '_blank');
        return;
      }

      // Request access to wallet
      const isAllowed = await window.freighter.requestAccess();
      if (!isAllowed) {
        throw new Error("User denied wallet access");
      }

      // Get the user's public key
      const publicKey = await window.freighter.getPublicKey();
      
      setWalletAddress(publicKey);
      setIsWalletConnected(true);
      localStorage.setItem("wallet_address", publicKey);
      localStorage.setItem("auth_token", "wallet_connected");
      console.log("Wallet connected successfully:", publicKey);
    } catch (error) {
      console.error("Wallet connection failed:", error);
      
      let errorMessage = "Failed to connect to wallet. Please try again.";
      
      if (error.message.includes("User declined access") || error.message.includes("denied")) {
        errorMessage = "Please approve the connection request in your Freighter wallet.";
      } else if (error.message.includes("not installed")) {
        errorMessage = "Please install Freighter wallet extension from https://www.freighter.app/";
      }
      
      alert(errorMessage);
    }
  };

  const disconnectWallet = () => {
    setIsWalletConnected(false);
    setWalletAddress("");
    setShowDropdown(false);
    localStorage.removeItem("wallet_address");
    localStorage.removeItem("auth_token");
  };

  const formatAddress = (address) => {
    if (!address) return "";
    return `${address.slice(0, 4)}...${address.slice(-4)}`;
  };

  return (
    <nav style={styles.navbar}>
      <div style={styles.logo}>
        <span style={styles.logoIcon}>🏡</span> LandChain
      </div>

      <div style={styles.navContainer}>
        <ul style={styles.navLinks}>
          <li>
            <NavLink
              to="/home"
              style={({ isActive }) => ({
                ...styles.link,
                ...(isActive ? styles.active : {}),
              })}
            >
              Home
            </NavLink>
          </li>
          <li>
            <NavLink
              to="/about"
              style={({ isActive }) => ({
                ...styles.link,
                ...(isActive ? styles.active : {}),
              })}
            >
              About
            </NavLink>
          </li>
          <li>
            <NavLink
              to="/contact"
              style={({ isActive }) => ({
                ...styles.link,
                ...(isActive ? styles.active : {}),
              })}
            >
              Contact
            </NavLink>
          </li>

          {isWalletConnected && (
            <>
              <li>
                <NavLink
                  to="/register-land"
                  style={({ isActive }) => ({
                    ...styles.link,
                    ...(isActive ? styles.active : {}),
                  })}
                >
                  Register Land
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="/properties"
                  style={({ isActive }) => ({
                    ...styles.link,
                    ...(isActive ? styles.active : {}),
                  })}
                >
                  Properties
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="/profile"
                  style={({ isActive }) => ({
                    ...styles.link,
                    ...(isActive ? styles.active : {}),
                  })}
                >
                  Profile
                </NavLink>
              </li>
            </>
          )}
        </ul>

        <div style={styles.walletSection}>
          {!isWalletConnected ? (
            <button style={styles.connectButton} onClick={connectWallet}>
              Connect Wallet
            </button>
          ) : (
            <div style={styles.walletInfo}>
              <button
                style={styles.walletButton}
                onClick={() => setShowDropdown(!showDropdown)}
              >
                <span style={styles.walletIcon}>👤</span>
                {formatAddress(walletAddress)}
              </button>
              {showDropdown && (
                <div style={styles.dropdown}>
                  <NavLink to="/profile" style={styles.dropdownItem}>
                    View Profile
                  </NavLink>
                  <button
                    style={styles.dropdownItem}
                    onClick={disconnectWallet}
                  >
                    Disconnect Wallet
                  </button>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </nav>
  );
}

const styles = {
  navbar: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '1rem 2rem',
    background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    color: '#fff',
    flexWrap: 'wrap',
    boxShadow: '0 4px 20px rgba(0, 0, 0, 0.15)',
    position: 'sticky',
    top: 0,
    zIndex: 1000,
    backdropFilter: 'blur(10px)',
  },
  logo: {
    fontWeight: '800',
    fontSize: '1.8rem',
    color: '#ffffff',
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem',
    textShadow: '2px 2px 4px rgba(0, 0, 0, 0.3)',
    letterSpacing: '0.5px',
  },
  logoIcon: {
    fontSize: '1.5rem',
    filter: 'drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3))',
  },
  navContainer: {
    display: 'flex',
    alignItems: 'center',
    gap: '2rem',
    flexWrap: 'wrap',
  },
  navLinks: {
    listStyle: 'none',
    display: 'flex',
    flexWrap: 'wrap',
    gap: '0.3rem',
    margin: 0,
    padding: 0,
  },
  link: {
    textDecoration: 'none',
    color: '#e8f4f8',
    fontSize: '0.95rem',
    fontWeight: '500',
    padding: '0.7rem 1.2rem',
    borderRadius: '25px',
    transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
    position: 'relative',
    overflow: 'hidden',
    display: 'inline-block',
    letterSpacing: '0.3px',
  },
  active: {
    fontWeight: '700',
    color: '#ffffff',
    backgroundColor: 'rgba(255, 255, 255, 0.2)',
    boxShadow: '0 4px 15px rgba(0, 0, 0, 0.2)',
    transform: 'translateY(-1px)',
    border: '1px solid rgba(255, 255, 255, 0.3)',
  },
  walletSection: {
    position: 'relative',
  },
  connectButton: {
    backgroundColor: '#1abc9c',
    color: '#ffffff',
    border: 'none',
    padding: '0.7rem 1.5rem',
    borderRadius: '25px',
    fontSize: '0.95rem',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'all 0.3s ease',
    boxShadow: '0 4px 15px rgba(26, 188, 156, 0.3)',
  },
  walletInfo: {
    position: 'relative',
  },
  walletButton: {
    backgroundColor: 'rgba(255, 255, 255, 0.2)',
    color: '#ffffff',
    border: '1px solid rgba(255, 255, 255, 0.3)',
    padding: '0.7rem 1.2rem',
    borderRadius: '25px',
    fontSize: '0.9rem',
    fontWeight: '500',
    cursor: 'pointer',
    transition: 'all 0.3s ease',
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem',
  },
  walletIcon: {
    fontSize: '1rem',
  },
  dropdown: {
    position: 'absolute',
    top: '100%',
    right: 0,
    marginTop: '0.5rem',
    backgroundColor: '#ffffff',
    borderRadius: '10px',
    boxShadow: '0 10px 30px rgba(0, 0, 0, 0.2)',
    overflow: 'hidden',
    minWidth: '180px',
    zIndex: 1001,
  },
  dropdownItem: {
    display: 'block',
    width: '100%',
    padding: '0.8rem 1rem',
    color: '#333',
    textDecoration: 'none',
    fontSize: '0.9rem',
    border: 'none',
    backgroundColor: 'transparent',
    cursor: 'pointer',
    transition: 'background-color 0.2s ease',
    textAlign: 'left',
  }
};

export default Navbar;