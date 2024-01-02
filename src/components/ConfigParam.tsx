interface Props {
    label: string;
    value?: string;
}

export default function ConfigParam({ label, value }: Props) {
    return (
        <>
            <label>{label}</label>
            <span>{value}</span>
        </>
    );
}