import {JSX, ReactNode} from 'react';

interface IfElseCondition {
    condition: boolean;
    ifElement: ReactNode;
    elseElement: ReactNode
}

export function IfElse({condition, ifElement, elseElement}: IfElseCondition): JSX.Element {
    if (condition) {
        return ifElement as JSX.Element;
    }

    return elseElement as JSX.Element;
}
