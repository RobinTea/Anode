import { invoke } from "@tauri-apps/api/core";

export interface BookSummary {
  id: string;
  title: string;
  author: string;
  updated_at: string;
  write_page_count: number;
  total_words: number;
}

export interface BookMeta {
  id: string;
  title: string;
  author: string;
  genre: string;
  synopsis: string;
  created_at: string;
  updated_at: string;
}

export interface PageMeta {
  id: string;
  kind: "plan" | "write" | "read";
  class: string;
  title: string;
  sort_key: number;
  status: string;
  word_count: number;
  updated_at: string;
}

export interface PageBody {
  format: string;
  format_version: number;
  doc: Record<string, unknown>;
  plain_text_cache: string;
  word_count: number;
}

export interface CompileOrderEntry {
  page_id: string;
  position: number;
  included: boolean;
  title: string;
}

export interface SnapshotInfo {
  filename: string;
  timestamp: string;
  size_bytes: number;
}

export const api = {
  isFirstRun: () => invoke<boolean>("is_first_run_cmd"),
  getConfig: () => invoke<{ library_path: string | null; first_run_complete: boolean; theme: string }>("get_config_cmd"),
  initLibrary: (path: string) => invoke<void>("init_library_cmd", { path }),
  defaultLibraryPath: () => invoke<string>("get_default_library_path"),
  listBooks: () => invoke<BookSummary[]>("list_books"),
  createBook: (title: string) => invoke<BookMeta>("create_book", { title }),
  deleteBook: (bookId: string) => invoke<void>("delete_book", { bookId }),
  getBookMeta: (bookId: string) => invoke<BookMeta>("get_book_meta", { bookId }),
  listPages: (bookId: string) => invoke<PageMeta[]>("list_pages", { bookId }),
  createPage: (bookId: string, kind: string, className: string, title: string) =>
    invoke<PageMeta>("create_page", { bookId, kind, class: className, title }),
  loadPageBody: (bookId: string, pageId: string) =>
    invoke<PageBody>("load_page_body", { bookId, pageId }),
  savePageBody: (bookId: string, pageId: string, doc: Record<string, unknown>, plainText: string) =>
    invoke<PageBody>("save_page_body", { bookId, pageId, doc, plainText }),
  getCompileOrder: (bookId: string) => invoke<CompileOrderEntry[]>("get_compile_order", { bookId }),
  setCompileOrder: (bookId: string, entries: CompileOrderEntry[]) =>
    invoke<void>("set_compile_order", { bookId, entries }),
  listSnapshots: (bookId: string, pageId: string) =>
    invoke<SnapshotInfo[]>("list_snapshots", { bookId, pageId }),
  restoreSnapshot: (bookId: string, pageId: string, filename: string) =>
    invoke<PageBody>("restore_snapshot", { bookId, pageId, filename }),
  exportBook: (bookId: string, includeSnapshots: boolean, outputPath: string) =>
    invoke<void>("export_book_cmd", { bookId, includeSnapshots, outputPath }),
  importBook: (anodePath: string) =>
    invoke<string>("import_book_cmd", { anodePath }),
  exportDocx: (bookId: string, outputPath: string) =>
    invoke<void>("export_docx_cmd", { bookId, outputPath }),
};
