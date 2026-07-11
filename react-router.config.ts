import type { Config } from "@react-router/dev/config";

export default {
  // Disables server-side engine generation, targeting native client bundle execution
  ssr: false,
  appDirectory: "src",
} satisfies Config;
