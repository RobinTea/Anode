import { writable, derived } from "svelte/store";

export interface UndoEntry {
  pageId: string;
  before: Record<string, unknown>;
  after: Record<string, unknown>;
  timestamp: number;
  plainText: string;
}

class UndoStack {
  private stack: UndoEntry[] = [];
  private position: number = -1;
  private maxSize: number = 200;

  push(entry: UndoEntry) {
    // Remove all entries after current position (for new entries after undo)
    this.stack = this.stack.slice(0, this.position + 1);

    // Add new entry
    this.stack.push(entry);
    this.position = this.stack.length - 1;

    // Cap the stack size
    if (this.stack.length > this.maxSize) {
      this.stack.shift();
      this.position--;
    }
  }

  undo(): UndoEntry | null {
    if (this.position <= 0) return null;
    this.position--;
    return this.stack[this.position];
  }

  redo(): UndoEntry | null {
    if (this.position >= this.stack.length - 1) return null;
    this.position++;
    return this.stack[this.position];
  }

  clear() {
    this.stack = [];
    this.position = -1;
  }

  canUndo(): boolean {
    return this.position > 0;
  }

  canRedo(): boolean {
    return this.position < this.stack.length - 1;
  }

  getStack(): UndoEntry[] {
    return [...this.stack];
  }

  getPosition(): number {
    return this.position;
  }
}

const createUndoStore = () => {
  const stack = new UndoStack();
  const { subscribe, update } = writable(stack);

  return {
    subscribe,
    push: (entry: UndoEntry) => {
      update((s) => {
        s.push(entry);
        return s;
      });
    },
    undo: (): UndoEntry | null => {
      let result: UndoEntry | null = null;
      update((s) => {
        result = s.undo();
        return s;
      });
      return result;
    },
    redo: (): UndoEntry | null => {
      let result: UndoEntry | null = null;
      update((s) => {
        result = s.redo();
        return s;
      });
      return result;
    },
    clear: () => {
      update((s) => {
        s.clear();
        return s;
      });
    },
    canUndo: derived(
      writable(stack),
      ($stack) => $stack.canUndo()
    ),
    canRedo: derived(
      writable(stack),
      ($stack) => $stack.canRedo()
    ),
  };
};

export const undoStore = createUndoStore();
export const undoRestoreData = writable<UndoEntry | null>(null);
