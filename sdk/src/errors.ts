export class PermissionDeniedError extends Error {
  constructor(permission: string) {
    super(`Permission denied: ${permission}`);
    this.name = "PermissionDeniedError";
  }
}

export class LugiOSError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "LugiOSError";
  }
}
