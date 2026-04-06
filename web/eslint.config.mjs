import js from "@eslint/js";
import tseslint from "typescript-eslint";
import nextPlugin from "@next/eslint-plugin-next";
import reactHooksPlugin from "eslint-plugin-react-hooks";

export default tseslint.config(
  // 1. Setup Base JS and TS recommended rules
  js.configs.recommended,
  ...tseslint.configs.recommended,

  // 2. Ignore build folders
  {
    ignores: [".next/**", "out/**", "build/**", "next-env.d.ts"],
  },

  // 3. Manually apply Next.js and React Hooks rules
  {
    plugins: {
      "@next/next": nextPlugin,
      "react-hooks": reactHooksPlugin,
    },
    rules: {
      ...nextPlugin.configs.recommended.rules,
      ...nextPlugin.configs["core-web-vitals"].rules,
      ...reactHooksPlugin.configs.recommended.rules,
      
      // Your "Clippy" custom rules
      "@typescript-eslint/no-unused-vars": "warn",
      "@typescript-eslint/no-explicit-any": "warn",
      "react-hooks/exhaustive-deps": "warn",
    },
  },

  // 4. Ensure we handle the specific Next.js settings
  {
    settings: {
      next: {
        rootDir: "web/",
      },
    },
  }
);