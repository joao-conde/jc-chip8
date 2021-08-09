const ROMS = [
    "Blitz",
    "Brix",
    "Breakout",
    "Connect4",
    "Flightrunner",
    "Hidden",
    "Merlin",
    "Missile",
    "Outlaw",
    "Pong",
    "Space Invaders",
    "Tank",
    "Tetris",
    "TicTac",
    "UFO",
    "Vers",
];

export async function getROM() {
    const select = document.querySelector("select#roms");
    const rom = select.options[select.selectedIndex].value;
    const response = await window.fetch(`public/roms/${rom}.ch8`);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}

export function listROMs() {
    const select = document.querySelector("select#roms");
    ROMS.sort().forEach((rom) => {
        const option = document.createElement("option");
        option.value = rom;
        option.innerHTML = rom;
        select.appendChild(option);
    });
}
