const movement = {
  forward: false,
  backward: false,
  left: false,
  right: false,
  jump: false,
  sprint: false,
  flyUp: false,
  flyDown: false
};
window.addEventListener(
  "keydown",
  (e) => {
    switch (e.code) {
      case "KeyW":
        movement.forward = true;
        break;
      case "KeyS":
        movement.backward = true;
        break;
      case "KeyA":
        movement.left = true;
        break;
      case "KeyD":
        movement.right = true;
        break;
      case "Space":
        movement.jump = true;
        break;
      case "ShiftLeft":
        movement.sprint = true;
        break;
      case "KeyE":
        movement.flyUp = true;
        break;
      case "KeyQ":
        movement.flyDown = true;
        break;
    }
  }
);
window.addEventListener(
  "keyup",
  (e) => {
    switch (e.code) {
      case "KeyW":
        movement.forward = false;
        break;
      case "KeyS":
        movement.backward = false;
        break;
      case "KeyA":
        movement.left = false;
        break;
      case "KeyD":
        movement.right = false;
        break;
      case "Space":
        movement.jump = false;
        break;
      case "ShiftLeft":
        movement.sprint = false;
        break;
      case "KeyE":
        movement.flyUp = false;
        break;
      case "KeyQ":
        movement.flyDown = false;
        break;
    }
  }
);
export default movement;
