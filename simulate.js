function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}

function simulate(n) {
  let hist = new Map();
  for (i = 0; i < 1000; i++) {
    let set = new Set();
    let c = 1;
    while (set.size < n) {
      set.add(getRandomInt(n));
      c++;
    }
    const histVal = hist.get(c);
    if (histVal === undefined) {
      hist.set(c, 1);
    } else {
      hist.set(c, histVal + 1);
    }
  }
  console.log(new Map([...hist.entries()].sort((a, b) => a[0] - b[0])));
}

simulate(3);
