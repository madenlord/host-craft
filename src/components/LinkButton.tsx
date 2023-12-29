import { NavLink, Link } from "react-router-dom";

import styles from "../style/LinkButton.module.css";

interface Props {
  to: string;
  text: string;
}

export default function LinkButton({ to, text }: Props) {
  return (
    <NavLink
      to={to}
      className={({ isActive }) => (isActive ? styles.active : "")}
    >
      <Link to={to}>
        <button type="button">{text}</button>
      </Link>
    </NavLink>
  );
}
