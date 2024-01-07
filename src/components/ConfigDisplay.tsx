import ConfigParam from "./ConfigParam";
import { ServerConfig } from "../services/serverConfigAPI";
import { RepoConfig } from "../services/repoConfigAPI";

import styles from "../style/ConfigDisplay.module.css";

interface Props {
  server: ServerConfig;
  repo: RepoConfig;
}

export default function ConfigDisplay({ server, repo }: Props) {
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
      <fieldset>
        <legend>
          <b>Repository</b>
        </legend>
        <div className={styles.configBox}>
          <ConfigParam label="User name:" value={repo.username} />
          <ConfigParam label="Repo URL:" value={repo.url} />
        </div>
      </fieldset>
    </div>
  );
}
