import { JSX } from 'react';

interface props {
  channelName: string;
  description: string;
}

export function JoinChannel({ channelName, description }: props): JSX.Element {
  return (
    <div>
      <div>
        <div># {channelName}</div>
        <div>
          <p>{description}</p>
        </div>

        <div>
          <button>Details</button>
          <button>Join</button>
        </div>

        <div>
          <a>Back to all channels</a>
        </div>
      </div>
    </div>
  );
}
