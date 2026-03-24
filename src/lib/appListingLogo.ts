import type { GitHubRepoStats, RegistryEntry } from "$lib/types";

/** Parse `https://github.com/owner/repo` (optional trailing slash, fragment). */
export function parseGithubRepoUrl(repoUrl: string): { owner: string; repo: string } | null {
  try {
    const u = new URL(repoUrl.trim());
    const host = u.hostname.replace(/^www\./, "");
    if (host !== "github.com") return null;
    const segments = u.pathname.split("/").filter(Boolean);
    if (segments.length < 2) return null;
    const owner = decodeURIComponent(segments[0]);
    const repo = decodeURIComponent(segments[1].replace(/\.git$/i, ""));
    if (!owner || !repo) return null;
    return { owner, repo };
  } catch {
    return null;
  }
}

/** GitHub redirects this URL to the owner (user or org) avatar. */
export function githubOwnerAvatarFromRepo(repoUrl: string, size = 64): string | null {
  const p = parseGithubRepoUrl(repoUrl);
  if (!p) return null;
  return `https://github.com/${p.owner}.png?size=${size}`;
}

/**
 * Image URL for marketplace listing: explicit registry logo, then GitHub-based fallback.
 * Prefer `stats.ownerAvatarUrl` when GitHub API data is already loaded (detail page).
 */
export function appListingImageUrl(
  entry: RegistryEntry,
  stats?: GitHubRepoStats | null,
  size = 64,
): string | null {
  const custom = entry.logoUrl?.trim();
  if (custom) return custom;
  if (stats?.ownerAvatarUrl) return stats.ownerAvatarUrl;
  return githubOwnerAvatarFromRepo(entry.repo, size);
}
