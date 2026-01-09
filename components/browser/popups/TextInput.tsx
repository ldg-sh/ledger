import styles from "./TextInput.module.scss";

interface TextInputProps {
  title: string;
  onChange: (newValue: string) => void;
  placeholder?: string;
  disabled?: boolean;
}

export default function TextInput({
  title,
  onChange,
  placeholder,
  disabled,
}: TextInputProps) {
  return (
    <div className={styles.textInputContainer}>
      <p className={styles.title}>{title}</p>
      <input
        type="text"
        name="text"
        className={styles.textInput}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
      />
    </div>
  );
}
