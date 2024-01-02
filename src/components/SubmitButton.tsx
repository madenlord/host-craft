import styles from '../style/SubmitButton.module.css';

interface Props {
    text: string;
    disabled?: boolean;
}

export default function SubmitButton({ text, disabled }: Props) {
    return (
    <button
        type="submit"
        className={disabled ? styles.disabled : undefined}
        disabled={disabled}
    >
        {text}
    </button>
    );
}