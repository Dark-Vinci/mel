interface typingInterface {
  userId: string;
  channelId: string;
}
//
// enum MessageType {
//     TYPING = 'TYPING',
//     CHANNEL_MESSAGE = 'CHANNEL_MESSAGE',
//     PRIVATE_MESSAGE = 'PRIVATE_MESSAGE',
// }
//
// interface ServerMessage<T extends any>{
//     type: MessageType;
//     data?: T;
// }

// {type: typing|message|private, data?:}

export class SocketCall {
  private socket: WebSocket;

  public constructor(url: string, dispatch: any) {
    this.socket = new WebSocket(url);

    this.socket.onclose = () => {
      console.log('WebSocket disconnected');
    };

    this.socket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    if (this.socket.OPEN) {
      this.socket.onmessage = (event) => {
        this.handleSocketEvent(event, dispatch);
      };
    }
  }

  private handleSocketEvent(event: MessageEvent<any>, _dispatch: any): void {
    const data = JSON.parse(event.data);
    console.log({ data });

    switch (data.type) {
      case 'typing': {
        // dispatch(typing)
        // send to appropriate channel if redux state
        break;
      }

      case 'message': {
        // dispatch(message)
        // send to the channel redux state
        break;
      }

      case 'private': {
        // dispatch(private)
        //  send to the right private dm state.
        break;
      }

      default: {
        console.log({ message: 'message' });
      }
    }
  }

  public broadcastTyping(payload: typingInterface): boolean {
    const strPayload = JSON.stringify(payload);

    if (this.socket.OPEN) {
      this.socket.send(strPayload);
      console.log({ res: 'message sent' });
      return true;
    }

    return false;
  }
}
