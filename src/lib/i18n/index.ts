import en from "./en.json";

const strings: Record<string, string> = en;

export function t(key: string, vars?: Record<string, string | number>): string {
  let s = strings[key] ?? key;
  if (vars) {
    for (const [k, v] of Object.entries(vars)) {
      s = s.replace(`{${k}}`, String(v));
    }
  }
  return s;
}
