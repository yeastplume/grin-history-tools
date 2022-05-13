const debug = require("debug")("grin-tools-web:group.test.js");
const request = require('supertest');
const HttpStatus = require("http-status-codes");

const apiPort = process.env.PORT || 3000;
const url = `http://127.0.0.1:${apiPort}`;

async function cleanupDb() {
  // No manual cleanup for now
  //return global.knex.raw('DELETE FROM devices');
}

let agents = [
  request(url),
  request(url),
  request(url),
  request(url),
  request(url),
];

let get_healthcheck_data = async function (agentId, address) {
  return new Promise((resolve) => {
    agents[agentId]
      .get("/seedcheck/latest")
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug("/block body=%o", body);
        //expect(body.email).toBe(emails[agentId]);
        resolve(body);
      });
  });
};

module.exports = {
  cleanupDb,
  url,
  agents,
  get_healthcheck_data,
};
