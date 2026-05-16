export default function BlockParticles({
  position,
  color
}) {
  return (
    <mesh position={position}>
      <sphereGeometry args={[0.1, 8, 8]} />
      <meshStandardMaterial color={color} />
    </mesh>
  );
}
