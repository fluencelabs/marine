const fs = require("fs");
const recast = require("recast");
const parser = require("@babel/parser");
const traverse = require("@babel/traverse").default;

const sourceFilePath = "../marine-js-pkg/marine_js.js";
const targetFilePath = "./src/marine_js.js";

const GET_IMPORTTS_FN_NAME = "__wbg_get_imports"

fs.readFile(sourceFilePath, "utf8", (err, sourceData) => {
  if (err) {
    console.error("Error reading source file:", err);
    process.exit(1);
  }

  const sourceAst = parser.parse(sourceData, { sourceType: "module" });
  let sourceFunction = null;

  traverse(sourceAst, {
    FunctionDeclaration(path) {
      if (path.node.id.name === GET_IMPORTTS_FN_NAME) {
        sourceFunction = path.node;
        path.stop();
      }
    },
  });

  if (!sourceFunction) {
    console.error(`Error: ${GET_IMPORTTS_FN_NAME} function not found in source file`);
    process.exit(1);
  }

  fs.readFile(targetFilePath, "utf8", (err, targetData) => {
    if (err) {
      console.error("Error reading target file:", err);
      process.exit(1);
    }

    const targetAst = recast.parse(targetData, {
      parser: {
        parse: (source) => parser.parse(source, { sourceType: "module" }),
      },
    });

    let targetFunctionPath = null;

    recast.visit(targetAst, {
      visitFunctionDeclaration(path) {
        if (path.node.id.name === GET_IMPORTTS_FN_NAME) {
          targetFunctionPath = path;
          return false;
        }
        this.traverse(path);
      },
    });

    if (!targetFunctionPath) {
      console.error(`Error: ${GET_IMPORTTS_FN_NAME} function not found in target file`);
      process.exit(1);
    }

    targetFunctionPath.replace(sourceFunction);
    const output = recast.print(targetAst).code;

    fs.writeFile(targetFilePath, output, "utf8", (err) => {
      if (err) {
        console.error("Error writing to target file:", err);
        process.exit(1);
      }

      console.log(`Function ${GET_IMPORTTS_FN_NAME} replaced successfully in target file.`);
    });
  });
});
