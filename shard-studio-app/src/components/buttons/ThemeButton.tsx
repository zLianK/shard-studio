import { useState, useEffect } from 'react'
import './IconButton.css'
import './ThemeButton.css'

function ThemeButton() {
  const [theme, setTheme] = useState(() => {
    return localStorage.getItem('theme') === 'light' ? 'light' : 'dark'
  })

  useEffect(() => {
    document.documentElement.dataset.theme = theme
    localStorage.setItem('theme', theme)
  }, [theme])

  return (
    <button
      type="button"
      className="icon-button theme-button"
      onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
      aria-label="Switch theme"
      title="Switch theme"
    >
      <span className="theme-button__icon theme-button__icon--sun" aria-hidden="true" />
      <span className="theme-button__icon theme-button__icon--moon" aria-hidden="true" />
    </button>
  )
}

export default ThemeButton
