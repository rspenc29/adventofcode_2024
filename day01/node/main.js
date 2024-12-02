
const fs = require('node:fs')

const lines = fs.readFileSync('../input.txt', 'utf8').split("\n")
    .map(line => line.split(/\s+/))
    .filter(line => line.length > 0)

const col1 = lines.map(line => line[0]).sort().map(n => +n),
    col2 = lines.map(line => line[1]).sort().map(n => +n)

const part1 = () => {
    return col1.map((n, i) => Math.abs(n - col2[i]))
        .reduce((a, n) => a + n, 0)
}

const part2 = () => {
    const counts = col2.reduce((a, n) => (a[n] = (a[n] || 0) + 1, a), {})

    return col1.map(n => n * (counts[n] || 0))
        .reduce((a, n) => a + n, 0)

    // less code but requires multiple iterations over col2
    //return col1.map(n => n * col2.filter(x => x === n).length)
    //    .reduce((a, n) => a + n, 0)
}

console.log({
    part1: part1(), // 1873376
    part2: part2(), // 18997088
})


