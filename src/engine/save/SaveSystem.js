export function saveWorld(blocks) {
  localStorage.setItem(
    "brixit_world",
    JSON.stringify(blocks)
  );
}
export function loadWorld() {
  return JSON.parse(
    localStorage.getItem("brixit_world")
  );
}
