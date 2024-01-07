import { Link } from "react-router-dom";

interface Props {
  to: string;
  text: string;
  onClick ?: Function;
}

export default function LinkButton({ to, text, onClick }: Props) {
  return (
      <Link to={to}>
        <button 
          type="button"
          onClick={ onClick 
            ? () => onClick() 
            : undefined
          }
        >
          {text}
        </button>
      </Link>
  );
}
