import { useState } from 'react'
import { showError } from '../ErrorToast'
import './IconButton.css'
import './SaveReportButton.css'

/**
 * A button that saves the report of the current experiment to disk.
 */
function SaveReportButton() {
  const [loading, setLoading] = useState(false)

  const handleSave = async () => {
    setLoading(true)
    try {
      const response = await fetch('/api/report', { method: 'POST' })
      if (!response.ok) {
        const body = await response.json().catch(() => null)
        console.error(`Saving the report failed (${response.status}):`, body?.error ?? body)
        showError(body?.error ?? 'An error occurred while saving the report.')
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
      className="icon-button save-report-button"
      onClick={handleSave}
      disabled={loading}
      aria-label="Save report"
      title="Save report"
    >
      <span className="save-report-button__icon" aria-hidden="true" />
      <span className="save-report-button__label">Save</span>
    </button>
  )
}

export default SaveReportButton
