export default class RenderQueue {
  constructor() {
    this.queue = [];
  }
  add(task) {
    this.queue.push(task);
  }
  process() {
    while (this.queue.length > 0) {
      const task =
        this.queue.shift();
      task();
    }
  }
}
