import { useRef, type CSSProperties } from 'react'
import ConfigureButton from './buttons/ConfigureButton'
import StartButton from './buttons/StartButton'
import type { PanelOrigin } from './SeedPanel'
import './StageCard.css'

type StageCardProps = {
  icon: string
  step: number
  title: string
  description: string
  onConfigure?: (origin: PanelOrigin) => void
  onStart?: () => void
  running?: boolean
}

function StageCard({
  step,
  title,
  description,
  icon,
  onConfigure,
  onStart,
  running,
}: StageCardProps) {
  const cardRef = useRef<HTMLElement>(null)

  const handleConfigure = () => {
    const card = cardRef.current!
    onConfigure?.({
      top: card.offsetTop,
      left: card.offsetLeft,
      width: card.offsetWidth,
      height: card.offsetHeight,
    })
  }

  return (
    <article ref={cardRef} className="stage-card">
      <div className="stage-card__header">
        <span className="stage-card__icon">
          <span
            className="stage-card__glyph"
            style={{ '--icon': `url("${icon}")` } as CSSProperties}
            aria-hidden="true"
          />
        </span>
        <span className="stage-card__step">Step {step}</span>
      </div>
      <h3 className="stage-card__title">{title}</h3>
      <p className="stage-card__description">{description}</p>
      <div className="stage-card__actions">
        <ConfigureButton onClick={handleConfigure} disabled={running} />
        <StartButton onClick={onStart} disabled={running} />
      </div>
    </article>
  )
}

export default StageCard
