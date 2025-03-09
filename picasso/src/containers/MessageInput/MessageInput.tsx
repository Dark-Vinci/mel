import {JSX, useRef, useState} from "react";
import {marked} from "marked";

import {ActiveEmoji, Emoji, IfElse, Plus, Send} from "@components";
// import {useDispatch} from "react-redux";
// import {AppDispatch} from "@/store";

const buttonStyle = {
    background: 'none',
    border: 'none',
    padding: '5px',
    cursor: 'pointer',
    fontSize: '14px',
    color: '#333',
};

export function MessageInput(): JSX.Element {
    // const dispatch = useDispatch<AppDispatch>();
    // // dispatch(remove)
    const [message, setMessage] = useState('');
    const [showFormatting, setShowFormatting] = useState(true);
    const [emojiActive, setEmojiActive] = useState(false);

    const textAreaRef = useRef<HTMLTextAreaElement>(null);

    const formats: Record<string, Record<string, string>> = {
        bold: {type: 'text', prefix: '*', suffix: '*', template: '*text*'},
        italics: {type: 'text', prefix: '_', suffix: '_', template: '_text_'},
        strikethrough: { type: 'text', prefix: '~', suffix: '~', template: '~text~' },
        inlineCode: { type: 'text', prefix: '`', suffix: '`', template: '`text`' },
        link: { type: 'text', template: '[text](url)' },
        numberedList: { type: 'line', prefix: '1. ' },
        bulletedList: { type: 'line', prefix: '- ' },
        blockquote: { type: 'line', prefix: '> ' },
        codeBlock: { type: 'block', template: '```\ntext\n```' },
    };

    const applyFormat = (formatType: string) => {
        const format = formats[formatType];

        if (!format) {
            return
        }

        const textArea = textAreaRef.current;
        const start = textArea!.selectionStart;
        const end = textArea!.selectionEnd;
        const selectedText = message.slice(start, end);

        let newText;
        let newStart;
        let newEnd;

        switch (format.type) {
            case 'text': {
                if (selectedText) {
                    newText = message.slice(0, start) + format.prefix + selectedText + format.suffix + message.slice(end);
                    newStart = start + format.prefix.length;
                    newEnd = end + format.prefix.length;
                } else {
                    const template = format.template || format.prefix + 'text' + format.suffix;
                    newText = message.slice(0, start) + template + message.slice(end);
                    const placeholderStart = template.indexOf('text');
                    newStart = start + placeholderStart;
                    newEnd = newStart + 4;
                }

                break;
            }

            case 'line': {
                if (selectedText) {
                    const lines = selectedText.split('\n');
                    const prefixedLines = lines.map((line) => format.prefix + line);
                    const newSelectedText = prefixedLines.join('\n');
                    newText = message.slice(0, start) + newSelectedText + message.slice(end);
                    newStart = start;
                    newEnd = start + newSelectedText.length;
                } else {
                    newText = message.slice(0, start) + format.prefix + message.slice(end);
                    newStart = start + format.prefix.length;
                    newEnd = newStart;
                }

                break;
            }

            case 'block': {
                if (selectedText) {
                    newText = message.slice(0, start) + '```\n' + selectedText + '\n```' + message.slice(end);
                    newStart = start + 4;
                    newEnd = end + 4;
                } else {
                    newText = message.slice(0, start) + format.template + message.slice(end);
                    newStart = start + 4;
                    newEnd = newStart + 4;
                }

                break;
            }

            default: {
                console.log("SHOULD NOT GET HERE")
            }
        }

        setMessage(newText!);
        setTimeout(() => {
            textArea!.selectionStart = newStart!;
            textArea!.selectionEnd = newEnd!;
            textArea!.focus();
        }, 0);
    };

    return (
        <div>
            <div>
                <div>
                    <button onClick={() => applyFormat('bold')} style={buttonStyle}>
                        B
                    </button>
                    <button onClick={() => applyFormat('italic')} style={buttonStyle}>
                        I
                    </button>
                    <button onClick={() => applyFormat('strikethrough')} style={buttonStyle}>
                        S̶
                    </button>
                    <button onClick={() => applyFormat('link')} style={buttonStyle}>
                        🔗
                    </button>
                    <button onClick={() => applyFormat('numberedList')} style={buttonStyle}>
                        1.
                    </button>
                    <button onClick={() => applyFormat('bulletedList')} style={buttonStyle}>
                        •
                    </button>
                    <button onClick={() => applyFormat('blockquote')} style={buttonStyle}>
                        ”
                    </button>
                    <button onClick={() => applyFormat('codeBlock')} style={buttonStyle}>
                        &lt;/&gt;
                    </button>
                    <button onClick={() => applyFormat('inlineCode')} style={buttonStyle}>
                        `{}`
                    </button>
                </div>

                <div>
                    <textarea
                        ref={textAreaRef}
                        value={message}
                        onChange={(e) => setMessage(e.target.value)}
                        placeholder="Message #general"
                        rows={4}
                        dangerouslySetInnerHTML={{ __html: marked(message) }}
                        style={{
                            width: '100%',
                            padding: '10px',
                            border: '1px solid #ddd',
                            borderRadius: '0 0 4px 4px',
                            resize: 'vertical',
                            fontFamily: 'inherit',
                        }}
                    />
                </div>

                <div>
                    <div>
                        <div>
                            <div onClick={() => console.log({me:'mmm'})}>
                                <Plus />
                            </div>

                            <div
                                onClick={() => setShowFormatting(!showFormatting)}
                            >
                                Aa
                            </div>

                            <div
                                onMouseEnter={() => setEmojiActive(true)}
                                onMouseLeave={() => setEmojiActive(false)}
                            >
                                <IfElse
                                    condition={emojiActive}
                                    ifElement={<ActiveEmoji />}
                                    elseElement={<Emoji />}
                                />
                            </div>

                            <div>@</div>
                        </div>

                        <div>RECORD</div>
                        <div>SHORT CUT</div>
                    </div>

                    <div>
                        <div>
                            <Send
                                shouldActivate={message.length > 0}
                            />
                        </div>

                        <div>SCHEDULE</div>
                    </div>
                </div>
            </div>
        </div>
    )
}

