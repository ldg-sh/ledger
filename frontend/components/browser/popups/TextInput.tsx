import { ReactNode, useState } from "react";
import styles from "./TextInput.module.scss";

interface TextInputProps {
  title: string;
  onChange: (newValue: string) => void;
  onSubmit: () => void;
  placeholder?: string;
  disabled?: boolean;
  select?: boolean;
  hint?: string | ReactNode;
}

export default function TextInput({
  title,
  onChange,
  onSubmit,
  placeholder,
  disabled,
  select = false,
  hint,
}: TextInputProps) {
  let [value, setValue] = useState("");

  let isReactNodeHint = typeof hint !== "string";

  return (
    <div className={styles.textInputContainer}>
      <p className={styles.title}>{title}</p>
      <input
        autoFocus={select}
        type="text"
        name="text"
        className={styles.textInput}
        onChange={(e) => {
          setValue(e.target.value);
          onChange(e.target.value);
        }}
        onKeyDown={(e) => {
          if (e.key === "Enter" && !disabled) {
            onSubmit();
          }
        }}
        value={value}
        placeholder={placeholder}
        disabled={disabled}
      />
      {isReactNodeHint && hint && <>{hint}</>}
      {!isReactNodeHint && hint && <p className={styles.hint}>{hint}</p>}
    </div>
  );
}
