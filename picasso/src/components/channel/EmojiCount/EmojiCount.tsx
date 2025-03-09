import {JSX} from "react";

interface emojiCount {
    count: number;
    value: string;
}

export function EmojiCount({count, value}: emojiCount): JSX.Element {
    return (
        <div>
            {value}
            {count}
        </div>
    )
}