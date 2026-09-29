import { useState } from 'react'
import { showError } from './ErrorToast'
import './ResetButton.css'

/**
 * A button that resets the simulation.
 */
function ResetButton() {
  const [loading, setLoading] = useState(false)

  const handleReset = async () => {
    setLoading(true)
    try {
      const response = await fetch('/api/reset', { method: 'POST' })
      if (!response.ok) {
        const body = await response.json().catch(() => null)
        console.error(`Reset failed (${response.status}):`, body?.error ?? body)
        showError('An error occurred while resetting the simulation.')
      }
    } catch (error) {
      console.error('Could not reach the API:', error)
      showError('Could not reach the API.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <button
      type="button"
      className="reset-button"
      onClick={handleReset}
      disabled={loading}
      aria-label="Reset simulation"
      title="Reset simulation"
    >
      <span
        className={`reset-button__icon${loading ? ' reset-button__icon--spinning' : ''}`}
        aria-hidden="true"
      />
      <span className="reset-button__label">Reset</span>
    </button>
  )
}

export default ResetButton
