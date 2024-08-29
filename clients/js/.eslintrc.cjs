//@ts-check

/** @type {import("eslint").Linter.Config} */
module.exports = {
  overrides: [
    {
      files: ["src/generated/**/*"],
      rules: {
        "@typescript-eslint/explicit-module-boundary-types": "off",
        "@typescript-eslint/no-explicit-any": "off",
        "@typescript-eslint/no-empty-interface": "off",
        "@typescript-eslint/no-unnecessary-condition": "off",
        "@typescript-eslint/no-unsafe-return": "off",

        "import/no-cycle": "off",
      },
    },
  ],
};
