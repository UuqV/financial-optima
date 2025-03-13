function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}

function simulate(n) {
  let hist = new Map();
  for (i = 0; i < 100; i++) {
    let set = new Set();
    let c = 0;
    while (set.size < n) {
      set.add(getRandomInt(n));
      c++;
    }
    const histVal = hist.get(c);
    if (histVal === undefined) {
      hist.set(c, 0);
    } else {
      hist.set(c, histVal + 1);
    }
  }
  console.log(hist);
}

simulate(3);
