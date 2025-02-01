import {JSX, ReactNode} from 'react';

interface IfCondition {
    condition: boolean;
    element: ReactNode
}

export function If({condition, element}: IfCondition): JSX.Element {
    if (!condition) {
        return null as unknown as JSX.Element;
    }

    return element as JSX.Element;
}
