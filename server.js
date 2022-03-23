import { createServer } from "http";
import { readFile } from "fs";
import { lookup } from "mime-types";

const port = 8080;
const webFolder = "web";

createServer((request, response) => {
  const url = request.url === "/" ? "/index.html" : request.url;
  const filename = `${process.cwd()}/${webFolder}${url}`;
  // lookup can deal with files in folders etc
  const mimeType = lookup(filename);
  console.log(`Request for ${filename} with mime type ${mimeType}`);

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
}).listen(port);

console.log("Static file server running at http://localhost:" + port + "/\nCTRL + C to shutdown");