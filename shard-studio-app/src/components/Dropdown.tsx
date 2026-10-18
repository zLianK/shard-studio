import { useEffect, useRef, useState, type CSSProperties } from 'react'
import { createPortal } from 'react-dom'
import './Dropdown.css'

export type DropdownOption<T extends string> = {
  value: T
  label: string
  description?: string
}

type DropdownProps<T extends string> = {
  options: DropdownOption<T>[]
  value: T
  onChange: (value: T) => void
  ariaLabel: string
}

/**
 * A dropdown that shows the selected option in a button and the options, with
 * an optional description, in a menu below it.
 */
function Dropdown<T extends string>({ options, value, onChange, ariaLabel }: DropdownProps<T>) {
  const [open, setOpen] = useState(false)
  const rootRef = useRef<HTMLDivElement>(null)
  const menuRef = useRef<HTMLDivElement>(null)
  const [menuStyle, setMenuStyle] = useState<CSSProperties>({})
  const selected = options.find((option) => option.value === value)

  useEffect(() => {
    if (!open) return
    const handleMouseDown = (event: MouseEvent) => {
      const target = event.target as Node
      if (!rootRef.current?.contains(target) && !menuRef.current?.contains(target)) setOpen(false)
    }
    const close = () => setOpen(false)
    document.addEventListener('mousedown', handleMouseDown)
    window.addEventListener('resize', close)
    window.addEventListener('scroll', close, true)
    return () => {
      document.removeEventListener('mousedown', handleMouseDown)
      window.removeEventListener('resize', close)
      window.removeEventListener('scroll', close, true)
    }
  }, [open])

  const toggle = () => {
    if (!open && rootRef.current) {
      const rect = rootRef.current.getBoundingClientRect()
      setMenuStyle({
        top: rect.bottom + 6,
        left: rect.left,
        minWidth: rect.width,
      })
    }
    setOpen(!open)
  }

  return (
    <div className="dropdown" ref={rootRef}>
      <button
        type="button"
        className="dropdown__trigger"
        aria-label={ariaLabel}
        aria-haspopup="listbox"
        aria-expanded={open}
        onClick={toggle}
      >
        {selected?.label}
        <span
          className={`dropdown__chevron${open ? ' dropdown__chevron--open' : ''}`}
          aria-hidden="true"
        />
      </button>

      {open &&
        createPortal(
          <div
            className="dropdown__menu"
            ref={menuRef}
            style={menuStyle}
            role="listbox"
            aria-label={ariaLabel}
          >
            {options.map((option) => {
              const isSelected = option.value === value
              return (
                <button
                  key={option.value}
                  type="button"
                  role="option"
                  aria-selected={isSelected}
                  className="dropdown__option"
                  onClick={() => {
                    onChange(option.value)
                    setOpen(false)
                  }}
                >
                  <span className="dropdown__text">
                    <span className="dropdown__label">{option.label}</span>
                    {option.description && (
                      <span className="dropdown__description">{option.description}</span>
                    )}
                  </span>
                  {isSelected && <span className="dropdown__check" aria-hidden="true" />}
                </button>
              )
            })}
          </div>,
          document.body,
        )}
    </div>
  )
}

export default Dropdown
