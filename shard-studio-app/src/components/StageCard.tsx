import type { CSSProperties } from 'react'
import './StageCard.css'

type StageCardProps = {
  icon: string
  step: number
  title: string
  description: string
}

function StageCard({ step, title, description, icon }: StageCardProps) {
  return (
    <article className="stage-card" aria-disabled="true">
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
      <span className="stage-card__badge">Coming soon!</span>
    </article>
  )
}

export default StageCard
