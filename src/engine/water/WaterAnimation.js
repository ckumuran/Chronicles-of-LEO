export function animateWater(
  mesh,
  time
) {
  mesh.position.y =
    Math.sin(time * 0.001) * 0.05;
}
