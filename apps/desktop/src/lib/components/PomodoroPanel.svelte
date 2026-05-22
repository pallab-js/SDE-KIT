<script lang="ts">
  import { pomodoro } from '$lib/stores/pomodoro';

  let s = $state({
    status: 'idle' as string,
    timeLeft: 1500,
    sessionsCompleted: 0,
  });
  $effect(() =>
    pomodoro.subscribe((v) => {
      s = v;
    })
  );
</script>

<div class="pomodoro">
  <div class="timer">
    <span class="time typo-display-xl">{pomodoro.formatTime(s.timeLeft)}</span>
    <span class="status typo-overline"
      >{s.status === 'idle'
        ? 'Ready'
        : s.status === 'focus'
          ? 'Focus'
          : s.status === 'break'
            ? 'Break'
            : 'Paused'}</span
    >
  </div>

  <div class="controls">
    {#if s.status === 'idle' || s.status === 'paused'}
      <button class="control-btn typo-caption" onclick={() => pomodoro.start()}
        >Start</button
      >
    {:else if s.status === 'focus' || s.status === 'break'}
      <button class="control-btn typo-caption" onclick={() => pomodoro.pause()}
        >Pause</button
      >
      <button
        class="control-btn secondary typo-caption"
        onclick={() => pomodoro.stop()}>Stop</button
      >
    {/if}
  </div>

  {#if s.sessionsCompleted > 0}
    <span class="session-count typo-small"
      >{s.sessionsCompleted} session{s.sessionsCompleted > 1 ? 's' : ''} today</span
    >
  {/if}

  <div class="settings">
    <label class="typo-small">
      Focus
      <select
        class="typo-caption"
        value={Math.floor(s.timeLeft / 60)}
        onchange={(e) =>
          pomodoro.setFocusDuration(
            Number((e.target as HTMLSelectElement).value) * 60
          )}
      >
        <option value="15">15m</option>
        <option value="25">25m</option>
        <option value="30">30m</option>
        <option value="45">45m</option>
        <option value="60">60m</option>
      </select>
    </label>
    <label class="typo-small">
      Break
      <select
        class="typo-caption"
        value={5}
        onchange={(e) =>
          pomodoro.setBreakDuration(
            Number((e.target as HTMLSelectElement).value) * 60
          )}
      >
        <option value="5">5m</option>
        <option value="10">10m</option>
        <option value="15">15m</option>
      </select>
    </label>
  </div>
</div>

<style>
  .pomodoro {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-3);
    padding: var(--spacing-4);
    text-align: center;
  }

  .timer {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
  }

  .time {
    color: var(--color-on-dark);
    font-variant-numeric: tabular-nums;
  }

  .status {
    color: var(--color-muted);
  }

  .controls {
    display: flex;
    gap: var(--spacing-2);
  }

  .control-btn {
    padding: 6px 20px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-primary);
    color: var(--color-on-primary);
    border-radius: var(--radius-sm);
    cursor: pointer;
  }

  .control-btn.secondary {
    background: var(--color-surface-dark-elevated);
    color: var(--color-on-dark-soft);
  }

  .control-btn:hover {
    opacity: 0.9;
  }

  .control-btn.secondary:hover {
    background: var(--color-surface-dark-soft);
  }

  .session-count {
    color: var(--color-muted);
  }

  .settings {
    display: flex;
    gap: var(--spacing-3);
    margin-top: var(--spacing-2);
  }

  .settings label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    color: var(--color-muted);
  }

  .settings select {
    padding: 3px 6px;
    border: 1px solid var(--color-surface-dark-border);
    background: var(--color-surface-dark-soft);
    color: var(--color-on-dark-soft);
    border-radius: var(--radius-xs);
  }
</style>
