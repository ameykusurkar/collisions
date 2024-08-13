import init, { Particle, Vec2, World } from "./pkg/collisions.js";

const WIDTH = 1200;
const HEIGHT = 800;

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const dragRange = document.getElementById('dragRange');
const dragValue = document.getElementById('dragValue');
const momentumValue = document.getElementById('momentumValue');

var world = null;
var memory = null;
var mousedown = false;
var phantomParticle = null;

var totalParticles = 0;

init().then((instance) => {
  memory = instance.memory;
  world = World.new(WIDTH, HEIGHT);

  canvas.addEventListener('mousedown', function(event) {
    if (event.button === 0) {
      mousedown = true;
      const rect = canvas.getBoundingClientRect();
      const mx = event.clientX - rect.left;
      const my = event.clientY - rect.top;
      phantomParticle = {
        radius: 15,
        // TODO: Need to subtract border/padding to ensure this is within canvas bounds
        px: mx,
        py: my,
      };
    }
  });

  canvas.addEventListener('mousemove', function(event) {
    if (phantomParticle) {
      const rect = canvas.getBoundingClientRect();
      const mx = event.clientX - rect.left;
      const my = event.clientY - rect.top;
      phantomParticle['currX'] = mx;
      phantomParticle['currY'] = my;
      console.log(mx, my);
    }
  });

  window.addEventListener('mouseup', function(event) {
    if (event.button === 0) {
      // Clean up
      mousedown = false;
      phantomParticle = null;
    }
  });

  canvas.addEventListener('mouseup', function(event) {
    if (event.button === 0) {
      if (mousedown) {
        const rect = canvas.getBoundingClientRect();
        const mx = event.clientX - rect.left;
        const my = event.clientY - rect.top;
        const pos = Vec2.new(phantomParticle['px'], phantomParticle['py']);
        const velK = 2;
        const vel = Vec2.new((phantomParticle['px'] - mx) * velK, (phantomParticle['py'] - my) * velK);
        world.try_push(Particle.new(pos, vel, phantomParticle['radius']));
      }
      mousedown = false;
      phantomParticle = null;
    }
  });

  requestAnimationFrame(renderLoop);
});

const renderLoop = () => {
  // TODO: Calculate frame interval
  world.step_frame(1.0 / 60, 1 - parseFloat(dragRange.value));
  dragValue.textContent = dragRange.value;
  momentumValue.textContent = world.momentum();

  if (totalParticles < 1000) {
    let p = Particle.new(Vec2.new(10, 200), Vec2.new(1000, 0), 7);
    if (world.try_push(p)) {
      totalParticles++;
    }
  }

  render(memory, world);

  requestAnimationFrame(renderLoop);
};

function render(memory, world) {
  const particleBuffer = new Float32Array(memory.buffer, world.particles(), world.num_particles() * 5);
  const colorBuffer = new Uint8Array(memory.buffer, world.colors(), world.num_particles() * 3);

  ctx.clearRect(0, 0, WIDTH, HEIGHT);
  for (let i = 0; i < world.num_particles(); i++) {
    fillCircle(
      ctx,
      particleBuffer[i * 5 + 0], // pos x
      particleBuffer[i * 5 + 1], // pos y
      particleBuffer[i * 5 + 4], // radius
      `rgb(${colorBuffer[i * 3 + 0]} ${colorBuffer[i * 3 + 1]} ${colorBuffer[i * 3 + 2]})`,
    )
  }

  if (phantomParticle) {
    fillCircle(
      ctx,
      phantomParticle['px'],
      phantomParticle['py'],
      phantomParticle['radius'],
      'rgb(255 200 200)',
    )

    if (phantomParticle['currX']) {
      ctx.beginPath();
      ctx.moveTo(phantomParticle['px'], phantomParticle['py']);
      ctx.lineTo(phantomParticle['currX'], phantomParticle['currY']);
      ctx.closePath();
      ctx.stroke();
    }
  }
}

function fillCircle(ctx, x, y, radius, color) {
  ctx.strokeStyle = color;
  ctx.beginPath();
  ctx.arc(x, y, radius, 0, Math.PI * 2, true);
  ctx.fillStyle = color;
  ctx.fill();
  ctx.stroke();
}
