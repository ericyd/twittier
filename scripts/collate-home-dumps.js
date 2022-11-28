const fs = require("fs");
const path = require("path");

function main() {
  const files = fs
    .readdirSync(process.env.HOME)
    .filter((filename) => filename.match(/^home.*\.json$/))
    .sort();
  const results = files.reduce((all, file) => {
    const data = fs.readFileSync(path.join(process.env.HOME, file)).toString();
    const json = JSON.parse(data);
    return [...all, ...json.data];
  }, []);

  fs.writeFileSync(
    path.join(process.env.HOME, "home-collated.json"),
    JSON.stringify({ data: results }, null, 2)
  );
  console.log("done");
}

main();
