<script lang="ts">
  import { onMount } from "svelte";
  import { api, type DailyQuest } from "$lib/api";

  let history = $state<DailyQuest[]>([]);
  let isLoading = $state(true);

  onMount(async () => {
    history = await api.getHistoryQuests(90); // Last 90 days
    isLoading = false;
  });

  function getContributionLevel(count: number, goal: number): number {
    if (count === 0) return 0;
    const ratio = count / goal;
    if (ratio >= 1) return 4;
    if (ratio >= 0.75) return 3;
    if (ratio >= 0.5) return 2;
    return 1;
  }
</script>

<div class="streak-heatmap">
  <h3>Writing Activity</h3>
  {#if isLoading}
    <div class="skeleton"></div>
  {:else}
    <div class="heatmap-grid">
      {#each history as day}
        <div 
          class="heatmap-cell level-{getContributionLevel(day.word_count, day.goal)}"
          title="{day.date}: {day.word_count} words"
        ></div>
      {/each}
    </div>
    <div class="heatmap-legend">
      <span>Less</span>
      <div class="heatmap-cell level-0"></div>
      <div class="heatmap-cell level-1"></div>
      <div class="heatmap-cell level-2"></div>
      <div class="heatmap-cell level-3"></div>
      <div class="heatmap-cell level-4"></div>
      <span>More</span>
    </div>
  {/if}
</div>

<style>
  .streak-heatmap {
    background: var(--bg-elevated);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--border);
  }

  h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.85rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .heatmap-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 3px;
  }

  .heatmap-cell {
    width: 10px;
    height: 10px;
    border-radius: 2px;
    background: var(--bg);
  }

  .level-0 { background: var(--bg); border: 1px solid var(--border); }
  .level-1 { background: #dcfce7; }
  .level-2 { background: #86efac; }
  .level-3 { background: #22c55e; }
  .level-4 { background: #15803d; }

  .heatmap-legend {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-top: 0.75rem;
    font-size: 0.7rem;
    color: var(--text-muted);
  }

  .skeleton {
    height: 50px;
    background: var(--bg);
    border-radius: 4px;
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0% { opacity: 0.5; }
    50% { opacity: 0.8; }
    100% { opacity: 0.5; }
  }
</style>
