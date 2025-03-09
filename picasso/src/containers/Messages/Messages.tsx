import { JSX, useState } from 'react';

import { IfElse, JoinChannel, Message, messageProps } from '@components';
import { MessageInput } from '@/containers/MessageInput';

interface props {
  isMember: boolean;
  messages: messageProps[];
}

export function Messages({ isMember, messages }: props): JSX.Element {
  const [currentDate, _setCurrentDate] = useState<Date>(new Date());

  return (
    <div>
      <div>
        {/* stikey */}
        {/* should be updated */}
        <div>
          <p>{currentDate.toString()}</p>
        </div>

        <div>
          {messages.map((message) => {
            return (
              <div key={message.id}>
                <Message {...message} />
              </div>
            );
          })}
        </div>

        <div>
          <IfElse
            condition={isMember}
            ifElement={<MessageInput />}
            elseElement={
              <JoinChannel
                channelName={'channel 125'}
                description={'broadcast'}
              />
            }
          />
        </div>
      </div>
    </div>
  );
}
