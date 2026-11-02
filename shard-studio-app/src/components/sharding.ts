import { post } from './api'
import { showError } from './ErrorToast'

export type ShardStrategy = 'hash'

/** Limits of the number of shards accepted by the API. */
export const MIN_SHARDS = 2
export const MAX_SHARDS = 16

/** Sharding settings, kept as strings so they can be bound to inputs. */
export type ShardConfig = {
  strategy: ShardStrategy
  shards: string
}

export const defaultShardConfig: ShardConfig = {
  strategy: 'hash',
  shards: '4',
}

/**
 * Starts the sharding with the given settings. Errors are logged to the console
 * and shown to the user as a friendly message.
 */
export async function startSharding({ strategy, shards }: ShardConfig): Promise<void> {
  const count = Number(shards)

  if (!Number.isInteger(count) || count < MIN_SHARDS || count > MAX_SHARDS) {
    showError(`The number of shards must be an integer from ${MIN_SHARDS} to ${MAX_SHARDS}.`)
    return
  }

  await post('/shard', {
    body: { strategy, shards: count },
    error: 'An error occurred while sharding the database.',
  })
}
