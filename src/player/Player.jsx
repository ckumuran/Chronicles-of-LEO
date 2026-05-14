import { RigidBody, CapsuleCollider } from '@react-three/rapier'
export default function Player() {
  return (
    <RigidBody
      colliders={false}
      mass={1}
      position={[0, 10, 0]}
      enabledRotations={[false, false, false]}
    >
      <CapsuleCollider args={[0.75, 0.5]} />
    </RigidBody>
  )
}
