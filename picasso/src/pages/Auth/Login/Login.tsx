import { JSX } from 'react';

import style from './style.module.scss'
import {Emoji} from "@components";

export function Login(): JSX.Element {
    return (
        <div className={style.container}>
            <div>
                <div>
                    <input placeholder="email" type="text"/>
                    <input placeholder={'password'} type="password"/>
                </div>

                <button>LOGIN <Emoji /></button>
            </div>

            <a href={'.'}> Sign Up </a>
        </div>
    )
}