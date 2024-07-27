onmessage = e => {
    let x = e.data.gay;

    let s = 0;
    let n = 10000000;
    for (let i = 0; i < n; i++) {
        s += e.data.gay;
    }

    postMessage([e.data.gay, s, x]);
};

// setInterval(() => {
// console.log("gaga");
// }, 500);
