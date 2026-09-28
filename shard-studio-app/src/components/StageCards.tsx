import resharding from '../assets/icons/resharding.svg'
import seeding from '../assets/icons/seeding.svg'
import sharding from '../assets/icons/sharding.svg'
import StageCard from './StageCard'
import './StageCards.css'

const stages = [
  {
    title: 'Seeding',
    description: 'Populate the database with initial data for the experiment.',
    icon: seeding,
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

function StageCards() {
  return (
    <div className="stage-grid">
      {stages.map((stage, index) => (
        <StageCard key={stage.title} step={index + 1} {...stage} />
      ))}
    </div>
  )
}

export default StageCards
