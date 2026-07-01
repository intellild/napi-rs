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

const caseSpecs = [
  ['small baseline from js', 'consumeSmallBaseline', [small]],
  ['small inline from js', 'consumeSmallInline', [small]],
  ['small compact from js', 'consumeSmallCompact', [small]],
  ['small auto from js', 'consumeSmallAuto', [small]],
  ['small baseline to js', 'createSmallBaseline', []],
  ['small inline to js', 'createSmallInline', []],
  ['small compact to js', 'createSmallCompact', []],
  ['small auto to js', 'createSmallAuto', []],
  ['medium baseline from js', 'consumeMediumBaseline', [medium]],
  ['medium inline from js', 'consumeMediumInline', [medium]],
  ['medium compact from js', 'consumeMediumCompact', [medium]],
  ['medium auto from js', 'consumeMediumAuto', [medium]],
  ['large baseline from js', 'consumeLargeBaseline', [large]],
  ['large inline from js', 'consumeLargeInline', [large]],
  ['large compact from js', 'consumeLargeCompact', [large]],
  ['large auto from js', 'consumeLargeAuto', [large]],
  ['union baseline from js', 'consumeUnionBaseline', [union]],
  ['union inline from js', 'consumeUnionInline', [union]],
  ['union compact from js', 'consumeUnionCompact', [union]],
  ['union auto from js', 'consumeUnionAuto', [union]],
  ['structured baseline from js', 'consumeStructuredBaseline', [structured]],
  ['structured inline from js', 'consumeStructuredInline', [structured]],
  ['structured compact from js', 'consumeStructuredCompact', [structured]],
  ['structured auto from js', 'consumeStructuredAuto', [structured]],
]

const cases = caseSpecs
  .filter(([, exportName]) => typeof native[exportName] === 'function')
  .map(([name, exportName, args]) => [name, () => native[exportName](...args)])

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
