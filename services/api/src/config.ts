export interface ApiConfig {
  port: number;
  logLevel: string;
  corsEnabled: boolean;
  apiKey?: string;
  unit09ProgramId: string;
  solanaRpcUrl: string;
  coreEngineRootDir: string;
}

export function loadApiConfig(): ApiConfig {
  return {
    port: parseInt(process.env.UNIT09_API_PORT || "8080", 10),
    logLevel: process.env.UNIT09_API_LOG_LEVEL || "info",
    corsEnabled: process.env.UNIT09_API_CORS === "false" ? false : true,
    apiKey: process.env.UNIT09_API_KEY,
    unit09ProgramId: process.env.UNIT09_PROGRAM_ID || "UNIT09_PROGRAM_PUBKEY_PLACEHOLDER",
    solanaRpcUrl: process.env.SOLANA_RPC_URL || "http://127.0.0.1:8899",
    coreEngineRootDir: process.env.UNIT09_ENGINE_ROOT_DIR || process.cwd()
  };
}
