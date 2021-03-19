const PIXEL_SET_COLOR = 0xFFFFFFFFFF; // white
const PIXEL_UNSET_COLOR = 0x000000FF; // black

const canvas = document.createElement("canvas");
canvas.width = 64;
canvas.height = 32;
const ctx = canvas.getContext("2d");

const scaledCanvas = document.querySelector("canvas#scaled");
const scaledCtx = scaledCanvas.getContext("2d");
scaledCtx.scale(scaledCanvas.width / canvas.width, scaledCanvas.height / canvas.height);
scaledCtx.imageSmoothingEnabled = false;

const image = ctx.createImageData(canvas.width, canvas.height);
const videoBuff = new DataView(image.data.buffer);

export function updateCanvas(pixels) {
    for (let i = 0; i < pixels.length; i++)
        videoBuff.setUint32(i * 4, pixels[i] ? PIXEL_SET_COLOR : PIXEL_UNSET_COLOR);
    ctx.putImageData(image, 0, 0);
    scaledCtx.drawImage(canvas, 0, 0);
}
