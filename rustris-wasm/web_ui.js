var game = null;

import init, { RustrisGame } from "./pkg/rustris_wasm.js";
init()
    .then(() => {
        game = RustrisGame.new();
        game.print_info();
    });

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.set(5, 10, 15);
camera.lookAt(5, 10, 0);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

var geometry = new THREE.BoxGeometry();
var material = {
    1: new THREE.MeshBasicMaterial({ color: '#CCCCCC' }),
    2: new THREE.MeshBasicMaterial({ color: '#BBBBBB' }),
    3: new THREE.MeshBasicMaterial({ color: '#AAAAAA' }),
    4: new THREE.MeshBasicMaterial({ color: '#999999' }),
    5: new THREE.MeshBasicMaterial({ color: '#888888' }),
    6: new THREE.MeshBasicMaterial({ color: '#777777' }),
    7: new THREE.MeshBasicMaterial({ color: '#666666' }),
    8: new THREE.MeshBasicMaterial({ color: '#555555' })
};

class Block {
    constructor(x, y) {
        this.x = x;
        this.y = y;
        this.mesh = null;
        this.color = -1;
    }

    setColor(color) {
        if (color == this.color) {
            return; // no change
        }

        if (this.mesh) {
            scene.remove(this.mesh);
            this.mesh = null;
        }

        this.color = color;
        if (this.color == -1) {
            return;
        }

        this.mesh = new THREE.Mesh(geometry, material[this.color]);
        this.mesh.position.set(this.x, this.y, 0);
        scene.add(this.mesh);
    }

    setPos(x, y) {
        this.x = x;
        this.y = y;

        if (this.mesh) {
            this.mesh.position.set(this.x, this.y, 0);
        }
    }

    animate() {
        if (!this.mesh) {
            return;
        }

        this.mesh.rotation.x += 0.01;
        this.mesh.rotation.y += 0.01;
    }
}

var bgBlocks = Array(20);
for (let y = 0; y < bgBlocks.length; y++) {
    bgBlocks[y] = Array(10);
    for (let x = 0; x < bgBlocks[y].length; x++) {
        bgBlocks[y][x] = new Block(x, y);
    }
}

var fgBlocks = Array(4);
for (let y = 0; y < fgBlocks.length; y++) {
    fgBlocks[y] = Array(4);
    for (let x = 0; x < fgBlocks[y].length; x++) {
        fgBlocks[y][x] = new Block(x, y);
    }
}

const animate = function () {
    requestAnimationFrame(animate);
    let update = false;
    if (game) {
        update = game.run_step();
    }

    for (let y = 0; y < bgBlocks.length; y++) {
        for (let x = 0; x < bgBlocks[y].length; x++) {
            if (update) {
                bgBlocks[y][x].setColor(game.board_color_at(x, y));
            }

            bgBlocks[y][x].animate();
        }
    }

    for (let y = 0; y < fgBlocks.length; y++) {
        for (let x = 0; x < fgBlocks[y].length; x++) {
            if (update) {
                fgBlocks[y][x].setColor(game.active_piece_at(x, y));
                fgBlocks[y][x].setPos(game.active_piece_x() + x, game.active_piece_y() + y);
            }

            fgBlocks[y][x].animate();
        }
    }

    renderer.render(scene, camera);
};

requestAnimationFrame(animate);