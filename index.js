import init, { Particle, Vec2, World } from "./pkg/collisions.js";
const WIDTH = 1200;
const HEIGHT = 800;
const RADIUS = 7;
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const dragRange = document.getElementById('dragRange');
const dragValue = document.getElementById('dragValue');
const fpsValue = document.getElementById('fpsValue');
const totalParticlesValue = document.getElementById('totalParticlesValue');
const collisionChecksPerSecondValue = document.getElementById('collisionChecksPerSecondValue');
const addParticles = document.getElementById('addParticles');
let world;
let memory;
let mousedown = false;
let phantomParticle = null;
let segments = [];
let totalParticles = 0;
let lastReported = 0;
let frameCount = 0;
let collisionCheckCount = 0;
init().then((instance) => {
    addParticles.checked = true;
    memory = instance.memory;
    world = World.new(WIDTH, HEIGHT);
    // Add a box
    addSegment(600, 500, 1200, 500); // Bottom
    addSegment(600, 500, 600, 400); // Left
    addSegment(1200, 500, 1200, 400); // Right
    canvas.addEventListener('mousedown', function (event) {
        if (event.button === 0) {
            mousedown = true;
            const rect = canvas.getBoundingClientRect();
            const mx = event.clientX - rect.left;
            const my = event.clientY - rect.top;
            phantomParticle = {
                radius: RADIUS,
                // TODO: Need to subtract border/padding to ensure this is within canvas bounds
                posX: mx,
                posY: my,
                currX: mx,
                currY: my,
            };
        }
    });
    canvas.addEventListener('mousemove', function (event) {
        if (phantomParticle) {
            const rect = canvas.getBoundingClientRect();
            const mx = event.clientX - rect.left;
            const my = event.clientY - rect.top;
            phantomParticle.currX = mx;
            phantomParticle.currY = my;
        }
    });
    window.addEventListener('mouseup', function (event) {
        if (event.button === 0) {
            // Clean up
            mousedown = false;
            phantomParticle = null;
        }
    });
    canvas.addEventListener('mouseup', function (event) {
        if (event.button === 0) {
            if (mousedown && phantomParticle) {
                const rect = canvas.getBoundingClientRect();
                const mx = event.clientX - rect.left;
                const my = event.clientY - rect.top;
                const pos = Vec2.new(phantomParticle.posX, phantomParticle.posY);
                const velK = 2;
                const vel = Vec2.new((phantomParticle.posX - mx) * velK, (phantomParticle.posY - my) * velK);
                tryAddParticle(Particle.new(pos, vel, phantomParticle.radius));
            }
            mousedown = false;
            phantomParticle = null;
        }
    });
    requestAnimationFrame(renderLoop);
});
const renderLoop = () => {
    var _a;
    const alg = parseInt(((_a = document.querySelector('input[name="collision"]:checked')) === null || _a === void 0 ? void 0 : _a.value) || "0");
    // TODO: Calculate frame interval
    collisionCheckCount += world.step_frame(1.0 / 60, 1 - parseFloat(dragRange.value), 8, alg);
    dragValue.textContent = dragRange.value;
    totalParticlesValue.textContent = totalParticles.toString();
    frameCount++;
    const now = Date.now();
    const elapsed = now - lastReported;
    if (elapsed > 1000) {
        fpsValue.textContent = (1000 * frameCount / elapsed).toFixed(2);
        collisionChecksPerSecondValue.textContent = (1000 * collisionCheckCount / elapsed).toLocaleString();
        collisionCheckCount = 0;
        frameCount = 0;
        lastReported = now;
    }
    if (addParticles.checked) {
        for (let i = 0; i < 5; i++) {
            let p = Particle.new(Vec2.new(10, 20 + i * 3 * RADIUS), Vec2.new(1000, 0), RADIUS);
            tryAddParticle(p);
        }
    }
    render(memory, world);
    requestAnimationFrame(renderLoop);
};
function render(memory, world) {
    const particleBuffer = new Float32Array(memory.buffer, world.particles(), world.num_particles() * 5);
    ctx.clearRect(0, 0, WIDTH, HEIGHT);
    const num_particles = world.num_particles();
    for (let i = 0; i < num_particles; i++) {
        const red = Math.ceil(255 - 255 * (i / num_particles));
        const blue = Math.ceil(255 * (i / num_particles));
        fillCircle(particleBuffer[i * 5 + 0], // pos x
        particleBuffer[i * 5 + 1], // pos y
        particleBuffer[i * 5 + 4], // radius
        `rgb(${red} ${0} ${blue})`);
    }
    for (const segI in segments) {
        const seg = segments[segI];
        ctx.strokeStyle = 'black';
        ctx.beginPath();
        ctx.moveTo(seg[0], seg[1]);
        ctx.lineTo(seg[2], seg[3]);
        ctx.stroke();
    }
    if (phantomParticle) {
        fillCircle(phantomParticle.posX, phantomParticle.posY, phantomParticle.radius, 'rgb(255 200 200)');
        if (phantomParticle.currX) {
            ctx.beginPath();
            ctx.moveTo(phantomParticle.posX, phantomParticle.posY);
            ctx.lineTo(phantomParticle.currX, phantomParticle.currY);
            ctx.closePath();
            ctx.stroke();
        }
    }
}
function addSegment(start1, end1, start2, end2) {
    segments.push([start1, end1, start2, end2]);
    world.push_segment(Vec2.new(start1, end1), Vec2.new(start2, end2));
}
function tryAddParticle(p) {
    if (world.try_push(p)) {
        totalParticles++;
    }
}
function fillCircle(x, y, radius, color) {
    ctx.strokeStyle = color;
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, Math.PI * 2, true);
    ctx.fillStyle = color;
    ctx.fill();
    ctx.stroke();
}
