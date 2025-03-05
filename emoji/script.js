// Constants
const GAME_WIDTH = 400;
const GAME_HEIGHT = 600;
const PLAYER_SPEED = 5;
const BULLET_SPEED = 10;
const ENEMY_SPEED = 2;
const SHOOT_COOLDOWN = 500; // milliseconds
const SPAWN_INTERVAL = 1000; // milliseconds

// Game state
let player = { x: GAME_WIDTH / 2 - 25, y: GAME_HEIGHT - 50, width: 50, height: 50 };
let enemies = [];
let bullets = [];
let score = 0;
let lives = 3;
let lastShootTime = 0;
let movingLeft = false;
let movingRight = false;
let gameLoopId;
let spawnId;

// DOM elements
const playerDiv = document.getElementById('player');
const enemiesDiv = document.getElementById('enemies');
const bulletsDiv = document.getElementById('bullets');
const scoreDiv = document.getElementById('score');
const livesDiv = document.getElementById('lives');
const gameOverDiv = document.getElementById('game-over');

// Initialize game
function init() {
    score = 0;
    lives = 3;
    enemies = [];
    bullets = [];
    player.x = GAME_WIDTH / 2 - player.width / 2;
    scoreDiv.textContent = 'Score: 0';
    livesDiv.textContent = 'Lives: 3';
    gameOverDiv.style.display = 'none';
    enemiesDiv.innerHTML = '';
    bulletsDiv.innerHTML = '';
    gameLoopId = setInterval(gameLoop, 16); // ~60fps
    spawnId = setInterval(spawnEnemy, SPAWN_INTERVAL);
}

// Main game loop
function gameLoop() {
    if (movingLeft) player.x = Math.max(0, player.x - PLAYER_SPEED);
    if (movingRight) player.x = Math.min(GAME_WIDTH - player.width, player.x + PLAYER_SPEED);
    moveBullets();
    moveEnemies();
    checkCollisions();
    render();
    if (lives <= 0) gameOver();
}

// Move bullets up
function moveBullets() {
    for (let i = bullets.length - 1; i >= 0; i--) {
        bullets[i].y -= BULLET_SPEED;
        if (bullets[i].y < 0) removeBullet(i);
    }
}

// Move enemies down
function moveEnemies() {
    for (let i = enemies.length - 1; i >= 0; i--) {
        enemies[i].y += ENEMY_SPEED;
        if (enemies[i].y > GAME_HEIGHT) {
            removeEnemy(i);
            lives--;
            livesDiv.textContent = `Lives: ${lives}`;
        }
    }
}

// Check collisions
function checkCollisions() {
    for (let i = bullets.length - 1; i >= 0; i--) {
        for (let j = enemies.length - 1; j >= 0; j--) {
            if (checkOverlap(bullets[i], enemies[j])) {
                removeBullet(i);
                removeEnemy(j);
                score += 10;
                scoreDiv.textContent = `Score: ${score}`;
                break; // One bullet hits one enemy
            }
        }
    }
}

// Collision detection
function checkOverlap(a, b) {
    return a.x < b.x + b.width &&
        a.x + a.width > b.x &&
        a.y < b.y + b.height &&
        a.y + a.height > b.y;
}

// Render game state
function render() {
    playerDiv.style.left = player.x + 'px';
    playerDiv.style.top = player.y + 'px';
    enemies.forEach(enemy => {
        enemy.div.style.left = enemy.x + 'px';
        enemy.div.style.top = enemy.y + 'px';
    });
    bullets.forEach(bullet => {
        bullet.div.style.left = bullet.x + 'px';
        bullet.div.style.top = bullet.y + 'px';
    });
}

// Spawn enemy
function spawnEnemy() {
    const x = Math.random() * (GAME_WIDTH - 50);
    const enemy = {
        x,
        y: 0,
        width: 50,
        height: 50,
        div: document.createElement('div')
    };
    enemy.div.className = 'enemy';
    enemy.div.textContent = '👾';
    enemiesDiv.appendChild(enemy.div);
    enemies.push(enemy);
}

// Shoot bullet
function shoot() {
    if (Date.now() - lastShootTime > SHOOT_COOLDOWN) {
        const bullet = {
            x: player.x + player.width / 2 - 2.5,
            y: player.y,
            width: 5,
            height: 10,
            div: document.createElement('div')
        };
        bullet.div.className = 'bullet';
        bulletsDiv.appendChild(bullet.div);
        bullets.push(bullet);
        lastShootTime = Date.now();
    }
}

// Remove bullet
function removeBullet(index) {
    bulletsDiv.removeChild(bullets[index].div);
    bullets.splice(index, 1);
}

// Remove enemy
function removeEnemy(index) {
    enemiesDiv.removeChild(enemies[index].div);
    enemies.splice(index, 1);
}

// Game over
function gameOver() {
    clearInterval(gameLoopId);
    clearInterval(spawnId);
    gameOverDiv.style.display = 'block';
}

// Event listeners
document.addEventListener('keydown', (e) => {
    if (e.key === 'ArrowLeft') movingLeft = true;
    else if (e.key === 'ArrowRight') movingRight = true;
    else if (e.key === ' ') shoot();
    else if (e.key === 'r' && lives <= 0) init();
});

document.addEventListener('keyup', (e) => {
    if (e.key === 'ArrowLeft') movingLeft = false;
    else if (e.key === 'ArrowRight') movingRight = false;
});

// Start game
init();