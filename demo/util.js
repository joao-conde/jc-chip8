export const FPS = 144; // FPS
export const CLOCK = 800; // Hz
export const KEY_MAPPER = {
    "1": 0x01,
    "2": 0x02,
    "3": 0x03,
    "4": 0x0C,
    "q": 0x04,
    "w": 0x05,
    "e": 0x06,
    "r": 0x0D,
    "a": 0x07,
    "s": 0x08,
    "d": 0x09,
    "f": 0x0E,
    "z": 0x0A,
    "x": 0x00,
    "c": 0x0B,
    "v": 0x0F
};

export async function getROM(rom) {
    const response = await window.fetch(`roms/${rom}`);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}
