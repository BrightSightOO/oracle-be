//@ts-check

const devPatterns = [
  "**/.eslintrc.?([cm])js",
  "*.config.?([cm])[tj]s",
  "configs/**/*",
  "clients/js/scripts/**/*",
];

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: [
    "eslint:recommended",
    "plugin:import/recommended",
    "plugin:eslint-comments/recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:prettier/recommended",
    "prettier",
  ],
  env: {
    es2020: true,
    "shared-node-browser": true,
  },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    sourceType: "module",
    ecmaVersion: 2020,
  },
  overrides: [
    {
      files: ["clients/js/**/*.ts"],
      extends: [
        "plugin:@typescript-eslint/stylistic-type-checked",
        "plugin:@typescript-eslint/strict-type-checked",
        "plugin:import/typescript",
      ],
      parserOptions: {
        project: ["clients/js/tsconfig.json"],
      },
      rules: {
        "@typescript-eslint/array-type": ["error", { default: "generic" }],
        "@typescript-eslint/consistent-type-definitions": ["error", "type"],
        "@typescript-eslint/consistent-type-imports": [
          "error",
          { prefer: "type-imports", fixStyle: "separate-type-imports" },
        ],
        "@typescript-eslint/explicit-module-boundary-types": "error",

        "@typescript-eslint/no-empty-interface": ["error", { allowSingleExtends: true }],
        "@typescript-eslint/no-namespace": ["error", { allowDeclarations: true }],
        "@typescript-eslint/no-non-null-assertion": "off",

        "@typescript-eslint/prefer-literal-enum-member": [
          "error",
          { allowBitwiseExpressions: true },
        ],

        "@typescript-eslint/restrict-template-expressions": [
          "error",
          {
            allowAny: false,
            allowBoolean: true,
            allowNullish: false,
            allowNumber: true,
            allowRegExp: true,
          },
        ],

        // TS verifies these.
        "consistent-return": "off",
      },
    },
    {
      files: ["*.d.ts"],
      rules: {
        "@typescript-eslint/no-empty-interface": "off",
        "no-var": "off",
      },
    },
    {
      files: ["*.cjs"],
      rules: {
        "@typescript-eslint/no-require-imports": "off",
      },
    },
    {
      files: devPatterns,
      env: {
        node: true,
      },
      rules: {
        "import/no-anonymous-default-export": "off",
        "no-console": "off",
        "no-restricted-globals": "off",
      },
    },
  ],
  rules: {
    "eslint-comments/disable-enable-pair": ["error", { allowWholeFile: true }],
    "eslint-comments/no-unused-disable": "error",

    "import/consistent-type-specifier-style": ["error", "prefer-top-level"],
    "import/extensions": ["error", "never", { pattern: { json: "always", mjs: "always" } }],
    "import/first": "error",
    "import/newline-after-import": "error",
    "import/no-absolute-path": "error",
    "import/no-anonymous-default-export": [
      "error",
      {
        allowAnonymousClass: false,
        allowAnonymousFunction: false,
        allowArray: false,
        allowArrowFunction: false,
        allowCallExpression: false,
        allowLiteral: false,
        allowObject: true,
      },
    ],
    "import/no-cycle": "error",
    "import/no-duplicates": "error",
    "import/no-extraneous-dependencies": ["error", { devDependencies: devPatterns }],
    "import/no-mutable-exports": "error",
    "import/no-relative-packages": "error",
    "import/no-self-import": "error",
    "import/no-useless-path-segments": "error",
    "import/order": [
      "error",
      {
        alphabetize: {
          order: "asc",
        },
        groups: [
          "type",
          "builtin",
          "external",
          ["internal", "unknown"],
          "parent",
          "sibling",
          "index",
          "object",
        ],
        "newlines-between": "always",
      },
    ],

    "@typescript-eslint/no-unused-vars": ["error", { argsIgnorePattern: "^_" }],

    eqeqeq: ["error", "always", { null: "ignore" }],
    "no-caller": "error",
    "no-console": "warn",
    "no-eval": "error",
    "no-extend-native": "error",
    "no-extra-bind": "error",
    "no-floating-decimal": "error",
    "no-implied-eval": "error",
    "no-label-var": "error",
    "no-labels": ["error", { allowLoop: true, allowSwitch: true }],
    "no-multi-str": "error",
    "no-new": "warn",
    "no-new-func": "error",
    "no-new-wrappers": "error",
    "no-object-constructor": "error",
    "no-octal-escape": "error",
    "no-proto": "error",
    "no-return-await": "error",
    "no-self-compare": "warn",
    "no-sequences": "error",
    "no-throw-literal": "error",
    "no-useless-call": "error",
    "no-useless-computed-key": "error",
    "no-useless-concat": "error",
    "no-useless-constructor": "error",
    "object-shorthand": "error",
    "operator-assignment": ["error", "always"],
    "prefer-const": ["error", { destructuring: "all" }],
    "prefer-template": "error",
    radix: "error",
    "sort-imports": ["error", { ignoreDeclarationSort: true }],
  },
  settings: {
    "import/resolver": {
      typescript: true,
    },
  },
};
