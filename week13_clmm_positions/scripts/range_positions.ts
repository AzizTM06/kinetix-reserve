function calcIl(k: number): number {
  return (2 * Math.sqrt(k)) / (1 + k) - 1;
}

function main() {
  for (const k of [1.1, 1.25, 1.5, 2.0]) {
    console.log(`k=${k.toFixed(2)} -> IL=${(calcIl(k) * 100).toFixed(3)}%`);
  }
}

main();
