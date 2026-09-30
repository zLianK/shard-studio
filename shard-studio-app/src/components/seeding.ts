import { showError } from './ErrorToast'

export type Distribution = 'uniform' | 'zipfian'

/** Seeding settings, kept as strings so they can be bound to inputs. */
export type SeedConfig = {
  distribution: Distribution
  n: string
  s: string
}

export const defaultSeedConfig: SeedConfig = {
  distribution: 'uniform',
  n: '1000',
  s: '1.1',
}

/**
 * Starts the seeding with the given settings. Errors are logged to the console
 * and shown to the user as a friendly message.
 */
export async function startSeeding({ distribution, n, s }: SeedConfig): Promise<void> {
  const count = Number(n)
  const skew = Number(s)

  if (!Number.isInteger(count) || count < 1) {
    showError('The number of records must be a positive integer.')
    return
  }
  if (distribution === 'zipfian' && (!Number.isFinite(skew) || skew < 0)) {
    showError('The skew must be a number greater than or equal to 0.')
    return
  }

  try {
    const body = distribution === 'uniform' ? { n: count } : { n: count, s: skew }
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
