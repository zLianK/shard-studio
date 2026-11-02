import { useState } from 'react'
import { post } from '../api'
import './IconButton.css'
import './SaveReportButton.css'

/**
 * A button that saves the report of the current experiment to disk.
 */
function SaveReportButton() {
  const [loading, setLoading] = useState(false)

  const handleSave = async () => {
    setLoading(true)
    await post('/report', { error: 'An error occurred while saving the report.' })
    setLoading(false)
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
