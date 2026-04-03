import { ReactNode, useEffect, useRef, useState } from "react";
import styles from "./TextInput.module.scss";

interface TextInputProps {
  onChange: (newValue: string) => void;
  onSubmit: () => void;
  placeholder?: string;
  originalValue?: string;
  disabled?: boolean;
  select?: boolean;
  hint?: string | ReactNode;
  title?: string;
  formType?: string;
}

export default function TextInput({
  onChange,
  onSubmit,
  placeholder,
  originalValue,
  disabled,
  select = false,
  hint,
  title,
  formType = "text",
}: TextInputProps) {
  let [value, setValue] = useState(originalValue || "");

  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (select && inputRef.current) {
      inputRef.current.select();
    }
  }, [select]);

  let isReactNodeHint = typeof hint !== "string";

  return (
    <div className={styles.textInputContainer}>
      {title ? <h1 className={styles.title}>{title || placeholder}</h1> : null}
      <input
        ref={inputRef}
        autoFocus={select}
        type={formType}
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
