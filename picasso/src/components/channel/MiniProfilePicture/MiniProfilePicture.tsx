import { JSX } from 'react';
import { IfElse } from '@/components/conditionals';

export interface miniProfilePhotoProps {
  url?: string;
  userName: string;
}

export function MiniProfilePhoto({
  url,
  userName,
}: miniProfilePhotoProps): JSX.Element {
  return (
    <div>
      <IfElse
        condition={!!url}
        ifElement={<img src={url} alt={'profile'} />}
        elseElement={
          <div style={{ background: '#000' }}>
            <p>{userName[0]}</p>
          </div>
        }
      />
    </div>
  );
}
