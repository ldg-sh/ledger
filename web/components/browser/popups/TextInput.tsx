import { ReactNode, useEffect, useRef, useState } from "react";
import styles from "./TextInput.module.scss";
import { cn } from "@/lib/util/class";

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
  errorHint?: boolean;
  required?: boolean;
}

export default function TextInput({
  onChange,
  onSubmit,
  placeholder,
  originalValue,
  disabled,
  select = false,
  errorHint = false,
  hint,
  title,
  formType = "text",
  required = false,
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
      <div className={styles.topText}>
        {title ? (
          <h1 className={styles.title}>
            {title || placeholder}
            {required && <span className={styles.required}>*</span>}
          </h1>
        ) : null}
        {!isReactNodeHint && hint && (
          <p className={cn(styles.hint, errorHint ? styles.error : undefined)}>
            {hint}
          </p>
        )}
      </div>

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
    </div>
  );
}
