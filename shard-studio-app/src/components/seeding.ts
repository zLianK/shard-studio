import { showError } from './ErrorToast'

export type Distribution = 'uniform' | 'zipfian'

/** Seeding settings, kept as strings so they can be bound to inputs. */
export type SeedConfig = {
  distribution: Distribution
  n: string
  k: string
}

export const defaultSeedConfig: SeedConfig = {
  distribution: 'uniform',
  n: '1000000',
  k: '1000',
}

/**
 * Starts the seeding with the given settings. Errors are logged to the console
 * and shown to the user as a friendly message.
 */
export async function startSeeding({ distribution, n, k }: SeedConfig): Promise<void> {
  const count = Number(n)
  const keys = Number(k)

  if (!Number.isInteger(count) || count < 1) {
    showError('The number of records must be a positive integer.')
    return
  }
  if (!Number.isInteger(keys) || keys < 1) {
    showError('The number of keys must be a positive integer.')
    return
  }

  try {
    const body = { n: count, k: keys }
    const response = await fetch(`/api/seed/${distribution}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    })
    if (!response.ok) {
      const error = await response.json().catch(() => null)
      console.error(`Seeding failed (${response.status}):`, error?.error ?? error)
      showError('An error occurred while seeding the database.')
    }
  } catch (error) {
    console.error('Could not reach the API:', error)
    showError('Could not reach the API.')
  }
}
