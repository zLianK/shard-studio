import './IconButton.css'
import './BackButton.css'

type BackButtonProps = {
  onClick: () => void
}

/**
 * A square button with a "<" icon that goes back to the previous view.
 */
function BackButton({ onClick }: BackButtonProps) {
  return (
    <button
      type="button"
      className="icon-button back-button"
      onClick={onClick}
      aria-label="Back"
      title="Back"
    >
      <span className="back-button__icon" aria-hidden="true" />
    </button>
  )
}

export default BackButton
