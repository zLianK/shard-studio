import './StageButton.css'
import './StartButton.css'

type StartButtonProps = {
  onClick?: () => void
  disabled?: boolean
}

/**
 * A highlighted button with a play icon that starts a stage.
 */
function StartButton({ onClick, disabled }: StartButtonProps) {
  return (
    <button
      type="button"
      className="stage-button start-button"
      onClick={onClick}
      disabled={disabled}
    >
      {disabled ? "Running..." : "Start"}
      <span className="start-button__icon" aria-hidden="true" />
    </button>
  )
}

export default StartButton
