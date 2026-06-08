<script lang="ts">
  import { t } from "$lib/i18n";
  import { api } from "$lib/api";
  import type { DailyQuest } from "$lib/api";

  let daily = $state<DailyQuest | null>(null);
  let weekly = $state<DailyQuest[]>([]);
  let isLoading = $state(false);

  async function loadQuests() {
    isLoading = true;
    try {
      daily = await api.getDailyQuest();
      weekly = await api.getWeeklyQuests();
    } catch (err) {
      console.error("Failed to load quests:", err);
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    loadQuests();
  });

  function formatDate(date: string): string {
    return new Date(date).toLocaleDateString(undefined, { 
      weekday: 'short', 
      month: 'short', 
      day: 'numeric' 
    });
  }

  function getProgressPercentage(quest: DailyQuest): number {
    return Math.min((quest.word_count / quest.goal) * 100, 100);
  }

  function isCompleted(quest: DailyQuest): boolean {
    return quest.word_count >= quest.goal;
  }
</script>

<div class="quest-panel">
  <h2>{t("quests.title")}</h2>
  
  {#if isLoading}
    <p class="loading">{t("loading")}...</p>
  {:else}
    <div class="quest-today">
      <h3>{t("quests.today")}</h3>
      {#if daily}
        <div class="progress-bar">
          <div 
            class="progress-fill {isCompleted(daily) ? 'completed' : ''}"
            style="width: {getProgressPercentage(daily)}%"
          ></div>
          <span class="progress-text">
            {#if isCompleted(daily)}
              {t("quests.completed")}
            {:else}
              {t("quests.progress", { current: daily.word_count, goal: daily.goal })}
            {/if}
          </span>
        </div>
      {/if}
    </div>

    <div class="quest-weekly">
      <h3>{t("quests.weekly")}</h3>
      {#if weekly.length > 0}
        <div class="weekly-grid">
          {#each weekly as quest (quest.date)}
            <div class="weekly-day">
              <div class="day-name">{formatDate(quest.date)}</div>
              <div class="day-progress">
                <span>{quest.word_count}</span>
                <span class="goal">/ {quest.goal}</span>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .quest-panel {
    background: var(--bg);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .quest-panel h2 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: var(--text);
  }

  .quest-today h3,
  .quest-weekly h3 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  .progress-bar {
    height: 24px;
    background: var(--bg-elevated);
    border-radius: 12px;
    overflow: hidden;
    position: relative;
    margin-bottom: 0.5rem;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 12px;
    transition: width 0.3s ease;
  }

  .progress-fill.completed {
    background: #22c55e;
  }

  .progress-text {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    font-size: 0.75rem;
    color: white;
    text-shadow: 0 0 2px rgba(0,0,0,0.5);
    white-space: nowrap;
  }

  .quest-weekly {
    margin-top: 1rem;
  }

  .weekly-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.5rem;
  }

  .weekly-day {
    background: var(--bg-elevated);
    border-radius: 6px;
    padding: 0.5rem;
    text-align: center;
  }

  .day-name {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-bottom: 0.25rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .day-progress {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    color: var(--text);
  }

  .day-progress .goal {
    color: var(--text-muted);
  }

  @media (max-width: 600px) {
    .weekly-grid {
      grid-template-columns: repeat(4, 1fr);
    }
  }

  .loading {
    color: var(--text-muted);
    font-style: italic;
    font-size: 0.9rem;
  }
</style>
