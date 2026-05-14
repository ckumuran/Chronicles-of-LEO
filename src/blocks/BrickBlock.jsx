import { RigidBody } from '@react-three/rapier'
export default function BrickBlock({ position, color }) {
  return (
    <RigidBody type="fixed">
      <group position={position}>
        <mesh castShadow receiveShadow>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial
            color={color}
            roughness={0.25}
            metalness={0.1}
          />
        </mesh>
        <mesh
          castShadow
          position={[0, 0.52, 0]}
        >
          <cylinderGeometry
            args={[0.18, 0.18, 0.08, 32]}
          />
          <meshStandardMaterial
            color={color}
            roughness={0.2}
            metalness={0.1}
          />
        </mesh>
      </group>
    </RigidBody>
  )
}
