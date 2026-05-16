export default function LegoSnapEffect({
  position
}) {
  return (
    <mesh position={position}>
      <ringGeometry args={[0.3, 0.5, 16]} />
      <meshBasicMaterial color="white" />
    </mesh>
  );
}
