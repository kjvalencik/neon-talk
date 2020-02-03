'use strict';

function clearStride(sieve, from, stride) {
   for (let i = from; i < sieve.length; i += stride) {
       sieve[i] = false;
   }
}

function sieve(max) {
    const sieve = (new Array(max / 2)).fill(true);
    let i = 0;

	sieve[0] = false; // 1 is not prime

    while (++i) {
		if (sieve[i]) {
			const p = 2 * i + 1;
            const pp = p * p;

			if (pp >= max) {
				break;
            }

			clearStride(sieve, Math.floor(pp / 2), p);
		}
    }

    return sieve;
}

module.exports = function countPrimes(max) {
    return sieve(max).reduce((y, x) => y + x, 0);
};
