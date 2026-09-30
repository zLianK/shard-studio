import './App.css'
import ResetButton from './components/buttons/ResetButton'
import ThemeButton from './components/buttons/ThemeButton'
import ErrorToast from './components/ErrorToast'
import StageCards from './components/StageCards'

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
            <div className="panel__actions">
              <ResetButton />
              <ThemeButton />
            </div>
          </div>
          <p className="panel__subtitle">
            Follow each stage of the experiment. The features below will be implemented in the
            upcoming stages.
          </p>
        </header>

        <h2 className="section-label">Stages</h2>
        <StageCards />
      </section>
      <ErrorToast />
    </main>
  )
}

export default App
