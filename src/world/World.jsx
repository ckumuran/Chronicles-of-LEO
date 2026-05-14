import BrickBlock from '../blocks/BrickBlock'
export default function World() {
  const size = 30
  const blocks = []
  for (let x = -size; x < size; x++) {
    for (let z = -size; z < size; z++) {
      const height =
        Math.floor(
          Math.sin(x * 0.3) * 2 +
          Math.cos(z * 0.3) * 2 +
          4
        )
      for (let y = 0; y < height; y++) {
        let color = '#b87a3d'
        if (y === height - 1) {
          color = '#57d657'
        }
        blocks.push(
          <BrickBlock
            key={`${x}-${y}-${z}`}
            position={[x, y, z]}
            color={color}
          />
        )
      }
    }
  }
  return <>{blocks}</>
}
