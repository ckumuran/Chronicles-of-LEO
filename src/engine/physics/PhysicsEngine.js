export default class PhysicsEngine {
  constructor() {
    this.gravity = -0.015;
  }
  applyGravity(player) {
    player.velocity.y += this.gravity;
  }
}
