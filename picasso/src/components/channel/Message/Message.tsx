import {JSX, useState} from "react";
import EmojiPicker from "emoji-picker-react";

import {MiniProfilePhoto} from "@/components/channel/MiniProfilePicture";
import {EmojiCount} from "@/components/channel/EmojiCount";
import {AddEmoji, Greater} from "@/components/icons";
import {getDayDifference, THREAD_PROFILE_MAX_COUNT} from "@tools";
import {If, IfElse} from "@/components";

export interface commenterProfile {
    userName: string;
    profileUrl?: string;
}

export interface messageProps {
    id: string;
    content: string;
    userName: string;
    createdAt: Date;
    profileUrl?: string;
    childrenMessageCommenterProfile: commenterProfile[];
    reactions: Record<string, number>;
    lastReplyDate: Date;
}

export function Message(
    {
        content,
        userName,
        profileUrl,
        createdAt,
        childrenMessageCommenterProfile,
        reactions,
        lastReplyDate,
    }: messageProps
): JSX.Element {
    const [emoji, setEmoji] = useState<boolean>(false);
    const [viewThread, setViewThread] = useState<boolean>(false);

    return (
        <div>
            <div>
                <div>
                    <MiniProfilePhoto
                        userName={userName}
                        url={profileUrl}
                    />
                </div>

                <div>
                    <div>
                        <div>
                            <p>{userName}</p>
                            <p>{createdAt.getHours()}:{createdAt.getMinutes()}</p>
                        </div>

                        <div>
                            <p dangerouslySetInnerHTML={{ __html:content}}></p>
                            {/*    this should be saved in the db as html */}
                        </div>

                        <div>
                            {/* emoji list */}
                            <div>
                                {
                                    Object.keys(reactions).map((reaction) => {
                                        return (
                                            <div key={reaction}>
                                                <EmojiCount
                                                    count={reactions[reaction]}
                                                    value={reaction}
                                                />
                                            </div>
                                        )
                                    })
                                }
                            </div>

                            {/* add more emoji */}
                            <div onClick={() => setEmoji(!emoji)}>
                                <AddEmoji />

                                <If
                                    condition={emoji}
                                    element={
                                        <div>
                                            <EmojiPicker />
                                        </div>
                                    }
                                />
                            </div>
                        </div>
                    </div>

                    <div
                        onMouseEnter={() => setViewThread(true)}
                        onMouseLeave={() => setViewThread(false)}
                    >
                        {
                            childrenMessageCommenterProfile
                                .slice(0, THREAD_PROFILE_MAX_COUNT)
                                .map((mess) => {
                                    return (
                                        <div key={mess.userName}>
                                            <MiniProfilePhoto
                                                userName={mess.userName}
                                                url={mess.profileUrl}
                                            />
                                        </div>
                                    )
                            })
                        }

                        <a href={'LINK'}>{childrenMessageCommenterProfile.length} reply</a>

                        <IfElse
                            condition={viewThread}
                            ifElement={<div>View thread</div>}
                            elseElement={
                                <p>Last reply {getDayDifference(lastReplyDate, new Date())} days</p>
                            }
                        />

                        <div>
                            <Greater />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}