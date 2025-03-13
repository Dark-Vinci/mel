import { JSX } from 'react'

import { useTheme } from '@/tools/hooks/theme';

export function ToggleTheme(): JSX.Element {
    const { theme, toggleTheme } = useTheme();

    return (
        <button className="button" onClick={toggleTheme}>
          Switch to {theme === 'light' ? 'dark' : 'light'} theme
        </button>
      );
}
