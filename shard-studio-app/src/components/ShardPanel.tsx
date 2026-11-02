import Dropdown, { type DropdownOption } from './Dropdown'
import { MAX_SHARDS, MIN_SHARDS, type ShardConfig, type ShardStrategy } from './sharding'
import StagePanel, { type PanelOrigin } from './StagePanel'

const strategies: DropdownOption<ShardStrategy>[] = [
  { value: 'hash', label: 'Hash', description: 'The hash of the key picks the shard.' },
]

type ShardPanelProps = {
  origin: PanelOrigin
  config: ShardConfig
  onChange: (config: ShardConfig) => void
  onClose: () => void
}

/**
 * The panel to configure the sharding.
 */
function ShardPanel({ origin, config, onChange, onClose }: ShardPanelProps) {
  return (
    <StagePanel title="Sharding" origin={origin} onClose={onClose}>
      <div className="stage-panel__field">
        <span>Strategy</span>
        <Dropdown
          ariaLabel="Strategy"
          options={strategies}
          value={config.strategy}
          onChange={(strategy) => onChange({ ...config, strategy })}
        />
      </div>

      <label className="stage-panel__field">
        <span>Number of shards</span>
        <input
          type="number"
          min={MIN_SHARDS}
          max={MAX_SHARDS}
          step={1}
          value={config.shards}
          onChange={(event) => onChange({ ...config, shards: event.target.value })}
        />
      </label>
    </StagePanel>
  )
}

export default ShardPanel
