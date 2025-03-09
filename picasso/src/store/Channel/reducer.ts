import { PayloadAction } from '@reduxjs/toolkit';
import { Channel, ChannelState } from '@/store/Channel/type.ts';

export class ChannelReducer {
  public constructor() {}

  public listChannel(
    _state: ChannelState,
    _action: PayloadAction<Channel>,
  ): void {
    // do the work here
  }

  public removeChannel(
    _state: ChannelState,
    _action: PayloadAction<string>,
  ): void {
    // state.posts = state.posts.filter(post => post.id !== action.payload);
  }
}
