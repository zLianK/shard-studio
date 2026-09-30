import { useState } from 'react'
import resharding from '../assets/icons/resharding.svg'
import seeding from '../assets/icons/seeding.svg'
import sharding from '../assets/icons/sharding.svg'
import SeedPanel, { type PanelOrigin } from './SeedPanel'
import StageCard from './StageCard'
import { defaultSeedConfig, startSeeding, type SeedConfig } from './seeding'
import './StageCards.css'

function StageCards() {
  // Seeding card position when Configure was clicked; null while the panel is closed.
  const [seedOrigin, setSeedOrigin] = useState<PanelOrigin | null>(null)
  const [seedConfig, setSeedConfig] = useState<SeedConfig>(defaultSeedConfig)
  const [isSeeding, setIsSeeding] = useState(false)

  const handleStartSeeding = async () => {
    setIsSeeding(true)
    await startSeeding(seedConfig)
    setIsSeeding(false)
  }

  const stages = [
    {
      title: 'Seeding',
      description: 'Populate the database with initial data for the experiment.',
      icon: seeding,
      onConfigure: setSeedOrigin,
      onStart: handleStartSeeding,
      running: isSeeding,
    },
    {
      title: 'Sharding',
      description: 'Distribute the data across multiple shards.',
      icon: sharding,
    },
    {
      title: 'Resharding',
      description: 'Redistribute the data when the shard topology changes.',
      icon: resharding,
    },
  ]

  return (
    <div className="stage-cards">
      <div className="stage-grid">
        {stages.map((stage, index) => (
          <StageCard key={stage.title} step={index + 1} {...stage} />
        ))}
      </div>
      {seedOrigin && (
        <SeedPanel
          origin={seedOrigin}
          config={seedConfig}
          onChange={setSeedConfig}
          onClose={() => setSeedOrigin(null)}
        />
      )}
    </div>
  )
}

export default StageCards
