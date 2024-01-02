import { Link } from "react-router-dom";

interface Props {
  to: string;
  text: string;
}

export default function LinkButton({ to, text }: Props) {
  return (
      <Link to={to}>
        <button type="button">{text}</button>
      </Link>
  );
}
