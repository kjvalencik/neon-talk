'use strict';

const bodyParser = require('body-parser');
const express = require('express');

const countPrimes = require('./sieve');
const { countPrimesMiddleware } = require('../native');

const {
    NODE_PORT = 3000
} = process.env;

const app = express();

const jsonBodyParser = bodyParser.json();
const bufferBodyParser = bodyParser.raw({
    type: 'application/json'
});

app.get('/healthz', (req, res) => res.send('OK\n'));

app.post('/api/sync', jsonBodyParser, ({ body: { max } = {} }, res, next) => {
    if (typeof max !== 'number') {
        return next(new Error('Must provide a numeric `max` field'));
    }

    const count = countPrimes(max);

    res.send({ count });
});

app.post('/api/async', bufferBodyParser, ({ body }, res, next) => {
    countPrimesMiddleware(body, (err, buf) => {
        if (err) {
            return next(err);
        }

        res.type('application/json').send(buf);
    });
});

app.use((err, req, res, next) => {
    console.error(err);
    next(err);
})

app.use(({ name, message }, _req, res, _next) => res.status(500).json({
    name,
    message,
}));

const server = app.listen(NODE_PORT, () => {
    const { port } = server.address();

    console.log(`Listening on: ${port}`);
});
