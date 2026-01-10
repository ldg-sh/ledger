import { useRef, useState } from "react";
import styles from "./TextInput.module.scss";
import { usePathname } from "next/navigation";
import { extractPathFromUrl } from "@/lib/util/url";

interface TextInputProps {
  title: string;
  onChange: (newValue: string) => void;
  placeholder?: string;
  disabled?: boolean;
  select?: boolean;
}

export default function TextInput({
  title,
  onChange,
  placeholder,
  disabled,
  select = false,
}: TextInputProps) {
  let pathname = usePathname();
  let [value, setValue] = useState("");

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
        placeholder={placeholder}
        disabled={disabled}
      />
      {value ? (
        <p className={styles.hint}>
          Your folder will be created{" "}
            {"at "}
            <strong> {"home" +
              (extractPathFromUrl(pathname) == ""
                ? "/"
                : "" + extractPathFromUrl(pathname)) +
              value}
              </strong>
        </p>
      ) : (
        <p className={styles.hint}>
          Your folder will be created relative to the current path.
        </p>
      )}
    </div>
  );
}
