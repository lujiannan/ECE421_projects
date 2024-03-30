// import * as wasm from "hello-wasm-pack";
// wasm.greet();

/* 
grabs rust src code (in lib.rs) (which is wasm binded), 
then uses in here as javascript code, 
which is then pulled into the html file (index.html).

*/

import * as wasm from "connect4";

const textbox1 = document.getElementById("PrimeNumber");
document.getElementById("CheckNumber").addEventListener("click", event => {
    const answer = wasm.check_prime(textbox1.value);
    drawAnswer(answer);
});

const canvas = document.getElementById("board");
const ctx = canvas.getContext("2d");
function drawAnswer(yn) {
    ctx.beginPath();
    let xpos = 50;
    let ypos = 50;
    ctx.arc(xpos, ypos, 25, 0, 2 * Math.PI);
    if (yn == 0) {
        ctx.fillStyle = 'red';
    } else {
        ctx.fillStyle = 'green';
    }
    ctx.fill();
    ctx.font = '24pt Calibri';
    ctx.fillStyle = 'white';
    ctx.textAlign = 'center';
    if (yn == 0) {
        ctx.fillText('❌', xpos, ypos); // Symbol on red.
    } else {
        ctx.fillText('✅', xpos, ypos); // Symbol on green.
    }
}