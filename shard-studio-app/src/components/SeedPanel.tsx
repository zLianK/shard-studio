import { useEffect, useState, type TransitionEvent } from 'react'
import BackButton from './buttons/BackButton'
import Dropdown, { type DropdownOption } from './Dropdown'
import type { Distribution, SeedConfig } from './seeding'
import './SeedPanel.css'

const distributions: DropdownOption<Distribution>[] = [
  { value: 'uniform', label: 'Uniform', description: 'Keys are spread evenly.' },
  { value: 'zipfian', label: 'Zipfian', description: 'A few keys are far more frequent.' },
]

/** Position and size of the card, relative to the container the panel lives in. */
export type PanelOrigin = {
  top: number
  left: number
  width: number
  height: number
}

type SeedPanelProps = {
  /** The card the panel expands from (and collapses back into). */
  origin: PanelOrigin
  config: SeedConfig
  onChange: (config: SeedConfig) => void
  onClose: () => void
}

/**
 * A panel to configure the database seeding. It expands from the Seeding card
 * to fill its container (the cards area) and collapses back.
 * Must be rendered inside a `position: relative` container.
 */
function SeedPanel({ origin, config, onChange, onClose }: SeedPanelProps) {
  const [expanded, setExpanded] = useState(false)

  // Start collapsed on the card, then expand on the next frame so it animates.
  useEffect(() => {
    const frame = requestAnimationFrame(() => setExpanded(true))
    return () => cancelAnimationFrame(frame)
  }, [])

  const handleTransitionEnd = (event: TransitionEvent) => {
    if (event.target === event.currentTarget && event.propertyName === 'height' && !expanded) {
      onClose()
    }
  }

  const rect = expanded ? { top: 0, left: 0, width: '100%', height: '100%' } : origin

  return (
    <section
      className={`seed-panel${expanded ? ' seed-panel--expanded' : ''}`}
      style={rect}
      role="dialog"
      aria-labelledby="seed-panel-title"
      onTransitionEnd={handleTransitionEnd}
    >
      <div className="seed-panel__content">
        <header className="seed-panel__header">
          <BackButton onClick={() => setExpanded(false)} />
          <h2 className="seed-panel__title" id="seed-panel-title">
            Seeding
          </h2>
        </header>

        <div className="seed-panel__fields">
          <div className="seed-panel__field">
            <span>Distribution</span>
            <Dropdown
              ariaLabel="Distribution"
              options={distributions}
              value={config.distribution}
              onChange={(distribution) => onChange({ ...config, distribution })}
            />
          </div>

          <label className="seed-panel__field">
            <span>Number of records (n)</span>
            <input
              type="number"
              min={1}
              step={1}
              value={config.n}
              onChange={(event) => onChange({ ...config, n: event.target.value })}
            />
          </label>

          {config.distribution === 'zipfian' && (
            <label className="seed-panel__field">
              <span>Skew (s)</span>
              <input
                type="number"
                min={0}
                step="any"
                value={config.s}
                onChange={(event) => onChange({ ...config, s: event.target.value })}
              />
            </label>
          )}
        </div>
      </div>
    </section>
  )
}

export default SeedPanel
