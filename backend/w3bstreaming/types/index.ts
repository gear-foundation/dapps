export interface IWatchMsg {
  streamId: string;
  signedMsg: string;
  encodedId: string;
}

export interface IBroadcastMsg {
  streamId: string;
}

export interface IStopBroadcastingMsg extends IBroadcastMsg {}

export interface IOfferMsg {
  userId: string;
  description: RTCSessionDescription;
  streamId: string;
}

export interface IStopWatchingMsg {
  streamId: string;
}

export interface IAnswerMsg {
  userId: string;
  streamId: string;
  description: RTCSessionDescription;
}

export interface ICandidateMsg {
  userId: string;
  streamId: string;
  candidate: RTCIceCandidate;
}

export interface IErrorResponse {
  message: string;
}

export interface GetInfoForUserMsg {
  streamId: string;
}
