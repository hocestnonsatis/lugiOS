export interface RegistryEntry {
  id: string;
  displayName: string;
  author: string;
  repo: string;
  description: string;
  tags: string[];
  verified: boolean;
  publishedAt: string;
}

export interface AppManifest {
  id: string;
  displayName: string;
  version: string;
  description: string;
  icon: string;
  entryPoint: string;
  permissions: string[];
  window: {
    width: number;
    height: number;
    resizable: boolean;
    alwaysOnTop: boolean;
  };
}

export interface GrantRecord {
  appId: string;
  granted: string[];
  grantedAt: string;
}
