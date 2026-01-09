export interface ColorInfo {
  background: string;
  foreground: string;
  backgroundHover: string;
  backgroundActive: string;
}

export const successColor: ColorInfo = {
  background: "rgba(56, 161, 105, 0.1)",
  foreground: "rgb(34, 197, 94)",
  backgroundHover: "rgba(34, 197, 94, 0.12)",
  backgroundActive: "rgba(34, 197, 94, 0.18)",
};

export const errorColor: ColorInfo = {
  background: "rgba(239, 68, 68, 0.05)",
  foreground: "rgb(239, 68, 68)",
  backgroundHover: "rgba(239, 68, 68, 0.08)",
  backgroundActive: "rgba(239, 68, 68, 0.12)",
};

export const defaultColor: ColorInfo = {
  background: "#ffffff00",
  foreground: "var(--color-text-primary)",
  backgroundHover: "var(--color-background-active)",
  backgroundActive: "var(--color-background-selected)",
};
