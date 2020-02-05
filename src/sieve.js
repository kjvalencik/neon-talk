'use strict';

function clearStride(sieve, from, stride) {
   for (let i = from; i < sieve.length; i += stride) {
       sieve[i] = 0;
   }
}

function sieve(max) {
    const sieve = (new Array((max / 2) | 0)).fill(1);
    let i = 0;

	sieve[0] = 0; // 1 is not prime

    while (++i) {
		if (sieve[i]) {
			const p = 2 * i + 1;
            const pp = p * p;

			if (pp >= max) {
				break;
            }

			clearStride(sieve, (pp / 2) | 0, p);
		}
    }

    return sieve;
}

module.exports = function countPrimes(max) {
    return sieve(max).reduce((y, x) => y + x, 0);
};
