import { ReactNode, useEffect, useRef, useState, useId } from "react";
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
  const [value, setValue] = useState(originalValue || "");
  const inputRef = useRef<HTMLInputElement>(null);
  
  const generatedId = useId(); 
  const hintId = `${generatedId}-hint`;

  useEffect(() => {
    if (select && inputRef.current) {
      inputRef.current.select();
    }
  }, [select]);

  const isReactNodeHint = typeof hint !== "string";

  return (
    <div className={styles.textInputContainer}>
      <div className={styles.topText}>
        {title ? (
          <label htmlFor={generatedId} className={styles.title}>
            {title || placeholder}
            {required && <span className={styles.required} aria-hidden="true">*</span>}
          </label>
        ) : null}
        {!isReactNodeHint && hint && (
          <p 
            id={hintId}
            className={cn(styles.hint, errorHint ? styles.error : undefined)}
            role={errorHint ? "alert" : undefined}
          >
            {hint}
          </p>
        )}
      </div>

      <input
        id={generatedId}
        ref={inputRef}
        autoFocus={select}
        type={formType}
        name={title?.toLowerCase().replace(/\s/g, "-") || "input"} 
        className={styles.textInput}
        aria-required={required}
        aria-invalid={errorHint}
        aria-describedby={hint ? hintId : undefined}
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
      {isReactNodeHint && <div id={hintId}>{hint}</div>}
    </div>
  );
}