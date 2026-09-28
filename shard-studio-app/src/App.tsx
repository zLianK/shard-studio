import './App.css'
import StageCards from './components/StageCards'
import ThemeToggle from './components/ThemeToggle'

function App() {
  return (
    <main className="app">
      <section className="panel">
        <header className="panel__header">
          <div className="panel__top">
            <div className="panel__brand">
              <img className="panel__logo" src="/favicon.svg" alt="" />
              <h1 className="panel__title">Shard Studio</h1>
            </div>
            <ThemeToggle />
          </div>
          <p className="panel__subtitle">
            Follow each stage of the experiment. The features below will be implemented in the
            upcoming stages.
          </p>
        </header>

        <h2 className="section-label">Stages</h2>
        <StageCards />
      </section>
    </main>
  )
}

export default App
