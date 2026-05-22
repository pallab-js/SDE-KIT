import { writable, get } from 'svelte/store';
import { notify } from './notifications';

export type PomodoroStatus = 'idle' | 'focus' | 'break' | 'paused';

interface PomodoroState {
  status: PomodoroStatus;
  timeLeft: number;
  focusDuration: number;
  breakDuration: number;
  sessionsCompleted: number;
}

function createPomodoro() {
  const FOCUS_DEFAULT = 25 * 60;
  const BREAK_DEFAULT = 5 * 60;

  const store = writable<PomodoroState>({
    status: 'idle',
    timeLeft: FOCUS_DEFAULT,
    focusDuration: FOCUS_DEFAULT,
    breakDuration: BREAK_DEFAULT,
    sessionsCompleted: 0,
  });

  let timer: ReturnType<typeof setInterval> | null = null;

  function tick() {
    store.update((s) => {
      if (s.timeLeft <= 1) {
        if (s.status === 'focus') {
          notify('Focus session complete! Time for a break.', 'info');
          return {
            ...s,
            timeLeft: s.breakDuration,
            status: 'break',
            sessionsCompleted: s.sessionsCompleted + 1,
          };
        } else {
          notify('Break over! Ready to focus?', 'info');
          return { ...s, timeLeft: s.focusDuration, status: 'idle' };
        }
      }
      return { ...s, timeLeft: s.timeLeft - 1 };
    });
  }

  function start() {
    const s = get(store);
    if (s.status === 'idle' || s.status === 'break') {
      const nextStatus: PomodoroStatus =
        s.status === 'break' ? 'break' : 'focus';
      store.update((v) => ({ ...v, status: nextStatus }));
      timer = setInterval(tick, 1000);
    } else if (s.status === 'paused') {
      timer = setInterval(tick, 1000);
      store.update((v) => ({ ...v, status: 'focus' }));
    }
  }

  function pause() {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
    store.update((v) => ({ ...v, status: 'paused' }));
  }

  function stop() {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
    store.update((s) => ({
      ...s,
      status: 'idle',
      timeLeft: s.focusDuration,
    }));
  }

  function setFocusDuration(seconds: number) {
    store.update((s) => ({ ...s, focusDuration: seconds, timeLeft: seconds }));
  }

  function setBreakDuration(seconds: number) {
    store.update((s) => ({ ...s, breakDuration: seconds }));
  }

  function formatTime(seconds: number): string {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }

  return {
    subscribe: store.subscribe,
    start,
    pause,
    stop,
    setFocusDuration,
    setBreakDuration,
    formatTime,
  };
}

export const pomodoro = createPomodoro();
