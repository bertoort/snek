import { Game, Canvas } from 'snek'

const game = Game.new(24, 24)
const canvas = Canvas.new('board', game)

let start = null
canvas.init()

const renderLoop = timestamp => {
  if (!start) start = timestamp
  var progress = timestamp - start
  if (progress > 50) {
    start = timestamp
    canvas.update()
  }
  requestAnimationFrame(renderLoop)
}

requestAnimationFrame(renderLoop)
