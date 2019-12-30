import { Board } from 'snek'

const board = Board.new('board', 32, 32)
board.draw()
let start = null

const renderLoop = timestamp => {
  if (!start) start = timestamp
  var progress = timestamp - start
  if (progress > 100) {
    start = timestamp
    board.tick()
    board.draw()
  }
  requestAnimationFrame(renderLoop)
}

requestAnimationFrame(renderLoop)
