const fs = require("fs");
const recast = require("recast");
const parser = require("@babel/parser");
const traverse = require("@babel/traverse").default;

const sourceFilePath = "../marine-js-pkg/marine_js.js";
const targetFilePath = "./src/marine_js.js";

const GET_IMPORTTS_FN_NAME = "__wbg_get_imports"

const WBG_ADAPTER_REGEX = /__wbg_adapter_\d+/;

fs.readFile(sourceFilePath, "utf8", (err, sourceData) => {
  if (err) {
    console.error("Error reading source file:", err);
    process.exit(1);
  }

  const sourceAst = parser.parse(sourceData, { sourceType: "module" });
  let sourceFunction = null;
  let wbgAdapterFunc = null;
  let imports = []
  traverse(sourceAst, {
    FunctionDeclaration(path) {
      if (path.node.id.name === GET_IMPORTTS_FN_NAME) {
        sourceFunction = path.node;
      } else if (WBG_ADAPTER_REGEX.test(path.node.id.name)) {
        wbgAdapterFunc = path.node;
      }
    },
    ImportDeclaration(path) {
      imports.push(path.node)
    }
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
    let wbgAdapderPath = null;
    let importsPaths = []

    recast.visit(targetAst, {
      visitFunctionDeclaration(path) {
        if (path.node.id.name === GET_IMPORTTS_FN_NAME) {
          targetFunctionPath = path;
        } else if (WBG_ADAPTER_REGEX.test(path.node.id.name)) {
          wbgAdapderPath = path;
        }

        this.traverse(path);
      },
      visitImportDeclaration(path) {
        importsPaths.push(path)
        this.traverse(path);
      }
    });

    if (!targetFunctionPath) {
      console.error(`Error: ${GET_IMPORTTS_FN_NAME} function not found in target file`);
      process.exit(1);
    }

    if (importsPaths.length !== imports.length) {
      console.error(`Error: source and destination have different number of import statements. Please update imports in destination manually.`);
      process.exit(1);
    }

    for(let importIndex = 0; importIndex < importsPaths.length; importIndex++) {
      importsPaths[importIndex].replace(imports[importIndex])
    }

    targetFunctionPath.replace(sourceFunction);
    wbgAdapderPath.replace(wbgAdapterFunc);
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
