const select = document.querySelector("select#roms");

const ROMS = [,
    "test/BC_test",
    "test/c8_test",
    "test/Keypad Test [Hap, 2006]",
    "test/test_opcode",
    "Blitz",
    "Breakout",
    "Brix",
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
    const rom = select.options[select.selectedIndex].value;
    const response = await window.fetch(`roms/${rom}.ch8`);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}

export function listROMs() {
    ROMS.forEach((rom) => {
        const option = document.createElement("option");
        option.value = rom;
        option.innerHTML = rom;
        select.appendChild(option);
    });
}
