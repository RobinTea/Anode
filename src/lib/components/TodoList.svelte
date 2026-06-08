<script lang="ts">
  import { t } from "$lib/i18n";
  import { api } from "$lib/api";
  import type { TodoItem } from "$lib/api";

  let todos = $state<TodoItem[]>([]);
  let newTodoText = $state("");
  let isLoading = $state(false);

  async function loadTodos() {
    isLoading = true;
    try {
      todos = await api.listTodos();
    } catch (err) {
      console.error("Failed to load todos:", err);
    } finally {
      isLoading = false;
    }
  }

  async function addTodo() {
    if (!newTodoText.trim()) return;
    try {
      const todo = await api.createTodo(newTodoText.trim());
      todos = [...todos, todo];
      newTodoText = "";
    } catch (err) {
      console.error("Failed to create todo:", err);
    }
  }

  async function toggleDone(id: string) {
    try {
      const index = todos.findIndex(t => t.id === id);
      if (index !== -1) {
        const done = await api.toggleTodoDone(id);
        todos[index] = { ...todos[index], done };
        todos = todos;
      }
    } catch (err) {
      console.error("Failed to toggle todo:", err);
    }
  }

  async function deleteTodo(id: string) {
    try {
      await api.deleteTodo(id);
      todos = todos.filter(t => t.id !== id);
    } catch (err) {
      console.error("Failed to delete todo:", err);
    }
  }

  $effect(() => {
    loadTodos();
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && newTodoText.trim()) {
      addTodo();
    }
  }
</script>

<div class="todos-panel">
  <h2>{t("todos.title")}</h2>
  
  <div class="todos-add">
    <input
      type="text"
      bind:value={newTodoText}
      placeholder={t("todos.add")}
      onkeydown={handleKeydown}
    />
    <button class="btn btn-primary btn-sm" onclick={addTodo} disabled={!newTodoText.trim()}>
      {t("todos.add")}
    </button>
  </div>

  {#if isLoading}
    <p class="loading">{t("loading")}...</p>
  {:else if todos.length === 0}
    <p class="todos-empty">{t("todos.empty")}</p>
  {:else}
    <ul class="todos-list">
      {#each todos as todo (todo.id)}
        <li class="todo-item {todo.done ? 'done' : ''}">
          <label class="todo-checkbox">
            <input
              type="checkbox"
              checked={todo.done}
              onchange={() => toggleDone(todo.id)}
            />
            <span class="todo-text">{todo.text}</span>
          </label>
          <button 
            class="btn-icon btn-delete"
            onclick={() => deleteTodo(todo.id)}
            title={t("todos.delete")}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .todos-panel {
    background: var(--bg);
    border-radius: 8px;
    padding: 1rem;
  }

  .todos-panel h2 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: var(--text);
  }

  .todos-add {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .todos-add input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-elevated);
    color: var(--text);
    font-size: 0.9rem;
  }

  .todos-add input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .btn-sm {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
  }

  .todos-empty {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 0;
  }

  .todos-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .todo-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 4px;
    transition: background 0.15s;
  }

  .todo-item:hover {
    background: var(--bg-elevated);
  }

  .todo-item.done .todo-text {
    text-decoration: line-through;
    color: var(--text-muted);
  }

  .todo-checkbox {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    flex: 1;
  }

  .todo-checkbox input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--accent);
    cursor: pointer;
  }

  .todo-text {
    color: var(--text);
    font-size: 0.9rem;
    cursor: pointer;
  }

  .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    border: none;
    background: transparent;
    cursor: pointer;
    color: var(--text-muted);
    border-radius: 4px;
    transition: color 0.15s, background 0.15s;
  }

  .btn-icon:hover {
    color: var(--text);
    background: var(--bg-elevated);
  }

  .btn-delete:hover {
    color: #dc2626;
    background: rgba(220, 38, 38, 0.1);
  }

  .loading {
    color: var(--text-muted);
    font-style: italic;
  }
</style>
