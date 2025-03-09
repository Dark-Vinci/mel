import { JSX } from 'react';

import { If, IfElse } from '@/components/conditionals';
import { generateRandomColor } from '@tools';
import style from './style.module.scss';

interface links {
  name: string;
  link: string;
}

interface fullUserProfileProps {
  profileUrl?: string;
  userName: string;
  links?: links[];
}

export function FullUserProfile({
  profileUrl,
  userName,
  links,
}: fullUserProfileProps): JSX.Element {
  return (
    // @ts-ignore
    <div style={style.container}>
      <div>
        <div>
          <div>
            <div>
              <p>Profile</p>
              <div>*</div>
            </div>
          </div>

          <div>
            <IfElse
              condition={!!profileUrl}
              ifElement={<img src={profileUrl} alt={'prfile image'} />}
              elseElement={
                <div
                  style={{
                    backgroundColor: generateRandomColor(),
                  }}
                >
                  {userName[0]}
                </div>
              }
            />
          </div>

          <div>
            <div>full name</div>
            <div>description</div>
            <div> status </div>
            <div>Local time</div>

            <div>
              <div>message</div>
              <div>view file</div>
              <div>vip</div>
              <div>menu</div>
            </div>
          </div>
        </div>

        <If
          condition={!!links && links.length > 0}
          element={
            <div>
              <p>About me</p>

              <div>
                {links!.map(({ name, link }) => {
                  return (
                    <div key={name}>
                      <p>{name}</p>
                      <a href={link}>{name}</a>
                    </div>
                  );
                })}
              </div>
            </div>
          }
        />
      </div>
    </div>
  );
}
