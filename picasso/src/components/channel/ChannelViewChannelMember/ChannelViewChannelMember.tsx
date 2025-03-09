import { JSX } from 'react';

import { MiniProfilePhoto, miniProfilePhotoProps } from '@components';

interface props {
  count: number;
  profiles: miniProfilePhotoProps[];
}

export function ChannelViewChannelMember({
  count,
  profiles,
}: props): JSX.Element {
  return (
    <div>
      <div>
        <MiniProfilePhoto
          userName={profiles[0].userName}
          url={profiles[0].url}
        />

        <MiniProfilePhoto
          userName={profiles[1].userName}
          url={profiles[1].url}
        />

        <MiniProfilePhoto
          userName={profiles[2].userName}
          url={profiles[2].url}
        />
      </div>

      <div>{count}</div>
    </div>
  );
}
