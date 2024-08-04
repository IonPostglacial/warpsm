const canvas = document.getElementById("canvas");
const cx = canvas.getContext("2d");
let stepPeriod = 300;
const tileset = new Image();

export function canvas_set_fill_style(hexColor) {
    var color = "#" + hexColor.toString(16).padStart(6, "0");
    cx.fillStyle = color;
}

export function canvas_fill_rect(x, y, width, height) {
    cx.fillRect(x, y, width, height);
}

export function canvas_fill() {
    cx.fill();
}

export function canvas_draw_image(sx, sy, sw, sh, dx, dy, dw, dh) {
    cx.drawImage(tileset, sx, sy, sw, sh, dx, dy, dw, dh);
}

export function game_over() {
    window.alert("Game Over !");
    window.location.reload();
}

window.addEventListener("TrunkApplicationStarted", async () => {
    let tilesetBlob = await fetch("simple-tileset.png");
    tilesetBlob = await tilesetBlob.blob();
    tileset.src = URL.createObjectURL(tilesetBlob);
    const keys = { ArrowUp: 0, ArrowDown: 1, ArrowLeft: 2, ArrowRight: 3 };
    let game = new window.wasmBindings.GameState();

    window.onkeydown = e => {
        e.stopPropagation();
        game.on_key_down(keys[e.code])
    };
    let lastUpdateTimestamp = -1;
    function step(timestamp) {
        if (lastUpdateTimestamp < 0) lastUpdateTimestamp = timestamp;
        const progress = timestamp - lastUpdateTimestamp;
        if (progress >= stepPeriod) {
            lastUpdateTimestamp = timestamp;
            game.step(progress);
        }
        window.requestAnimationFrame(step);
    }
    window.requestAnimationFrame(step);
});
