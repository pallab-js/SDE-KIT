import {
  colors,
  spacing,
  radius,
  typography,
  surfaces,
  text,
  componentTokens,
} from './tokens';

/**
 * CSS variable name mappings for Tailwind v4 @theme integration.
 * These should be kept in sync with app.css @theme values.
 */
export const cssVar = {
  // Surfaces
  surfaceBase: '--surface-base',
  surfaceElevated: '--surface-elevated',
  surfaceSoft: '--surface-soft',
  surfaceBorder: '--surface-border',

  // Text
  textPrimary: '--text-primary',
  textSecondary: '--text-secondary',
  textMuted: '--text-muted',

  // Brand
  colorPrimary: '--color-primary',
  colorPrimaryActive: '--color-primary-active',
  colorSuccess: '--color-success',
  colorWarning: '--color-warning',
  colorError: '--color-error',

  // Typography
  fontDisplay: '--font-display',
  fontSans: '--font-sans',
  fontMono: '--font-mono',

  // Spacing
  spacingSection: '--spacing-section',
  spacingCard: '--spacing-card',

  // Radius
  radiusButton: '--radius-button',
  radiusCard: '--radius-card',
  radiusHero: '--radius-hero',
} as const;

/**
 * Generates CSS custom property declarations from design tokens
 * for a given theme mode.
 */
export function generateCSSVariables(
  theme: 'light' | 'dark' = 'dark'
): Record<string, string> {
  const s = theme === 'dark' ? surfaces.dark : surfaces.light;
  const t = theme === 'dark' ? text.dark : text.light;

  return {
    [cssVar.surfaceBase]: s.base,
    [cssVar.surfaceElevated]: s.elevated,
    [cssVar.surfaceSoft]: s.soft,
    [cssVar.surfaceBorder]: s.border,

    [cssVar.textPrimary]: t.primary,
    [cssVar.textSecondary]: t.secondary,
    [cssVar.textMuted]: t.muted,

    [cssVar.colorPrimary]: colors.primary,
    [cssVar.colorPrimaryActive]: colors.primaryActive,
    [cssVar.colorSuccess]: colors.success,
    [cssVar.colorWarning]: colors.warning,
    [cssVar.colorError]: colors.error,

    [cssVar.fontDisplay]: typography.fontDisplay,
    [cssVar.fontSans]: typography.fontSans,
    [cssVar.fontMono]: typography.fontMono,

    [cssVar.spacingSection]: spacing[24],
    [cssVar.spacingCard]: spacing[8],

    [cssVar.radiusButton]: radius.md,
    [cssVar.radiusCard]: radius.lg,
    [cssVar.radiusHero]: radius.xl,
  };
}

/**
 * Applies theme CSS variables to document.documentElement
 * and sets data-theme attribute.
 */
export function applyTheme(theme: 'light' | 'dark'): void {
  const root = document.documentElement;
  root.setAttribute('data-theme', theme);
  const vars = generateCSSVariables(theme);
  for (const [prop, value] of Object.entries(vars)) {
    root.style.setProperty(prop, value);
  }
}
