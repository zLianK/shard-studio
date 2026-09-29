import { useEffect, useState } from 'react'
import './ErrorToast.css'

let show: (message: string) => void = () => {}

/**
 * Shows an error toast with the given message.
 * 
 * @param message {@code string} The error message to display.
 */
export function showError(message: string) {
  show(message)
}

/**
 * A toast component that displays error messages.
 */
function ErrorToast() {
  const [message, setMessage] = useState<string | null>(null)
  const [visible, setVisible] = useState(false)

  useEffect(() => {
    show = (next) => {
      setMessage(next)
      setVisible(true)
    }
    return () => {
      show = () => {}
    }
  }, [])

  useEffect(() => {
    if (!visible) return
    const timer = setTimeout(() => setVisible(false), 5000)
    return () => clearTimeout(timer)
  }, [visible, message])

  if (!message) return null

  return (
    <div
      className={`error-toast ${visible ? 'error-toast--in' : 'error-toast--out'}`}
      role="alert"
      onAnimationEnd={() => {
        if (!visible) setMessage(null)
      }}
    >
      {message}
    </div>
  )
}

export default ErrorToast
