import { useEffect, useState, type ReactNode, type TransitionEvent } from 'react'
import BackButton from './buttons/BackButton'
import './StagePanel.css'

/** Position and size of the card, relative to the container the panel lives in. */
export type PanelOrigin = {
  top: number
  left: number
  width: number
  height: number
}

type StagePanelProps = {
  title: string
  /** The card the panel expands from (and collapses back into). */
  origin: PanelOrigin
  onClose: () => void
  /** The fields of the stage, wrapped in `.stage-panel__field` elements. */
  children: ReactNode
}

/**
 * A panel to configure a stage. It expands from the stage card to fill its
 * container (the cards area) and collapses back.
 * Must be rendered inside a `position: relative` container.
 */
function StagePanel({ title, origin, onClose, children }: StagePanelProps) {
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
      className={`stage-panel${expanded ? ' stage-panel--expanded' : ''}`}
      style={rect}
      role="dialog"
      aria-labelledby="stage-panel-title"
      onTransitionEnd={handleTransitionEnd}
    >
      <div className="stage-panel__content">
        <header className="stage-panel__header">
          <BackButton onClick={() => setExpanded(false)} />
          <h2 className="stage-panel__title" id="stage-panel-title">
            {title}
          </h2>
        </header>

        <div className="stage-panel__fields">{children}</div>
      </div>
    </section>
  )
}

export default StagePanel
