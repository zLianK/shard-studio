import { useState, useEffect } from 'react';
import './ThemeToggle.css'

function ThemeToggle() {
  const [theme, setTheme] = useState(() => {
    return localStorage.getItem('theme') === 'light' ? 'light' : 'dark';
  });

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    localStorage.setItem('theme', theme);
  }, [theme]);

  return (
    <button
      type="button"
      className="theme-toggle"
      onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
      aria-label={`Switch theme`}
      title={`Switch theme`}
    >
      <span className="theme-toggle__icon theme-toggle__icon--sun" aria-hidden="true" />
      <span className="theme-toggle__icon theme-toggle__icon--moon" aria-hidden="true" />
    </button>
  )
}

export default ThemeToggle
