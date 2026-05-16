export function checkCollision(player, blocks) {
  for (const block of blocks) {
    const dx = Math.abs(player.x - block.x);
    const dy = Math.abs(player.y - block.y);
    const dz = Math.abs(player.z - block.z);
    if (
      dx < 1 &&
      dy < 1 &&
      dz < 1
    ) {
      return true;
    }
  }
  return false;
}
