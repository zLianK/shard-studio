import Dropdown, { type DropdownOption } from './Dropdown'
import type { Distribution, SeedConfig } from './seeding'
import StagePanel, { type PanelOrigin } from './StagePanel'

const distributions: DropdownOption<Distribution>[] = [
  { value: 'uniform', label: 'Uniform', description: 'Keys are spread evenly.' },
  { value: 'zipfian', label: 'Zipfian', description: 'A few keys are far more frequent.' },
  {
    value: 'hotspot', label: 'Hotspot', description: '20% of the keys holds 80% of the rows.',
  },
]

type SeedPanelProps = {
  origin: PanelOrigin
  config: SeedConfig
  onChange: (config: SeedConfig) => void
  onClose: () => void
}

/**
 * The panel to configure the database seeding.
 */
function SeedPanel({ origin, config, onChange, onClose }: SeedPanelProps) {
  return (
    <StagePanel title="Seeding" origin={origin} onClose={onClose}>
      <div className="stage-panel__field">
        <span>Distribution</span>
        <Dropdown
          ariaLabel="Distribution"
          options={distributions}
          value={config.distribution}
          onChange={(distribution) => onChange({ ...config, distribution })}
        />
      </div>

      <label className="stage-panel__field">
        <span>Number of records (n)</span>
        <input
          type="number"
          min={1}
          step={1}
          value={config.n}
          onChange={(event) => onChange({ ...config, n: event.target.value })}
        />
      </label>

      <label className="stage-panel__field">
        <span>Key space size (k)</span>
        <input
          type="number"
          min={1}
          step={1}
          value={config.k}
          onChange={(event) => onChange({ ...config, k: event.target.value })}
        />
      </label>
    </StagePanel>
  )
}

export default SeedPanel
