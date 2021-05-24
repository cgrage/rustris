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
var material = [
    new THREE.MeshBasicMaterial({ color: getRandomColor() }),
    new THREE.MeshBasicMaterial({ color: getRandomColor() }),
    new THREE.MeshBasicMaterial({ color: getRandomColor() }),
    new THREE.MeshBasicMaterial({ color: getRandomColor() }),
    new THREE.MeshBasicMaterial({ color: getRandomColor() })
];

function rndMat() {
    return material[Math.floor(Math.random() * material.length)];
}

function getRandomColor() {
    var letters = '0123456789ABCDEF';
    var color = '#';
    for (var i = 0; i < 6; i++) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}

var blocks = null;

const animate = function () {
    requestAnimationFrame(animate);
    let redraw = false;
    if (game) {
        redraw = game.run_step();
    }

    if (redraw) {
        if (blocks) {
            for (let y = 0; y < blocks.length; y++) {
                for (let x = 0; x < blocks[y].length; x++) {
                    scene.remove(blocks[y][x]);
                }
            }
        } else {
            blocks = Array(20);
            for (let y = 0; y < blocks.length; y++) {
                blocks[y] = Array(10);
            }
        }

        for (let y = 0; y < blocks.length; y++) {
            for (let x = 0; x < blocks[y].length; x++) {
                blocks[y][x] = new THREE.Mesh(geometry, rndMat());
                blocks[y][x].position.set(x, y, 0);
                scene.add(blocks[y][x]);
            }
        }
    }

    // cube.rotation.x += 0.01;
    // cube.rotation.y += 0.01;

    renderer.render(scene, camera);
};

requestAnimationFrame(animate);