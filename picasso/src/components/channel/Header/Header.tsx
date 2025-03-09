import {JSX} from "react";
import {ChannelViewChannelMember} from "@/components/channel/ChannelViewChannelMember";

interface headerProps {
    title: string;
    topic: string;
    memberCount: string;
}

export function Header({title, topic}: headerProps): JSX.Element {
    return (
        <div>
            <div>
                <div>
                    <div><p># {title}</p></div>
                    <div><p>{topic}</p><a>EDIT</a></div>

                    <div>
                        <div>
                            <ChannelViewChannelMember
                                count={0}
                                profiles={[]}
                            />
                        </div>

                        <div>ACTION</div>
                    </div>
                </div>
                <div>down</div>
            </div>
        </div>
    )
}
