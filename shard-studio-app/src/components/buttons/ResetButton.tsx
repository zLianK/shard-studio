import { useState } from 'react'
import { post } from '../api'
import './IconButton.css'
import './ResetButton.css'

/**
 * A button that resets the simulation.
 */
function ResetButton() {
  const [loading, setLoading] = useState(false)

  const handleReset = async () => {
    setLoading(true)
    await post('/reset', { error: 'An error occurred while resetting the simulation.' })
    setLoading(false)
  }

  return (
    <button
      type="button"
      className="icon-button reset-button"
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
