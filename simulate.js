let set = new Set();

function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}
let i = 0;
while (set.size < 50) {
  set.add(getRandomInt(50));
  i++;
}
console.log(i);
