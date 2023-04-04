const fs = require("fs");
const recast = require('recast');
const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;

const sourceFilePath = "../marine-js-pkg/marine_js.js";
const targetFilePath = "./src/marine_js.js";

fs.readFile(sourceFilePath, 'utf8', (err, sourceData) => {
  if (err) {
    console.error('Error reading source file:', err);
    return;
  }

  const sourceAst = parser.parse(sourceData, { sourceType: 'module' });
  let sourceFunction = null;

  traverse(sourceAst, {
    FunctionDeclaration(path) {
      if (path.node.id.name === 'getImports') {
        sourceFunction = path.node;
        path.stop();
      }
    },
  });

  if (!sourceFunction) {
    console.error('Error: getImports function not found in source file');
    return;
  }

  fs.readFile(targetFilePath, 'utf8', (err, targetData) => {
    if (err) {
      console.error('Error reading target file:', err);
      return;
    }

    const targetAst = recast.parse(targetData, {
      parser: {
        parse: (source) => parser.parse(source, { sourceType: 'module' }),
      },
    });

    let targetFunctionPath = null;

    recast.visit(targetAst, {
      visitFunctionDeclaration(path) {
        if (path.node.id.name === 'getImports') {
          targetFunctionPath = path;
          return false;
        }
        this.traverse(path);
      },
    });

    if (!targetFunctionPath) {
      console.error('Error: getImports function not found in target file');
      return;
    }

    targetFunctionPath.replace(sourceFunction);
    const output = recast.print(targetAst).code;

    fs.writeFile(targetFilePath, output, 'utf8', (err) => {
      if (err) {
        console.error('Error writing to target file:', err);
        return;
      }

      console.log('Function getImports replaced successfully in target file.');
    });
  });
});
