import './StageButton.css'

type ConfigureButtonProps = {
  onClick: () => void
  disabled?: boolean
}

/**
 * A secondary button that opens the configuration of a stage.
 */
function ConfigureButton({ onClick, disabled }: ConfigureButtonProps) {
  return (
    <button type="button" className="stage-button" onClick={onClick} disabled={disabled}>
      Configure
    </button>
  )
}

export default ConfigureButton
