import {JSX, ReactNode} from "react";

interface ifProps {
    condition: boolean;
    element: ReactNode,
}

export function If({condition, element}: ifProps): JSX.Element {
    if (!condition) {
        return null as unknown as JSX.Element;
    }

    return element as JSX.Element;
}