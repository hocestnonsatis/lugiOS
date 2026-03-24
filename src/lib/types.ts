export interface RegistryEntry {
  id: string;
  displayName: string;
  author: string;
  repo: string;
  description: string;
  tags: string[];
  verified: boolean;
  publishedAt: string;
  /** HTTPS URL to a square listing icon; if omitted, UI uses GitHub (owner avatar / fallback). */
  logoUrl?: string | null;
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

/** From `GET /repos/{owner}/{repo}` (GitHub API). */
export interface GitHubRepoStats {
  fullName: string;
  ownerLogin: string;
  ownerAvatarUrl: string;
  description: string | null;
  htmlUrl: string;
  stars: number;
  forks: number;
  openIssues: number;
  watchers: number;
  defaultBranch: string;
  pushedAt: string | null;
  homepage: string | null;
  language: string | null;
  topics: string[];
  licenseName: string | null;
}
