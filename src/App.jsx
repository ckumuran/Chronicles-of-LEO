import { Canvas } from '@react-three/fiber'
import { Sky, PointerLockControls } from '@react-three/drei'
import { Physics } from '@react-three/rapier'
import World from './world/World'
import Player from './player/Player'
import Crosshair from './ui/Crosshair'
export default function App() {
  return (
    <>
      <Crosshair />
      <Canvas
        shadows
        camera={{
          position: [0, 10, 10],
          fov: 75
        }}
      >
        <Sky sunPosition={[100, 20, 100]} />
        <ambientLight intensity={1.2} />
        <directionalLight
          castShadow
          position={[20, 30, 20]}
          intensity={2}
          shadow-mapSize-width={2048}
          shadow-mapSize-height={2048}
        />
        <Physics>
          <World />
          <Player />
        </Physics>
        <PointerLockControls />
      </Canvas>
    </>
  )
}
