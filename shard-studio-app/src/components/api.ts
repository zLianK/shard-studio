import { showError } from './ErrorToast'

type PostOptions = {
  body?: unknown
  error: string
}

/**
 * Sends a POST request to the API, showing any failure to the user.
 *
 * @returns Whether the request succeeded.
 */
export async function post(path: string, { body, error }: PostOptions): Promise<boolean> {
  try {
    const request: RequestInit = { method: 'POST' }
    if (body !== undefined) {
      request.headers = { 'Content-Type': 'application/json' }
      request.body = JSON.stringify(body)
    }

    const response = await fetch(`/api${path}`, request)
    if (response.ok) return true

    const apiError = await response.json().catch(() => null)
    console.error(`Request to ${path} failed (${response.status}):`, apiError?.error ?? apiError)
    showError(apiError?.error ?? error)
  } catch (exception) {
    console.error('Could not reach the API:', exception)
    showError('Could not reach the API.')
  }
  return false
}
