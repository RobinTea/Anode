export type EditorMode = "write" | "command";
export type ViewTab = "plan" | "write" | "read";

export interface StatusState {
  editorMode: EditorMode;
  saveState: "saved" | "saving" | "idle";
  savedAt: Date | null;
  pageWords: number;
  bookWords: number;
  sessionWords: number;
  clock: string;
}

export const appStatus = $state<StatusState>({
  editorMode: "write",
  saveState: "idle",
  savedAt: null,
  pageWords: 0,
  bookWords: 0,
  sessionWords: 0,
  clock: formatClock(),
});

let sessionBaseline = 0;

export function setWordCounts(page: number, book: number) {
  appStatus.pageWords = page;
  appStatus.bookWords = book;
  if (sessionBaseline === 0) sessionBaseline = book;
  appStatus.sessionWords = Math.max(0, book - sessionBaseline);
}

export function setSaveState(state: StatusState["saveState"], at?: Date) {
  appStatus.saveState = state;
  if (at) appStatus.savedAt = at;
}

export function tickClock() {
  appStatus.clock = formatClock();
}

function formatClock(): string {
  return new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hour12: false });
}
