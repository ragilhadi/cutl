import { formatExpirationDate } from '../utils/clipboard';
import type { AnalyticsResponse } from '../api/types';

/**
 * Renders analytics data into the given container element.
 * Replaces any existing content in the container.
 */
export function renderAnalytics(container: HTMLElement, data: AnalyticsResponse): void {
  const { total_visits, countries, referers, daily, recent_visits } = data;

  // ── Daily chart ──────────────────────────────────────────────
  const recentDays = daily.slice(-14);
  const maxCount = Math.max(...recentDays.map(d => d.count), 1);
  const firstDate = recentDays[0]?.date.slice(5) ?? '';
  const lastDate = recentDays[recentDays.length - 1]?.date.slice(5) ?? '';

  const dailyBarsHtml = recentDays.map(d => {
    const pct = Math.max(Math.round((d.count / maxCount) * 100), d.count > 0 ? 3 : 0);
    return `<div class="flex-1 flex flex-col justify-end h-full">
        <div class="w-full rounded-sm bg-indigo-500 dark:bg-indigo-400 hover:bg-indigo-600 dark:hover:bg-indigo-300 transition-colors"
             style="height:${pct}%"
             title="${d.date}: ${d.count} visit${d.count !== 1 ? 's' : ''}"></div>
      </div>`;
  }).join('');

  const dailySection = recentDays.length > 0 ? `
    <div>
      <p class="text-xs font-semibold uppercase tracking-wide text-slate-400 dark:text-slate-500 mb-2">Daily visits</p>
      <div class="flex items-end gap-0.5 h-10">${dailyBarsHtml}</div>
      <div class="flex justify-between mt-1">
        <span class="text-[10px] text-slate-400 dark:text-slate-500">${escapeHtml(firstDate)}</span>
        <span class="text-[10px] text-slate-400 dark:text-slate-500">${escapeHtml(lastDate)}</span>
      </div>
    </div>` : '';

  // ── Countries ────────────────────────────────────────────────
  const countriesHtml = countries.length === 0
    ? '<p class="text-xs text-slate-400 dark:text-slate-500 italic">No data yet</p>'
    : countries.slice(0, 5).map(c => `
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs text-slate-600 dark:text-slate-300 truncate">${escapeHtml(c.value ?? 'Unknown')}</span>
          <span class="text-xs font-semibold tabular-nums text-slate-800 dark:text-slate-200">${c.count}</span>
        </div>`).join('');

  // ── Referrers ────────────────────────────────────────────────
  const referrersHtml = referers.length === 0
    ? '<p class="text-xs text-slate-400 dark:text-slate-500 italic">No data yet</p>'
    : referers.slice(0, 5).map(r => `
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs text-slate-600 dark:text-slate-300 truncate">${escapeHtml(r.value ?? 'Direct')}</span>
          <span class="text-xs font-semibold tabular-nums text-slate-800 dark:text-slate-200">${r.count}</span>
        </div>`).join('');

  // ── Recent visits ─────────────────────────────────────────────
  const recentHtml = recent_visits.length === 0
    ? '<p class="text-xs text-slate-400 dark:text-slate-500 italic">No visits yet</p>'
    : recent_visits.slice(0, 8).map(v => {
        const loc = [v.city, v.country].filter(Boolean).join(', ') || 'Unknown';
        const ref = v.referer ? escapeHtml(v.referer) : 'Direct';
        return `
          <div class="flex items-start justify-between gap-2 py-1.5 border-b border-slate-100 dark:border-slate-800 last:border-0">
            <div class="min-w-0">
              <p class="text-xs text-slate-500 dark:text-slate-400">${formatExpirationDate(v.visited_at)}</p>
              <p class="text-[10px] text-slate-400 dark:text-slate-500 truncate" title="${escapeAttr(ref)}">${ref}</p>
            </div>
            <span class="shrink-0 text-xs text-slate-600 dark:text-slate-400">${escapeHtml(loc)}</span>
          </div>`;
      }).join('');

  // ── Render ───────────────────────────────────────────────────
  container.innerHTML = `
    <div class="mt-3 rounded-xl bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 overflow-hidden animate-slide-down">

      <!-- Header: total visits -->
      <div class="px-4 pt-4 pb-3 flex items-end justify-between border-b border-slate-100 dark:border-slate-800">
        <div>
          <span class="text-3xl font-bold tabular-nums text-slate-900 dark:text-slate-50">${total_visits}</span>
          <span class="ml-1.5 text-sm text-slate-400 dark:text-slate-500">total visit${total_visits !== 1 ? 's' : ''}</span>
        </div>
        <div class="text-right">
          <p class="text-[10px] text-slate-400 dark:text-slate-500">Created ${formatExpirationDate(data.created_at)}</p>
          <p class="text-[10px] text-slate-400 dark:text-slate-500">Expires ${formatExpirationDate(data.expires_at)}</p>
        </div>
      </div>

      <div class="px-4 py-3 space-y-4">

        ${dailySection}

        <!-- Countries & Referrers -->
        <div class="grid grid-cols-2 gap-x-6 gap-y-1">
          <div>
            <p class="text-xs font-semibold uppercase tracking-wide text-slate-400 dark:text-slate-500 mb-2">Countries</p>
            <div class="space-y-1.5">${countriesHtml}</div>
          </div>
          <div>
            <p class="text-xs font-semibold uppercase tracking-wide text-slate-400 dark:text-slate-500 mb-2">Referrers</p>
            <div class="space-y-1.5">${referrersHtml}</div>
          </div>
        </div>

        <!-- Recent visits -->
        <div>
          <p class="text-xs font-semibold uppercase tracking-wide text-slate-400 dark:text-slate-500 mb-1">Recent visits</p>
          <div>${recentHtml}</div>
        </div>

      </div>
    </div>
  `;
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

function escapeAttr(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}
