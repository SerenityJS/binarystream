export default {
  rules: {
    "prettier/prettier": [
      "warn",
      {
        tabWidth: 2,
        useTabs: false,
        semi: true,
        singleQuote: false,
        jsxSingleQuote: false,
        trailingComma: "none",
      },
      {
        usePrettierrc: false,
      },
    ],
    "promise/catch-or-return": "off",
    "@typescript-eslint/no-extraneous-class": "off",
    "@typescript-eslint/consistent-type-imports": "off",
    "@typescript-eslint/no-invalid-void-type": "off",
    "@typescript-eslint/prefer-literal-enum-member": "off",
    "@typescript-eslint/no-floating-promises": "off",
    "@typescript-eslint/no-dynamic-delete": "off",
  },
  languageOptions: {
    parserOptions: {
      projectService: {
        maximumDefaultProjectFileMatchCount_THIS_WILL_SLOW_DOWN_LINTING: 10000,
      },
    },
  },
}