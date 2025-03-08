import {JSX, ReactNode} from 'react';

interface ifElseProps {
    condition: boolean;
    ifElement: ReactNode;
    elseElement: ReactNode;
}

export function IfElse({condition, elseElement, ifElement}: ifElseProps): JSX.Element {
    if (condition) {
        return ifElement as JSX.Element;
    }

    return elseElement as JSX.Element;
}