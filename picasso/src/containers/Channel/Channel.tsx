import { JSX } from 'react';
import { Header } from '@components';

export function Channel(): JSX.Element {
  return (
    <div>
      <div>
        <div>
          <Header title={''} topic={''} memberCount={''} />
        </div>

        <div>MAIN</div>
        <div>INPUT/JOIN</div>
      </div>
    </div>
  );
}
