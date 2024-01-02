import ConfigParam from "./ConfigParam";
import { ServerConfig } from "../services/serverConfig";

import styles from "../style/ConfigDisplay.module.css";

interface Props {
  server: ServerConfig;
}

export default function ConfigDisplay({ server }: Props) {
  return (
    <div className={styles.configDisplay}>
      <fieldset>
        <legend>
          <b>Server</b>
        </legend>
        <div className={styles.configBox}>
          <ConfigParam label="Maximum RAM memory:" value={server.mem_max} />
          <ConfigParam label="Memory at startup:" value={server.mem_init} />
          <ConfigParam
            label="Enable server GUI:"
            value={server.gui ? "Yes" : "No"}
          />
        </div>
      </fieldset>
    </div>
  );
}
