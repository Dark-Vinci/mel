import { configureStore } from '@reduxjs/toolkit';
import {channelStore} from './Channel';

export const store = configureStore({
    reducer: {
        channel: channelStore.channelReducer,
    },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
