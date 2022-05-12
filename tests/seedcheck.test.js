const debug = require("debug")("grin-tools-web:seedcheck.test.js");
const { EXPECTATION_FAILED } = require("http-status-codes");
const HttpStatus = require("http-status-codes");
var td = require("./testdata.js");

jest.setTimeout(2000);

describe("device/", () => {
  beforeAll(async () => {
    return td.cleanupDb();
  });
  afterAll(async () => {
    return td.cleanupDb();
  });

  test("/seedcheck - Return seed check JSON", async () => {
    const result = await td.get_healthcheck_data(0);
    console.log(result);
    //expect(result).toBe("peanut butter");
  });
});
