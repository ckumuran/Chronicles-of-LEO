export default function Crosshair() {
  return (
    <div
      style={{
        position: 'absolute',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        color: 'white',
        fontSize: '24px',
        pointerEvents: 'none',
        userSelect: 'none',
        zIndex: 1000
      }}
    >
      +
    </div>
  )
}
