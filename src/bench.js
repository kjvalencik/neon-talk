'use strict';

const assert = require('assert');
const util = require('util');

const { Suite } = require('benchmark');

const {
    countPrimesSync,
    countPrimesAsync
} = require('../native');

const countPrimesJs = require('./sieve');

const countPrimes = util.promisify(countPrimesAsync);

const MAGNITUDE = 6;
const MAX_PRIME = 10 ** MAGNITUDE;
const ROUNDS = 4;

const NUM_PRIMES = [
    0,  // primes in 0..10^0
    4,  // primes in 0..10^1
    25, // etc
    168,
    1_229,
    9_592,
    78_498,
    664_579,
    5_761_455,
    50_847_534,
    455_052_511,
    4_118_054_813,
    37_607_912_018,
    346_065_536_839,
    3_204_941_750_802,
    29_844_570_422_669,
    279_238_341_033_925,
    2_623_557_157_654_233,
    24_739_954_287_740_860,
    234_057_667_276_344_607,
    2_220_819_602_560_918_840,
];

function verify(count) {
    assert.equal(count + 1, NUM_PRIMES[MAGNITUDE]);
}

const suite = (new Suite())
    .add('Javascript', () => {
        for (let i = 0; i < ROUNDS; i += 1) {
            verify(countPrimesJs(MAX_PRIME));
        }
    })
    .add('Rust Sync', () => {
        for (let i = 0; i < ROUNDS; i += 1) {
            verify(countPrimesSync(MAX_PRIME));
        }
    })
    .add('Rust Async', {
        defer: true,
        fn(deferred) {
            Promise
                .all(
                    [...new Array(ROUNDS)].map(() => {
                        return countPrimes(MAX_PRIME).then(verify);
                    })
                )
                .then(() => deferred.resolve());
        }
    })
    .on('cycle', event => console.log(String(event.target)))
    .on('complete', () => {
        console.log(`Fastest is ${suite.filter('fastest').map('name')}`);
    })
    .run({ 'async': true });
