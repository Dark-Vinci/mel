import {JSX, useRef, useState} from "react";

export function MessageInput(): JSX.Element {
    const [message, setMessage] = useState('');

    const textAreaRef = useRef(null);
    return (
        <div>
            <div>
                <div>top</div>
                <div>input</div>
                <div>bottom</div>
            </div>
        </div>
    )
}
