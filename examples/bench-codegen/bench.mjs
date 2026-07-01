import { createRequire } from 'node:module'
import { performance } from 'node:perf_hooks'

const require = createRequire(import.meta.url)
const native = require('./bench_codegen.node')

const ITERATIONS = Number(process.env.ITERATIONS || 100_000)
const SAMPLES = Number(process.env.SAMPLES || 25)

const small = { a: 'alpha', b: 42 }
const medium = {
  name: 'medium',
  count: 7,
  enabled: true,
  tags: ['a', 'b', 'c'],
  threshold: 0.75,
  mode: 'fast',
}
const large = {
  f01: 'a', f02: 'b', f03: 'c', f04: 'd', f05: 'e',
  n01: 1, n02: 2, n03: 3, n04: 4, n05: 5,
  b01: true, b02: false, b03: true,
  o01: 'x', o02: 'y', o03: 9, o04: true, o05: ['r', 's'],
  p01: 1.5, p02: 2.5,
}
const union = {
  names: ['one', 'two'],
  payload: Buffer.from('payload'),
  maybeFlag: 9,
}
const structured = {
  type: 'Gamma',
  value: 'payload',
  extra: 'tail',
}

const cases = [
  ['small inline from js', () => native.consumeSmallInline(small)],
  ['small compact from js', () => native.consumeSmallCompact(small)],
  ['small auto from js', () => native.consumeSmallAuto(small)],
  ['small inline to js', () => native.createSmallInline()],
  ['small compact to js', () => native.createSmallCompact()],
  ['small auto to js', () => native.createSmallAuto()],
  ['medium inline from js', () => native.consumeMediumInline(medium)],
  ['medium compact from js', () => native.consumeMediumCompact(medium)],
  ['medium auto from js', () => native.consumeMediumAuto(medium)],
  ['large inline from js', () => native.consumeLargeInline(large)],
  ['large compact from js', () => native.consumeLargeCompact(large)],
  ['large auto from js', () => native.consumeLargeAuto(large)],
  ['union inline from js', () => native.consumeUnionInline(union)],
  ['union compact from js', () => native.consumeUnionCompact(union)],
  ['union auto from js', () => native.consumeUnionAuto(union)],
  ['structured inline from js', () => native.consumeStructuredInline(structured)],
  ['structured compact from js', () => native.consumeStructuredCompact(structured)],
  ['structured auto from js', () => native.consumeStructuredAuto(structured)],
]

function percentile(values, percentileValue) {
  const index = Math.min(
    values.length - 1,
    Math.floor((values.length - 1) * percentileValue),
  )
  return values[index]
}

function runCase(name, fn) {
  for (let i = 0; i < 1_000; i++) fn()

  const samples = []
  for (let sample = 0; sample < SAMPLES; sample++) {
    const start = performance.now()
    for (let i = 0; i < ITERATIONS; i++) fn()
    const elapsed = performance.now() - start
    samples.push(elapsed / ITERATIONS)
  }
  samples.sort((a, b) => a - b)
  const mean = samples.reduce((total, value) => total + value, 0) / samples.length
  const ops = 1_000 / mean
  console.log(
    `${name.padEnd(28)} ${ops.toFixed(0).padStart(10)} ops/sec  p75 ${(percentile(samples, 0.75) * 1_000).toFixed(3)} us  p99 ${(percentile(samples, 0.99) * 1_000).toFixed(3)} us`,
  )
}

console.log(`iterations=${ITERATIONS} samples=${SAMPLES} node=${process.version}`)
for (const [name, fn] of cases) {
  runCase(name, fn)
}
