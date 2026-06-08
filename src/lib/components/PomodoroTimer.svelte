<script lang="ts">
  import { onDestroy } from "svelte";

  let timeLeft = $state(25 * 60); // 25 minutes in seconds
  let isRunning = $state(false);
  let mode = $state<"work" | "break">("work");
  let timer: ReturnType<typeof setInterval> | undefined;

  function toggle() {
    if (isRunning) {
      clearInterval(timer);
      isRunning = false;
    } else {
      isRunning = true;
      timer = setInterval(() => {
        if (timeLeft > 0) {
          timeLeft -= 1;
        } else {
          switchMode();
        }
      }, 1000);
    }
  }

  function switchMode() {
    if (mode === "work") {
      mode = "break";
      timeLeft = 5 * 60;
      new Notification("Anode Pomodoro", { body: "Time for a break!" });
    } else {
      mode = "work";
      timeLeft = 25 * 60;
      new Notification("Anode Pomodoro", { body: "Back to work!" });
    }
  }

  function reset() {
    clearInterval(timer);
    isRunning = false;
    timeLeft = mode === "work" ? 25 * 60 : 5 * 60;
  }

  const formattedTime = $derived(() => {
    const mins = Math.floor(timeLeft / 60);
    const secs = timeLeft % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  });

  onDestroy(() => {
    if (timer) clearInterval(timer);
  });
</script>

<div class="pomodoro" class:isRunning class:isBreak={mode === "break"}>
  <button class="timer-display" onclick={toggle} title="Toggle Timer">
    <span class="icon">{mode === "work" ? "🍅" : "☕"}</span>
    <span class="time">{formattedTime()}</span>
  </button>
  <button class="reset-btn" onclick={reset} title="Reset">↺</button>
</div>

<style>
  .pomodoro {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.5rem;
    border-radius: 4px;
    background: var(--bg);
    border: 1px solid var(--border);
    font-size: 0.85rem;
    height: 24px;
  }

  .isRunning {
    border-color: var(--accent);
  }

  .isBreak {
    background: #dcfce7; /* Light green */
  }

  .timer-display {
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    cursor: pointer;
    padding: 0;
    color: var(--text);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .icon {
    font-size: 0.9rem;
  }

  .reset-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    font-size: 1rem;
    padding: 0;
    line-height: 1;
  }

  .reset-btn:hover {
    color: var(--text);
  }
</style>
