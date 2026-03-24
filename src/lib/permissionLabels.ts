/** Short English description for install-time review. */
export function describePermission(token: string): string {
  const table: Record<string, string> = {
    storage: "Isolated key–value storage for this app only",
    notifications: "Show native desktop notifications",
    "clipboard:read": "Read text from the system clipboard",
    "clipboard:write": "Write text to the system clipboard",
    "audio:play": "Decode and play audio files from the app bundle",
    "filesystem:read": "Open a file picker and read files you select",
    "filesystem:write": "Save files via a save dialog to paths you choose",
    network: "Make outbound HTTP/HTTPS requests to any host",
  };
  if (token.startsWith("network:domain:")) {
    const host = token.slice("network:domain:".length);
    return `HTTP/HTTPS requests only to ${host}`;
  }
  return table[token] ?? token;
}
