/**
 * Claude-style Design Tokens
 * Reference: DESIGN.md specification
 * All values must match app.css @theme variables
 */

export const colors = {
  // Brand & Accent
  primary: '#cc785c',
  primaryActive: '#a9583e',
  primaryDisabled: '#e6dfd8',
  accentTeal: '#5db8a6',
  accentAmber: '#e8a55a',

  // Surfaces - Cream (Light)
  canvas: '#faf9f5',
  surfaceSoft: '#f5f0e8',
  surfaceCard: '#efe9de',
  surfaceCreamStrong: '#e8e0d2',

  // Surfaces - Dark
  surfaceDark: '#181715',
  surfaceDarkElevated: '#252320',
  surfaceDarkSoft: '#1f1e1b',
  surfaceDarkBorder: '#302d2b',

  // Text
  ink: '#141413',
  bodyStrong: '#252523',
  body: '#3d3d3a',
  muted: '#6c6a64',
  mutedSoft: '#8e8b82',
  onPrimary: '#ffffff',
  onDark: '#faf9f5',
  onDarkSoft: '#a09d96',

  // Borders
  hairline: '#e6dfd8',
  hairlineSoft: '#ebe6df',

  // Semantic
  success: '#5db872',
  warning: '#d4a017',
  error: '#c64545',
} as const;

export const surfaces = {
  dark: {
    base: colors.surfaceDark,
    soft: colors.surfaceDarkSoft,
    elevated: colors.surfaceDarkElevated,
    border: colors.surfaceDarkBorder,
  },
  light: {
    base: colors.canvas,
    soft: colors.surfaceSoft,
    elevated: colors.surfaceCard,
    border: colors.hairline,
  },
} as const;

export const text = {
  dark: {
    primary: colors.onDark,
    secondary: colors.onDarkSoft,
    muted: colors.muted,
  },
  light: {
    primary: colors.ink,
    secondary: colors.body,
    muted: colors.muted,
  },
} as const;

export const typography = {
  fontDisplay:
    "'Cormorant Garamond', 'EB Garamond', 'Tiempos Headline', Georgia, 'Times New Roman', serif",
  fontSans:
    "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
  fontMono: "'JetBrains Mono', 'Fira Code', 'ui-monospace', monospace",

  displayXl: {
    fontSize: 28,
    fontWeight: 400,
    lineHeight: 1.2,
    letterSpacing: -0.3,
  },
  displayLg: {
    fontSize: 24,
    fontWeight: 400,
    lineHeight: 1.25,
    letterSpacing: -0.2,
  },
  displayMd: {
    fontSize: 20,
    fontWeight: 400,
    lineHeight: 1.3,
    letterSpacing: 0,
  },
  displaySm: {
    fontSize: 16,
    fontWeight: 400,
    lineHeight: 1.4,
    letterSpacing: 0,
  },
  title: { fontSize: 16, fontWeight: 500, lineHeight: 1.4 },
  body: { fontSize: 14, fontWeight: 400, lineHeight: 1.55 },
  caption: { fontSize: 12, fontWeight: 500, lineHeight: 1.4 },
  small: { fontSize: 11, fontWeight: 500, lineHeight: 1.4 },
  overline: {
    fontSize: 12,
    fontWeight: 500,
    lineHeight: 1.4,
    letterSpacing: 1.5,
    textTransform: 'uppercase' as const,
  },
  mono: {
    fontSize: 13,
    fontWeight: 400,
    lineHeight: 1.6,
    fontFamily: 'var(--font-mono)',
  },
  button: { fontSize: 14, fontWeight: 500, lineHeight: 1 },
} as const;

export const spacing = {
  1: '4px',
  2: '8px',
  3: '12px',
  4: '16px',
  5: '20px',
  6: '24px',
  7: '28px',
  8: '32px',
  10: '40px',
  12: '48px',
  14: '56px',
  16: '64px',
  24: '96px',
} as const;

export const radius = {
  xs: '4px',
  sm: '6px',
  md: '8px',
  lg: '12px',
  xl: '16px',
  '2xl': '24px',
  pill: '9999px',
  full: '9999px',
} as const;

export const componentTokens = {
  button: { height: '40px', padding: '12px 20px', radius: radius.md },
  card: { padding: spacing[8], radius: radius.lg },
  input: { height: '36px', padding: '8px 12px', radius: radius.md },
  badge: { height: '24px', padding: '4px 12px', radius: radius.pill },
} as const;

export type ThemeMode = 'light' | 'dark';
export const defaultTheme: ThemeMode = 'dark';
