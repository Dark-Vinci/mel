import {createSlice, PayloadAction} from "@reduxjs/toolkit";

import {Channel, ChannelState} from "@/store/Channel/type";
import {ChannelReducer} from "@/store/Channel/reducer";


const initialState: ChannelState = {
    posts: [],
};

let c = new ChannelReducer();

const channelSlice = createSlice({
    name: 'channel',
    initialState,
    reducers: {
        listChannel(state: ChannelState, action: PayloadAction<Channel>): void {
            c.listChannel(state, action)
        },

        removeChannel(state: ChannelState, action: PayloadAction<string>): void {
            c.removeChannel(state, action)
        }
    },
});

export const channelStore = {
    listChannel: channelSlice.actions.listChannel,
    removeChannel: channelSlice.actions.removeChannel,
    channelReducer: channelSlice.reducer,
};
