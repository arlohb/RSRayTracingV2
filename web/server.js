import { createServer as createServerHTTP } from "http";
import { createServer as createServerHTTPS } from "https";
import { readFile, readFileSync } from "fs";
import { lookup } from "mime-types";

const port = 8080;
const webFolder = "dist";

const app = (request, response) => {
  const url = request.url === "/" ? "/index.html" : request.url;
  const filename = `${process.cwd()}/${webFolder}${url}`;
  // lookup can deal with files in folders etc
  const mimeType = lookup(filename);
  console.log(`Request for ${filename} with mime type ${mimeType}`);

  // should promisify this
  readFile(filename, null, (err, data) => {
    if (err) {
      response.writeHead(404, {"Content-Type": "text/plain"});
      response.write("404 Not Found\n");
      response.end();
      return;
    }

    response.writeHead(200, {
      ...(mimeType !== false && {
        "Content-Type": mimeType
      }),
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    });
    response.write(data);
    response.end();
  })
};

createServerHTTP(app).listen(port);

try {
  // should promisify this
  const { key, cert } = JSON.parse(readFileSync("web/secrets.json"));

  createServerHTTPS({
    // should promisify this
    key: readFileSync(key),
    cert: readFileSync(cert),
  }, app).listen(port + 1);
} catch (e) {
  console.log(`HTTPS failed with error ${e}`);
}

console.log("Static file server running at http://localhost:" + port + "/\nCTRL + C to shutdown");